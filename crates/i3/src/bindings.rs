use crate::application::bindings;
use crate::commands_parser::parse_command;
use crate::con::arena::{containers, resolve};
use crate::config::modes;
use crate::flags::KeyboardGroups;
use crate::ipc::ipc_send_binding_event;
use crate::types::BindingRelease;
use crate::types::InputType;
use i3_common::safewrappers::sasprintf;
use std::os::unix::ffi::OsStrExt;
use std::rc::Rc;
use xcb::{x, XidNew};

fn connection() -> Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
}
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * bindings.c: Functions for configuring, finding and, running bindings.
 */
// Binding policy consumes copied values; legacy mode/keycode queues retain
// malloc ownership. XKB/XCB and callbacks require the exclusive event thread;
// temporary replies/states and RMLVO strings have scoped owners.

pub use crate::application::Binding;
pub use crate::application::Binding_Keycode;

pub use crate::application::bindings_head;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct modes_head {
    pub slh_first: *mut Mode,
}
pub struct Mode {
    pub name: Vec<u8>,
    pub pango_markup: bool,
    bindings: OwnedBindingsHead,
    pub modes: ModeLinks,
}
// Queue heads have self-referential tail links and escaped intrusive aliases.
// Store only the stable allocation address: moving a Box after publishing its
// pointers could invalidate those aliases under Box's uniqueness rules.
struct OwnedBindingsHead {
    pointer: std::ptr::NonNull<bindings_head>,
}

impl OwnedBindingsHead {
    fn new() -> Self {
        let pointer = Box::into_raw(Box::new(bindings_head {
            tqh_first: std::ptr::null_mut(),
            tqh_last: std::ptr::null_mut(),
        }));
        // Derive self-pointers only after Box ownership has become a raw owner.
        unsafe {
            (*pointer).tqh_last = &raw mut (*pointer).tqh_first;
        }
        Self {
            pointer: std::ptr::NonNull::new(pointer).expect("Box allocation is non-null"),
        }
    }

    fn as_ptr(&self) -> *mut bindings_head {
        self.pointer.as_ptr()
    }
}

impl Drop for OwnedBindingsHead {
    fn drop(&mut self) {
        // Mode teardown drains the queue; reorder detaches every node and
        // refreshes the active alias before the old owner is dropped.
        unsafe {
            drop(Box::from_raw(self.pointer.as_ptr()));
        }
    }
}

impl Mode {
    pub fn new(name: Vec<u8>, pango_markup: bool) -> Self {
        Self {
            name: name.split(|byte| *byte == 0).next().unwrap().to_vec(),
            pango_markup,
            bindings: OwnedBindingsHead::new(),
            modes: ModeLinks {
                sle_next: std::ptr::null_mut(),
            },
        }
    }

    /// Borrowed event-thread queue alias, valid until this mode's head is replaced or dropped.
    pub fn bindings_ptr(&self) -> *mut bindings_head {
        self.bindings.as_ptr()
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModeLinks {
    pub sle_next: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub has_errors: bool,
    pub has_warnings: bool,
    pub line_number: i32,
    pub line_copy: *mut ::core::ffi::c_char,
    pub filename: *const ::core::ffi::c_char,
    pub compact_error: *mut ::core::ffi::c_char,
    pub first_column: i32,
    pub last_column: i32,
}
pub use crate::application::ipc_client;
pub use crate::assignments::CommandResult;

#[derive(Copy, Clone)]
#[repr(C)]
struct Resolve {
    pub bind: *mut Binding,
    pub keysym: xkb_keysym_t,
    pub xkb_state: *mut xkb_state,
    pub xkb_state_no_shift: *mut xkb_state,
    pub xkb_state_numlock: *mut xkb_state,
    pub xkb_state_numlock_no_shift: *mut xkb_state,
}

pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong = (1 as ::core::ffi::c_ulong) << 31;
pub const I3_IPC_EVENT_MODE: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 2 as ::core::ffi::c_ulong;

static mut xkb_context: *mut xkb_context = std::ptr::null_mut::<xkb_context>();
static mut xkb_keymap: *mut xkb_keymap = std::ptr::null_mut::<xkb_keymap>();

pub static mut command_error_nagbar_pid: libc::pid_t = -(1 as libc::pid_t);

pub const DEFAULT_BINDING_MODE: &[u8] = b"default";
unsafe fn mode_from_name(name: &[u8], pango_markup: bool) -> *mut Mode {
    let name = name.split(|byte| *byte == 0).next().unwrap();
    let mut mode: *mut Mode = modes.slh_first;

    while !mode.is_null() {
        if (*mode).name == name {
            return mode;
        }
        mode = (*mode).modes.sle_next;
    }
    mode = Box::into_raw(Box::new(Mode::new(name.to_vec(), pango_markup)));
    (*mode).modes.sle_next = modes.slh_first;
    modes.slh_first = mode;
    mode
}

/// Unlink and destroy the first mode and its bindings.
///
/// # Safety
/// The event thread exclusively owns the initialized mode registry. Every linked
/// node is a unique Box allocation and no borrowed queue/name view is in use.
#[deny(unsafe_op_in_unsafe_fn)]
pub unsafe fn remove_first_mode() {
    unsafe {
        let mode = modes.slh_first;
        if mode.is_null() {
            return;
        }
        modes.slh_first = (*mode).modes.sle_next;
        if bindings == (*mode).bindings_ptr() {
            bindings = std::ptr::null_mut();
        }
        if current_binding_mode.as_deref() == Some((*mode).name.as_slice()) {
            current_binding_mode = None;
        }
        while !(*(*mode).bindings.as_ptr()).tqh_first.is_null() {
            let bind = (*(*mode).bindings.as_ptr()).tqh_first;
            if !(*bind).bindings.tqe_next.is_null() {
                (*(*bind).bindings.tqe_next).bindings.tqe_prev = (*bind).bindings.tqe_prev;
            } else {
                (*(*mode).bindings.as_ptr()).tqh_last = (*bind).bindings.tqe_prev;
            }
            *(*bind).bindings.tqe_prev = (*bind).bindings.tqe_next;
            binding_free(bind);
        }
        drop(Box::from_raw(mode));
    }
}

pub unsafe fn configure_binding(
    bindtype: *const ::core::ffi::c_char,
    modifiers: *const ::core::ffi::c_char,
    input_code: *const ::core::ffi::c_char,
    release: *const ::core::ffi::c_char,
    border: *const ::core::ffi::c_char,
    whole_window: *const ::core::ffi::c_char,
    exclude_titlebar: *const ::core::ffi::c_char,
    command: *const ::core::ffi::c_char,
    modename: &[u8],
    pango_markup: bool,
) -> *mut Binding {
    let mut new_binding: *mut Binding =
        scalloc(1_usize, ::core::mem::size_of::<Binding>() as usize) as *mut Binding;
    {
        // Snapshot native bytes before the logger can dispatch a callback.
        let log_value_0 = i3_common::logging::pointer(new_binding);
        let log_pointer_2 = bindtype;
        let log_value_1 = i3_common::logging::string_bytes(
            (if log_pointer_2.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_2))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        let log_pointer_3 = modifiers;
        let log_value_2 = i3_common::logging::string_bytes(
            (if log_pointer_3.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_3))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        let log_pointer_4 = input_code;
        let log_value_3 = i3_common::logging::string_bytes(
            (if log_pointer_4.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_4))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        let log_pointer_5 = release;
        let log_value_4 = i3_common::logging::string_bytes(
            (if log_pointer_5.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(log_pointer_5))
            })
            .map(std::ffi::CStr::to_bytes),
            None,
        )
        .to_vec();
        if let (
            Ok(log_value_1_text),
            Ok(log_value_2_text),
            Ok(log_value_3_text),
            Ok(log_value_4_text),
        ) = (
            std::str::from_utf8(&log_value_1),
            std::str::from_utf8(&log_value_2),
            std::str::from_utf8(&log_value_3),
            std::str::from_utf8(&log_value_4),
        ) {
            ::log::debug!("../src/bindings.c:configure_binding:64 - Binding {} bindtype {}, modifiers {}, input code {}, release {}\n", log_value_0, log_value_1_text, log_value_2_text, log_value_3_text, log_value_4_text);
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                format!(
                    "../src/bindings.c:configure_binding:64 - Binding {} bindtype ",
                    log_value_0
                )
                .as_bytes(),
            );
            message.extend_from_slice(&log_value_1);
            message.extend_from_slice(", modifiers ".as_bytes());
            message.extend_from_slice(&log_value_2);
            message.extend_from_slice(", input code ".as_bytes());
            message.extend_from_slice(&log_value_3);
            message.extend_from_slice(", release ".as_bytes());
            message.extend_from_slice(&log_value_4);
            message.extend_from_slice("\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    (*new_binding).release = if !release.is_null() {
        BindingRelease::KeyRelease
    } else {
        BindingRelease::KeyPress
    };
    (*new_binding).border = !border.is_null();
    (*new_binding).whole_window = !whole_window.is_null();
    (*new_binding).exclude_titlebar = !exclude_titlebar.is_null();
    if strcmp(bindtype, c"bindsym".as_ptr()) == 0 {
        (*new_binding).input_type = if strncasecmp(
            input_code,
            c"button".as_ptr(),
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1_usize),
        ) == 0
        {
            InputType::Mouse
        } else {
            InputType::Keyboard
        };
        (*new_binding).symbol = sstrdup(input_code);
    } else {
        let Ok(keycode) = parse_long(std::ffi::CStr::from_ptr(input_code).to_bytes(), 10) else {
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_pointer_1 = input_code;
                let log_value_0 = i3_common::logging::string_bytes(
                    (if log_pointer_1.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_1))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!(
                        "ERROR: Could not parse \"{}\" as an input code, ignoring this binding.\n",
                        log_value_0_text
                    );
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice("ERROR: Could not parse \"".as_bytes());
                    message.extend_from_slice(&log_value_0);
                    message.extend_from_slice(
                        "\" as an input code, ignoring this binding.\n".as_bytes(),
                    );
                    i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                }
            };
            free(new_binding as *mut ::core::ffi::c_void);
            new_binding = ::core::ptr::null_mut::<Binding>();
            return ::core::ptr::null_mut::<Binding>();
        };
        (*new_binding).keycode = keycode as u32;
        (*new_binding).input_type = InputType::Keyboard;
    }
    (*new_binding).command = sstrdup(command);
    (*new_binding).event_state_mask = event_state_from_str(modifiers);
    let group_bits_set = (((*new_binding).event_state_mask >> 16) & 15).count_ones();
    if group_bits_set > 1 {
        ::log::error!("ERROR: Keybinding has more than one Group specified, but your X server is always in precisely one group. The keybinding can never trigger.\n");
    }
    let mode: *mut Mode = mode_from_name(modename, pango_markup);
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
    (*new_binding).bindings.tqe_prev = (*(*mode).bindings.as_ptr()).tqh_last;
    *(*(*mode).bindings.as_ptr()).tqh_last = new_binding;
    (*(*mode).bindings.as_ptr()).tqh_last = &raw mut (*new_binding).bindings.tqe_next;
    (*new_binding).keycodes_head.tqh_first = ::core::ptr::null_mut::<Binding_Keycode>();
    (*new_binding).keycodes_head.tqh_last = &raw mut (*new_binding).keycodes_head.tqh_first;
    new_binding
}
fn group_matches(mask: u32, group: u32) -> bool {
    KeyboardGroups::from_state_mask(mask).matches_group(group)
}
unsafe fn binding_in_current_group(bind: *const Binding) -> bool {
    if (*bind).event_state_mask >> 16 != 0 && xkb_current_group > 3 {
        {
            let log_value_0 = (xkb_current_group);
            ::log::error!("ERROR: BUG: xkb_current_group (= {}) outside of [XCB_XKB_GROUP_1..XCB_XKB_GROUP_4]\n", log_value_0);
        };
    }
    group_matches((*bind).event_state_mask, xkb_current_group as u32)
}
unsafe fn grab_keycode_for_binding(connection: &xcb::Connection, bind: *mut Binding, keycode: u32) {
    let mods: i32 = ((*bind).event_state_mask & 0xffff_u32) as i32;
    {
        let log_value_0 = i3_common::logging::pointer(bind);
        let log_value_1 = (keycode) as i32;
        let log_value_2 = ((*bind).event_state_mask);
        let log_value_3 = (mods) as u32;
        ::log::debug!("../src/bindings.c:grab_keycode_for_binding:141 - Binding {} Grabbing keycode {} with event state mask 0x{:x} (mods 0x{:x})\n", log_value_0, log_value_1, log_value_2, log_value_3);
    };
    for modifiers in grab_masks(mods as u32, xcb_numlock_mask) {
        connection.send_request(&x::GrabKey {
            owner_events: false,
            grab_window: x::Window::new(root),
            modifiers: x::ModMask::from_bits_retain(u32::from(modifiers)),
            key: keycode as u8,
            pointer_mode: x::GrabMode::Async,
            keyboard_mode: x::GrabMode::Async,
        });
    }
}

pub unsafe fn grab_all_keys() {
    let connection = connection();
    let mut bind: *mut Binding = (*bindings).tqh_first;

    while !bind.is_null() {
        if !((*bind).input_type != InputType::Keyboard) && binding_in_current_group(bind) {
            if (*bind).keycode > 0_u32 {
                grab_keycode_for_binding(&connection, bind, (*bind).keycode);
            } else {
                let mut binding_keycode: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;

                while !binding_keycode.is_null() {
                    let keycode: i32 = (*binding_keycode).keycode as i32;
                    let mods: i32 = ((*binding_keycode).modifiers & 0xffff_u32) as i32;
                    {
                        let log_value_0 = i3_common::logging::pointer(bind);
                        let log_value_1 = (keycode);
                        let log_value_2 = (mods);
                        ::log::debug!("../src/bindings.c:grab_all_keys:176 - Binding {} Grabbing keycode {} with mods {}\n", log_value_0, log_value_1, log_value_2);
                    };
                    connection.send_request(&x::GrabKey {
                        owner_events: false,
                        grab_window: x::Window::new(root),
                        modifiers: x::ModMask::from_bits_retain(u32::from(mods as u16)),
                        key: keycode as u8,
                        pointer_mode: x::GrabMode::Async,
                        keyboard_mode: x::GrabMode::Async,
                    });
                    binding_keycode = (*binding_keycode).keycodes.tqe_next;
                }
            }
        }
        bind = (*bind).bindings.tqe_next;
    }
}

pub unsafe fn regrab_all_buttons() {
    let connection = connection();
    let buttons = bindings_get_buttons_to_grab();
    let buttons_owner = Native::take(buttons);
    connection.send_request(&x::GrabServer {});
    let server_grab = ServerGrab(connection.clone());
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        if !(*con).window.is_null() {
            connection.send_request(&x::UngrabButton {
                button: x::ButtonIndex::Any,
                grab_window: x::Window::new((*(*con).window).id),
                modifiers: x::ModMask::ANY,
            });
            xcb_grab_buttons(conn, (*(*con).window).id, buttons);
        }
    }
    drop(buttons_owner);
    drop(server_grab);
}
// The lookup changes release state while walking the legacy queue. Only copied
// key/mask/release values enter safe policy; no graph references cross callbacks.
unsafe fn get_binding(
    state_filtered: u32,
    is_release: bool,
    input_code: u16,
    input_type: InputType,
) -> *mut Binding {
    find_binding(bindings, state_filtered, is_release, input_code, input_type)
}
unsafe fn find_binding(
    head: *mut bindings_head,
    state_filtered: u32,
    is_release: bool,
    input_code: u16,
    input_type: InputType,
) -> *mut Binding {
    let mut bind = (*head).tqh_first;
    if !is_release {
        while !bind.is_null() {
            if (*bind).input_type == input_type
                && (*bind).release == BindingRelease::KeyReleaseIgnoreMods
            {
                (*bind).release = BindingRelease::KeyRelease;
            }
            bind = (*bind).bindings.tqe_next;
        }
    }
    let mut result: *mut Binding = std::ptr::null_mut();
    bind = (*head).tqh_first;
    while !bind.is_null() {
        let next = (*bind).bindings.tqe_next;
        let groups = (*bind).event_state_mask & 0xffff0000;
        if (*bind).input_type != input_type || state_filtered & groups != groups {
            bind = next;
            continue;
        }
        let symbol_keyboard = input_type == InputType::Keyboard && !(*bind).symbol.is_null();
        if !symbol_keyboard && (*bind).keycode != u32::from(input_code) {
            bind = next;
            continue;
        }
        let mut key = (*bind).keycodes_head.tqh_first;
        let mut found = false;
        while !key.is_null() {
            let same_key = !symbol_keyboard || (*key).keycode == input_code as u8;
            if same_key
                && modifier_match(
                    (*key).modifiers,
                    state_filtered,
                    (*bind).release,
                    is_release,
                )
            {
                found = true;
                break;
            }
            key = (*key).keycodes.tqe_next;
        }
        if found {
            match release_action((*bind).release, is_release) {
                ReleaseAction::Arm => {
                    (*bind).release = BindingRelease::KeyReleaseIgnoreMods;
                    if !result.is_null() {
                        break;
                    }
                }
                ReleaseAction::Run => {
                    if is_release {
                        return bind;
                    }
                    if result.is_null() {
                        result = bind;
                    }
                }
                ReleaseAction::Skip => {}
            }
        }
        bind = next;
    }
    result
}

pub unsafe fn get_binding_from_event(
    response_type: u8,
    event_state: u16,
    detail: u8,
) -> *mut Binding {
    // Bare core opcodes preserve the historical treatment of SendEvent flags.
    let is_release = matches!(response_type, 3 | 5);
    let input_type = if matches!(response_type, 4 | 5) {
        InputType::Mouse
    } else {
        InputType::Keyboard
    };
    let event_detail = u16::from(detail);
    let mut state_filtered: u32 =
        (event_state as i32 & !((::xcb::x::ModMask::LOCK.bits()) as i32)) as u32;
    {
        let log_value_0 = (state_filtered);
        ::log::debug!("../src/bindings.c:get_binding_from_xcb_event:329 - (removed capslock, state = 0x{:x})\n", log_value_0);
    };
    state_filtered = filtered_event_state(event_state);
    {
        let log_value_0 = (state_filtered);
        ::log::debug!("../src/bindings.c:get_binding_from_xcb_event:354 - (transformed keyboard group, state = 0x{:x})\n", log_value_0);
    };
    get_binding(state_filtered, is_release, event_detail, input_type)
}
unsafe extern "C" fn add_keycode_if_matches(
    keymap: *mut xkb_keymap,
    key: xkb_keycode_t,
    data: *mut ::core::ffi::c_void,
) {
    let resolving: *const Resolve = data as *const Resolve;
    let mut numlock_state: *mut xkb_state = (*resolving).xkb_state_numlock;
    let mut sym: xkb_keysym_t = xkb_state_key_get_one_sym((*resolving).xkb_state, key);
    if sym != (*resolving).keysym {
        let layout: xkb_layout_index_t =
            xkb_state_key_get_layout((*resolving).xkb_state, key) as xkb_layout_index_t;
        if layout == XKB_LAYOUT_INVALID as xkb_layout_index_t {
            return;
        }
        if xkb_state_key_get_level((*resolving).xkb_state, key, layout) > 1 as xkb_level_index_t {
            return;
        }
        if sym >= XKB_KEY_KP_Space as xkb_keysym_t && sym <= XKB_KEY_KP_Equal as xkb_keysym_t {
            return;
        }
        numlock_state = (*resolving).xkb_state_numlock_no_shift;
        sym = xkb_state_key_get_one_sym((*resolving).xkb_state_no_shift, key);
        if sym != (*resolving).keysym {
            return;
        }
    }
    let bind: *mut Binding = (*resolving).bind;
    append_keycode(bind, key as u8, (*bind).event_state_mask);
    append_keycode(
        bind,
        key as u8,
        (*bind).event_state_mask | (::xcb::x::ModMask::LOCK.bits()) as i32 as u32,
    );
    if (*bind).event_state_mask & xcb_numlock_mask != xcb_numlock_mask {
        let sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(numlock_state, key);
        if sym_numlock == (*resolving).keysym {
            append_keycode(bind, key as u8, (*bind).event_state_mask | xcb_numlock_mask);
            append_keycode(
                bind,
                key as u8,
                (*bind).event_state_mask
                    | xcb_numlock_mask
                    | (::xcb::x::ModMask::LOCK.bits()) as i32 as u32,
            );
        } else {
            {
                let log_value_0 = (key) as i32;
                let log_value_1 = (sym_numlock) as u32;
                ::log::debug!("../src/bindings.c:add_keycode_if_matches:440 - Skipping automatic numlock fallback, key {} resolves to 0x{:x} with numlock\n", log_value_0, log_value_1);
            };
        }
    }
}

pub unsafe fn translate_keysyms() {
    let mut temporary_states = XkbStates::default();
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut dummy_state: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut has_errors: bool = false;
    dummy_state = temporary_states.create(xkb_keymap);
    if dummy_state.is_null()
        || {
            dummy_state_no_shift = temporary_states.create(xkb_keymap);
            dummy_state_no_shift.is_null()
        }
        || {
            dummy_state_numlock = temporary_states.create(xkb_keymap);
            dummy_state_numlock.is_null()
        }
        || {
            dummy_state_numlock_no_shift = temporary_states.create(xkb_keymap);
            dummy_state_numlock_no_shift.is_null()
        }
    {
        ::log::error!("ERROR: Could not create XKB state, cannot translate keysyms.\n");
    } else {
        bind = ::core::ptr::null_mut::<Binding>();
        bind = (*bindings).tqh_first;
        while !bind.is_null() {
            if (*bind).input_type == InputType::Mouse {
                let button = match parse_long(
                    std::ffi::CStr::from_ptr(
                        (*bind).symbol.add(
                            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                                .wrapping_sub(1_usize),
                        ),
                    )
                    .to_bytes(),
                    10,
                ) {
                    Ok(button) => button,
                    Err(error) => {
                        {
                            // Snapshot native bytes before the logger can dispatch a callback.
                            let log_pointer_1 = (*bind).symbol;
                            let log_value_0 = i3_common::logging::string_bytes(
                                (if log_pointer_1.is_null() {
                                    None
                                } else {
                                    Some(std::ffi::CStr::from_ptr(log_pointer_1))
                                })
                                .map(std::ffi::CStr::to_bytes),
                                None,
                            )
                            .to_vec();
                            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                                ::log::error!(
                                    "ERROR: Could not translate string to button: \"{}\"\n",
                                    log_value_0_text
                                );
                            } else {
                                let mut message = Vec::new();
                                message.extend_from_slice(
                                    "ERROR: Could not translate string to button: \"".as_bytes(),
                                );
                                message.extend_from_slice(&log_value_0);
                                message.extend_from_slice("\"\n".as_bytes());
                                i3_common::logging::emit(
                                    i3_common::logging::Level::Error,
                                    &message,
                                );
                            }
                        };

                        // Native i3 continues with strtol's output even after
                        // logging malformed button input. Preserve that value.
                        error.value
                    }
                };
                let key: u8 = button as u8;
                (*bind).keycode = key as u32;
                {
                    let log_value_0 = (key as i32) as i32;
                    ::log::debug!("../src/bindings.c:translate_keysyms:474 - Binding Mouse button, Keycode = {}\n", log_value_0);
                };
            }
            let group = translation_group((*bind).event_state_mask);
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = i3_common::logging::pointer(bind);
                let log_value_1 = (group) as i32;
                let log_value_2 = ((*bind).event_state_mask) as i32;
                let log_pointer_4 = if (*bind).event_state_mask
                    & KeyboardGroups::GROUP2.bits() as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                };
                let log_value_3 = i3_common::logging::string_bytes(
                    (if log_pointer_4.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_4))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                let log_pointer_5 = if (*bind).event_state_mask
                    & KeyboardGroups::GROUP3.bits() as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                };
                let log_value_4 = i3_common::logging::string_bytes(
                    (if log_pointer_5.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_5))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                let log_pointer_6 = if (*bind).event_state_mask
                    & KeyboardGroups::GROUP4.bits() as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                };
                let log_value_5 = i3_common::logging::string_bytes(
                    (if log_pointer_6.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(log_pointer_6))
                    })
                    .map(std::ffi::CStr::to_bytes),
                    None,
                )
                .to_vec();
                if let (Ok(log_value_3_text), Ok(log_value_4_text), Ok(log_value_5_text)) = (
                    std::str::from_utf8(&log_value_3),
                    std::str::from_utf8(&log_value_4),
                    std::str::from_utf8(&log_value_5),
                ) {
                    ::log::debug!("../src/bindings.c:translate_keysyms:492 - Binding {} group = {}, event_state_mask = {}, &2 = {}, &3 = {}, &4 = {}\n", log_value_0, log_value_1, log_value_2, log_value_3_text, log_value_4_text, log_value_5_text);
                } else {
                    let mut message = Vec::new();
                    message.extend_from_slice(format!("../src/bindings.c:translate_keysyms:492 - Binding {} group = {}, event_state_mask = {}, &2 = ", log_value_0, log_value_1, log_value_2).as_bytes());
                    message.extend_from_slice(&log_value_3);
                    message.extend_from_slice(", &3 = ".as_bytes());
                    message.extend_from_slice(&log_value_4);
                    message.extend_from_slice(", &4 = ".as_bytes());
                    message.extend_from_slice(&log_value_5);
                    message.extend_from_slice("\n".as_bytes());
                    i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                }
            };
            xkb_state_update_mask(
                dummy_state,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_no_shift,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    ^ (::xcb::x::KeyButMask::SHIFT.bits()) as i32 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_numlock,
                (*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    | xcb_numlock_mask as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            xkb_state_update_mask(
                dummy_state_numlock_no_shift,
                ((*bind).event_state_mask as xkb_mod_mask_t & 0x1fff as xkb_mod_mask_t
                    | xcb_numlock_mask as xkb_mod_mask_t)
                    ^ (::xcb::x::KeyButMask::SHIFT.bits()) as i32 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            if (*bind).keycode > 0_u32 {
                while !(*bind).keycodes_head.tqh_first.is_null() {
                    let mut first: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
                    if !(*first).keycodes.tqe_next.is_null() {
                        (*(*first).keycodes.tqe_next).keycodes.tqe_prev =
                            (*first).keycodes.tqe_prev;
                    } else {
                        (*bind).keycodes_head.tqh_last = (*first).keycodes.tqe_prev;
                    }
                    *(*first).keycodes.tqe_prev = (*first).keycodes.tqe_next;
                    free(first as *mut ::core::ffi::c_void);
                    first = ::core::ptr::null_mut::<Binding_Keycode>();
                }
                append_keycode(bind, (*bind).keycode as u8, (*bind).event_state_mask);
                append_keycode(
                    bind,
                    (*bind).keycode as u8,
                    (*bind).event_state_mask | (::xcb::x::ModMask::LOCK.bits()) as i32 as u32,
                );
                if (*bind).event_state_mask & xcb_numlock_mask != xcb_numlock_mask {
                    let sym: xkb_keysym_t =
                        xkb_state_key_get_one_sym(dummy_state, (*bind).keycode as xkb_keycode_t);
                    let sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(
                        dummy_state_numlock,
                        (*bind).keycode as xkb_keycode_t,
                    );
                    if sym == sym_numlock {
                        append_keycode(
                            bind,
                            (*bind).keycode as u8,
                            (*bind).event_state_mask | xcb_numlock_mask,
                        );
                        append_keycode(
                            bind,
                            (*bind).keycode as u8,
                            (*bind).event_state_mask
                                | xcb_numlock_mask
                                | (::xcb::x::ModMask::LOCK.bits()),
                        );
                    } else {
                        {
                            let log_value_0 = ((*bind).keycode) as i32;
                            let log_value_1 = (sym_numlock) as u32;
                            ::log::debug!("../src/bindings.c:translate_keysyms:560 - Skipping automatic numlock fallback, key {} resolves to 0x{:x} with numlock\n", log_value_0, log_value_1);
                        };
                    }
                }
            } else {
                let keysym: xkb_keysym_t =
                    xkb_keysym_from_name((*bind).symbol, XKB_KEYSYM_NO_FLAGS) as xkb_keysym_t;
                if keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_pointer_1 = (*bind).symbol;
                        let log_value_0 = i3_common::logging::string_bytes(
                            (if log_pointer_1.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_1))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                            ::log::error!(
                                "ERROR: Could not translate string to key symbol: \"{}\"\n",
                                log_value_0_text
                            );
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(
                                "ERROR: Could not translate string to key symbol: \"".as_bytes(),
                            );
                            message.extend_from_slice(&log_value_0);
                            message.extend_from_slice("\"\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                        }
                    };
                } else {
                    let mut resolving: Resolve = Resolve {
                        bind,
                        keysym,
                        xkb_state: dummy_state,
                        xkb_state_no_shift: dummy_state_no_shift,
                        xkb_state_numlock: dummy_state_numlock,
                        xkb_state_numlock_no_shift: dummy_state_numlock_no_shift,
                    };
                    while !(*bind).keycodes_head.tqh_first.is_null() {
                        let mut first_0: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
                        if !(*first_0).keycodes.tqe_next.is_null() {
                            (*(*first_0).keycodes.tqe_next).keycodes.tqe_prev =
                                (*first_0).keycodes.tqe_prev;
                        } else {
                            (*bind).keycodes_head.tqh_last = (*first_0).keycodes.tqe_prev;
                        }
                        *(*first_0).keycodes.tqe_prev = (*first_0).keycodes.tqe_next;
                        free(first_0 as *mut ::core::ffi::c_void);
                        first_0 = ::core::ptr::null_mut::<Binding_Keycode>();
                    }
                    xkb_keymap_key_for_each(
                        xkb_keymap,
                        add_keycode_if_matches
                            as unsafe extern "C" fn(
                                *mut xkb_keymap,
                                xkb_keycode_t,
                                *mut ::core::ffi::c_void,
                            ) -> (),
                        &raw mut resolving as *mut ::core::ffi::c_void,
                    );
                    let mut keycodes = String::new();
                    let mut num_keycodes: i32 = 0;
                    let mut binding_keycode_3: *mut Binding_Keycode =
                        (*bind).keycodes_head.tqh_first;

                    while !binding_keycode_3.is_null() {
                        use std::fmt::Write;
                        write!(&mut keycodes, " {}", (*binding_keycode_3).keycode).unwrap();
                        num_keycodes += 1;
                        let mut check: *mut Binding = (*bindings).tqh_first;

                        while !check.is_null() {
                            if !(check == bind)
                                && (*check).symbol.is_null()
                                && !((*check).keycode != (*binding_keycode_3).keycode as u32
                                    || (*check).event_state_mask != (*binding_keycode_3).modifiers
                                    || (*check).release as u32 != (*bind).release as u32)
                            {
                                has_errors = true;
                                {
                                    // Snapshot native bytes before the logger can dispatch a callback.
                                    let log_pointer_1 = (*bind).symbol;
                                    let log_value_0 = i3_common::logging::string_bytes(
                                        (if log_pointer_1.is_null() {
                                            None
                                        } else {
                                            Some(std::ffi::CStr::from_ptr(log_pointer_1))
                                        })
                                        .map(std::ffi::CStr::to_bytes),
                                        None,
                                    )
                                    .to_vec();
                                    let log_value_1 = ((*check).keycode) as i32;
                                    let log_value_2 = ((*bind).event_state_mask) as u32;
                                    if let (Ok(log_value_0_text),) =
                                        (std::str::from_utf8(&log_value_0),)
                                    {
                                        ::log::error!("ERROR: Duplicate keybinding in config file:\n  keysym = {}, keycode = {}, state_mask = 0x{:x}\n", log_value_0_text, log_value_1, log_value_2);
                                    } else {
                                        let mut message = Vec::new();
                                        message.extend_from_slice("ERROR: Duplicate keybinding in config file:\n  keysym = ".as_bytes());
                                        message.extend_from_slice(&log_value_0);
                                        message.extend_from_slice(
                                            format!(
                                                ", keycode = {}, state_mask = 0x{:x}\n",
                                                log_value_1, log_value_2
                                            )
                                            .as_bytes(),
                                        );
                                        i3_common::logging::emit(
                                            i3_common::logging::Level::Error,
                                            &message,
                                        );
                                    }
                                };
                            }
                            check = (*check).bindings.tqe_next;
                        }
                        binding_keycode_3 = (*binding_keycode_3).keycodes.tqe_next;
                    }
                    let keycodes = std::ffi::CString::new(keycodes).unwrap();
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = ((*bind).event_state_mask) as u32;
                        let log_pointer_2 = (*bind).symbol;
                        let log_value_1 = i3_common::logging::string_bytes(
                            (if log_pointer_2.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_2))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        let log_value_2 = (keysym) as u32;
                        let log_pointer_4 = keycodes.as_ptr();
                        let log_value_3 = i3_common::logging::string_bytes(
                            (if log_pointer_4.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_4))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        let log_value_4 = (num_keycodes);
                        if let (Ok(log_value_1_text), Ok(log_value_3_text)) = (
                            std::str::from_utf8(&log_value_1),
                            std::str::from_utf8(&log_value_3),
                        ) {
                            ::log::debug!("../src/bindings.c:translate_keysyms:618 - state=0x{:x}, cfg=\"{}\", sym=0x{:x} → keycodes{} ({})\n", log_value_0, log_value_1_text, log_value_2, log_value_3_text, log_value_4);
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(format!("../src/bindings.c:translate_keysyms:618 - state=0x{:x}, cfg=\"", log_value_0).as_bytes());
                            message.extend_from_slice(&log_value_1);
                            message.extend_from_slice(
                                format!("\", sym=0x{:x} → keycodes", log_value_2).as_bytes(),
                            );
                            message.extend_from_slice(&log_value_3);
                            message.extend_from_slice(format!(" ({})\n", log_value_4).as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
                        }
                    };
                }
            }
            bind = (*bind).bindings.tqe_next;
        }
    }

    drop(temporary_states);
    if has_errors {
        start_config_error_nagbar(current_configpath, true);
    }
}

// Preserve the historical mode event's raw insertion, including unescaped
// quotes/invalid UTF-8. This is a wire-compatibility builder, not a C-text API.
fn mode_event_payload(name: &[u8], pango_markup: bool) -> Vec<u8> {
    let name = name.split(|byte| *byte == 0).next().unwrap();
    let mut payload = b"{\"change\":\"".to_vec();
    payload.extend_from_slice(name);
    payload.extend_from_slice(if pango_markup {
        b"\", \"pango_markup\":true}"
    } else {
        b"\", \"pango_markup\":false}"
    });
    payload
}

pub unsafe fn switch_mode(new_mode: &[u8]) {
    let requested_mode = new_mode.split(|byte| *byte == 0).next().unwrap().to_vec();
    let new_mode = requested_mode.as_slice();
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    {
        let log_value_0 = new_mode.to_vec();
        if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
            ::log::debug!(
                "../src/bindings.c:switch_mode:642 - Switching to mode {}\n",
                log_value_0_text
            );
        } else {
            let mut message = Vec::new();
            message.extend_from_slice(
                "../src/bindings.c:switch_mode:642 - Switching to mode ".as_bytes(),
            );
            message.extend_from_slice(&log_value_0);
            message.extend_from_slice("\n".as_bytes());
            i3_common::logging::emit(i3_common::logging::Level::Debug, &message);
        }
    };
    mode = modes.slh_first;
    while !mode.is_null() {
        if (*mode).name != new_mode {
            mode = (*mode).modes.sle_next;
        } else {
            let mode_name = (*mode).name.clone();
            let pango_markup = (*mode).pango_markup;
            ungrab_all_keys();
            bindings = (*mode).bindings_ptr();
            current_binding_mode = Some((*mode).name.clone());
            translate_keysyms();
            grab_all_keys();
            regrab_all_buttons();
            let mut bind: *mut Binding = (*bindings).tqh_first;

            while !bind.is_null() {
                if (*bind).release == BindingRelease::KeyReleaseIgnoreMods {
                    (*bind).release = BindingRelease::KeyRelease;
                }
                bind = (*bind).bindings.tqe_next;
            }
            let event_msg = mode_event_payload(&mode_name, pango_markup);
            ipc_send_event(b"mode", I3_IPC_EVENT_MODE as u32, &event_msg);
            return;
        }
    }
    ::log::error!("ERROR: Mode not found\n");
}
unsafe extern "C" fn reorder_binding_cmp(
    a: *const ::core::ffi::c_void,
    b: *const ::core::ffi::c_void,
) -> i32 {
    let first: *mut Binding = *(a as *mut *mut Binding);
    let second: *mut Binding = *(b as *mut *mut Binding);
    if (*first).event_state_mask < (*second).event_state_mask {
        return 1;
    }
    if (*first).event_state_mask == (*second).event_state_mask {
        return 0;
    }
    -(1)
}
unsafe fn reorder_bindings_of_mode(mode: *mut Mode) {
    let mut tmp = Vec::<*mut Binding>::new();
    let mut current = (*(*mode).bindings.as_ptr()).tqh_first;
    while !current.is_null() {
        tmp.push(current);
        current = (*current).bindings.tqe_next;
    }
    let n = tmp.len();
    qsort(
        tmp.as_mut_ptr().cast(),
        n,
        ::core::mem::size_of::<*mut Binding>() as usize,
        Some(
            reorder_binding_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> i32,
        ),
    );
    let reordered_owner = OwnedBindingsHead::new();
    let reordered = reordered_owner.as_ptr();
    let mut i = 0usize;
    while i < n {
        current = tmp[i];
        if !(*current).bindings.tqe_next.is_null() {
            (*(*current).bindings.tqe_next).bindings.tqe_prev = (*current).bindings.tqe_prev;
        } else {
            (*(*mode).bindings.as_ptr()).tqh_last = (*current).bindings.tqe_prev;
        }
        *(*current).bindings.tqe_prev = (*current).bindings.tqe_next;
        (*current).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
        (*current).bindings.tqe_prev = (*reordered).tqh_last;
        *(*reordered).tqh_last = current;
        (*reordered).tqh_last = &raw mut (*current).bindings.tqe_next;
        i += 1;
    }

    if (*(*mode).bindings.as_ptr()).tqh_first.is_null() {
    } else {
        __assert_fail(
            c"TAILQ_EMPTY(mode->bindings)".as_ptr(),
            c"../src/bindings.c".as_ptr(),
            713_u32,
            c"void reorder_bindings_of_mode(struct Mode *)".as_ptr(),
        );
    };
    if bindings == (*mode).bindings_ptr() {
        bindings = reordered;
    }
    (*mode).bindings = reordered_owner;
}

pub unsafe fn reorder_bindings() {
    let mut mode: *mut Mode = modes.slh_first;

    while !mode.is_null() {
        reorder_bindings_of_mode(mode);
        mode = (*mode).modes.sle_next;
    }
}
unsafe fn binding_same_key(a: *mut Binding, b: *mut Binding) -> bool {
    if (*a).input_type != (*b).input_type {
        return false;
    }
    if (*a).symbol.is_null() && !(*b).symbol.is_null()
        || !(*a).symbol.is_null() && (*b).symbol.is_null()
    {
        return false;
    }
    if !(*a).symbol.is_null() && strcasecmp((*a).symbol, (*b).symbol) != 0 {
        return false;
    }
    if (*a).keycode != (*b).keycode
        || (*a).event_state_mask != (*b).event_state_mask
        || (*a).release as u32 != (*b).release as u32
    {
        return false;
    }
    true
}

pub unsafe fn check_for_duplicate_bindings(context: *mut context) {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut current: *mut Binding = (*bindings).tqh_first;

    while !current.is_null() {
        bind = (*bindings).tqh_first;
        while !bind.is_null() {
            if bind == current {
                break;
            }
            if binding_same_key(bind, current) {
                (*context).has_errors = true;
                if (*current).keycode != 0_u32 {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = ((*current).event_state_mask);
                        let log_value_1 = ((*current).keycode) as i32;
                        let log_pointer_3 = (*current).command;
                        let log_value_2 = i3_common::logging::string_bytes(
                            (if log_pointer_3.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_3))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_2_text),) = (std::str::from_utf8(&log_value_2),) {
                            ::log::error!("ERROR: Duplicate keybinding in config file:\n  state mask 0x{:x} with keycode {}, command \"{}\"\n", log_value_0, log_value_1, log_value_2_text);
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(format!("ERROR: Duplicate keybinding in config file:\n  state mask 0x{:x} with keycode {}, command \"", log_value_0, log_value_1).as_bytes());
                            message.extend_from_slice(&log_value_2);
                            message.extend_from_slice("\"\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                        }
                    };
                } else {
                    {
                        // Snapshot native bytes before the logger can dispatch a callback.
                        let log_value_0 = ((*current).event_state_mask);
                        let log_pointer_2 = (*current).symbol;
                        let log_value_1 = i3_common::logging::string_bytes(
                            (if log_pointer_2.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_2))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        let log_pointer_3 = (*current).command;
                        let log_value_2 = i3_common::logging::string_bytes(
                            (if log_pointer_3.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(log_pointer_3))
                            })
                            .map(std::ffi::CStr::to_bytes),
                            None,
                        )
                        .to_vec();
                        if let (Ok(log_value_1_text), Ok(log_value_2_text)) = (
                            std::str::from_utf8(&log_value_1),
                            std::str::from_utf8(&log_value_2),
                        ) {
                            ::log::error!("ERROR: Duplicate keybinding in config file:\n  state mask 0x{:x} with keysym {}, command \"{}\"\n", log_value_0, log_value_1_text, log_value_2_text);
                        } else {
                            let mut message = Vec::new();
                            message.extend_from_slice(format!("ERROR: Duplicate keybinding in config file:\n  state mask 0x{:x} with keysym ", log_value_0).as_bytes());
                            message.extend_from_slice(&log_value_1);
                            message.extend_from_slice(", command \"".as_bytes());
                            message.extend_from_slice(&log_value_2);
                            message.extend_from_slice("\"\n".as_bytes());
                            i3_common::logging::emit(i3_common::logging::Level::Error, &message);
                        }
                    };
                }
            }
            bind = (*bind).bindings.tqe_next;
        }
        current = (*current).bindings.tqe_next;
    }
}
unsafe fn binding_copy(bind: *mut Binding) -> *mut Binding {
    let ret: *mut Binding = smalloc(::core::mem::size_of::<Binding>() as usize) as *mut Binding;
    *ret = *bind;
    if !(*bind).symbol.is_null() {
        (*ret).symbol = sstrdup((*bind).symbol);
    }
    if !(*bind).command.is_null() {
        (*ret).command = sstrdup((*bind).command);
    }
    (*ret).keycodes_head.tqh_first = ::core::ptr::null_mut::<Binding_Keycode>();
    (*ret).keycodes_head.tqh_last = &raw mut (*ret).keycodes_head.tqh_first;
    let mut binding_keycode: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;

    while !binding_keycode.is_null() {
        let ret_binding_keycode: *mut Binding_Keycode =
            smalloc(::core::mem::size_of::<Binding_Keycode>()) as *mut Binding_Keycode;
        *ret_binding_keycode = *binding_keycode;
        (*ret_binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
        (*ret_binding_keycode).keycodes.tqe_prev = (*ret).keycodes_head.tqh_last;
        *(*ret).keycodes_head.tqh_last = ret_binding_keycode;
        (*ret).keycodes_head.tqh_last = &raw mut (*ret_binding_keycode).keycodes.tqe_next;
        binding_keycode = (*binding_keycode).keycodes.tqe_next;
    }
    ret
}

pub unsafe fn binding_free(mut bind: *mut Binding) {
    if bind.is_null() {
        return;
    }
    while !(*bind).keycodes_head.tqh_first.is_null() {
        let mut first: *mut Binding_Keycode = (*bind).keycodes_head.tqh_first;
        if !(*first).keycodes.tqe_next.is_null() {
            (*(*first).keycodes.tqe_next).keycodes.tqe_prev = (*first).keycodes.tqe_prev;
        } else {
            (*bind).keycodes_head.tqh_last = (*first).keycodes.tqe_prev;
        }
        *(*first).keycodes.tqe_prev = (*first).keycodes.tqe_next;
        free(first as *mut ::core::ffi::c_void);
        first = ::core::ptr::null_mut::<Binding_Keycode>();
    }
    free((*bind).symbol as *mut ::core::ffi::c_void);
    (*bind).symbol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*bind).command as *mut ::core::ffi::c_void);
    (*bind).command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free(bind as *mut ::core::ffi::c_void);
    bind = ::core::ptr::null_mut::<Binding>();
}

pub unsafe fn run_binding(bind: *mut Binding, con: *mut Con) -> *mut CommandResult {
    let source = std::ffi::CStr::from_ptr((*bind).command).to_bytes();
    let command = if con.is_null() {
        source.to_vec()
    } else {
        let mut command =
            format!("[con_id=\"{}\"] ", i3_common::logging::pointer(con)).into_bytes();
        command.extend_from_slice(source);
        command
    };
    let bind_cp = binding_copy(bind);
    let binding_owner = BindingCopy(bind_cp);
    let modename = current_binding_mode
        .clone()
        .unwrap_or_else(|| DEFAULT_BINDING_MODE.to_vec());
    let result: *mut CommandResult =
        parse_command(&command, None, ::core::ptr::null_mut::<ipc_client>());
    if (*result).needs_tree_render {
        tree_render();
    }
    if (*result).parse_error {
        let error_path = errorfilename.clone();
        let mut pager_bytes = b"i3-sensible-pager \"".to_vec();
        pager_bytes.extend_from_slice(
            error_path
                .as_deref()
                .map_or(b"(null)".as_slice(), |path| path.as_os_str().as_bytes()),
        );
        pager_bytes.extend_from_slice(b"\"\n");
        let pageraction = std::ffi::CString::new(pager_bytes).unwrap();
        let font_pattern = config
            .font
            .pattern()
            .map(|bytes| std::ffi::CString::new(bytes).expect("native font argument has no NUL"));
        let mut argv: [*mut ::core::ffi::c_char; 11] = [
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            c"-f".as_ptr().cast_mut(),
            font_pattern
                .as_ref()
                .map_or(std::ptr::null_mut(), |pattern| pattern.as_ptr().cast_mut()),
            c"-t".as_ptr().cast_mut(),
            c"error".as_ptr().cast_mut(),
            c"-m".as_ptr().cast_mut(),
            c"The configured command for this shortcut could not be run successfully."
                .as_ptr()
                .cast_mut(),
            c"-b".as_ptr().cast_mut(),
            c"show errors".as_ptr().cast_mut(),
            pageraction.as_ptr().cast_mut(),
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        start_nagbar(
            &raw mut command_error_nagbar_pid,
            &raw mut argv as *mut *mut ::core::ffi::c_char,
        );
    }
    ipc_send_binding_event(b"run", bind_cp, &modename);
    drop(binding_owner);
    result
}
#[derive(Default)]
struct RuleNames([Option<std::ffi::CString>; 5]);
impl RuleNames {
    fn native(&self) -> xkb_rule_names {
        let field = |index: usize| {
            self.0[index]
                .as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr())
        };
        xkb_rule_names {
            rules: field(0),
            model: field(1),
            layout: field(2),
            variant: field(3),
            options: field(4),
        }
    }
}
fn rule_names(bytes: &[u8]) -> RuleNames {
    let mut names = RuleNames::default();
    let mut remaining = bytes;
    for field in &mut names.0 {
        if remaining.is_empty() {
            break;
        }
        let length = remaining
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(remaining.len());
        *field = Some(std::ffi::CString::new(&remaining[..length]).unwrap());
        remaining = remaining.get(length + 1..).unwrap_or_default();
    }
    names
}
/// Query only on the live event-thread display; replies own all byte views.
unsafe fn fill_rmlvo_from_root() -> Option<RuleNames> {
    let connection = connection();
    let atom = connection
        .wait_for_reply(connection.send_request(&x::InternAtom {
            only_if_exists: false,
            name: b"_XKB_RULES_NAMES",
        }))
        .ok()?
        .atom();
    let query = |words| {
        let cookie = connection.send_request_unchecked(&x::GetProperty {
            delete: false,
            window: x::Window::new(root),
            property: atom,
            r#type: x::ATOM_ANY,
            long_offset: 0,
            long_length: words,
        });
        connection.wait_for_reply_unchecked(cookie).ok().flatten()
    };
    let mut words = 256u32;
    let mut reply = query(words)?;
    if crate::xcb::property_payload(&reply).is_some_and(|bytes| !bytes.is_empty())
        && reply.bytes_after() > 0
    {
        words = words.wrapping_add(reply.bytes_after().div_ceil(4));
        reply = query(words)?;
    }
    let bytes = crate::xcb::property_payload(&reply)?;
    if bytes.is_empty() {
        return None;
    }
    Some(rule_names(bytes))
}

pub unsafe fn load_keymap() -> bool {
    if xkb_context.is_null() {
        xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if xkb_context.is_null() {
            ::log::error!("ERROR: Could not create xkbcommon context\n");
            return false;
        }
    }
    let mut new_keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
    let mut device_id: i32 = 0;
    if xkb_supported as i32 != 0 && {
        device_id = xkb_x11_get_core_keyboard_device_id(conn.cast());
        device_id > -(1)
    } {
        new_keymap = xkb_x11_keymap_new_from_device(
            xkb_context,
            conn.cast(),
            device_id,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
        if new_keymap.is_null() {
            ::log::error!("ERROR: xkb_x11_keymap_new_from_device failed\n");
            return false;
        }
    } else {
        ::log::info!("No XKB / core keyboard device? Assembling keymap from local RMLVO.\n");
        let owned_names = fill_rmlvo_from_root();
        if owned_names.is_none() {
            ::log::error!("ERROR: Could not get _XKB_RULES_NAMES atom from root window, falling back to defaults.\n");
        }
        let owned_names = owned_names.unwrap_or_default();
        let names = owned_names.native();
        new_keymap = xkb_keymap_new_from_names(xkb_context, &names, XKB_KEYMAP_COMPILE_NO_FLAGS);
        if new_keymap.is_null() {
            ::log::error!("ERROR: xkb_keymap_new_from_names failed\n");
            return false;
        }
    }
    xkb_keymap_unref(xkb_keymap);
    xkb_keymap = new_keymap;
    true
}

pub unsafe fn bindings_get_buttons_to_grab() -> *mut libc::c_int {
    let mut buttons = vec![1, 2, 3];
    let mut bind = (*bindings).tqh_first;
    while !bind.is_null() {
        if buttons.len() == 24 {
            break;
        }
        if (*bind).input_type == InputType::Mouse && (*bind).whole_window {
            if let Ok(button) = parse_long(
                std::ffi::CStr::from_ptr((*bind).symbol.add(6)).to_bytes(),
                10,
            ) {
                append_button(&mut buttons, button);
            } else {
                ::log::error!("ERROR: Could not parse button number, skipping this binding. Please report this bug in i3.\n");
            }
        }
        bind = (*bind).bindings.tqe_next;
    }
    buttons.push(0);
    let pointer = scalloc(buttons.len(), std::mem::size_of::<i32>()).cast::<i32>();
    std::ptr::copy_nonoverlapping(buttons.as_ptr(), pointer, buttons.len());
    pointer
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::config_directives::event_state_from_str;
use crate::config_parser::start_config_error_nagbar;
use crate::ipc::ipc_send_event;
use crate::tree::tree_render;
use crate::util::parse_long;
use crate::util::start_nagbar;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrdup;

// Canonical Pango backend ABI; font policy remains local.

use xkbcommon::xkb::ffi::*;
use xkbcommon::xkb::x11::ffi::*;

use xkbcommon::xkb::keysyms::{
    KEY_KP_Equal as XKB_KEY_KP_Equal, KEY_KP_Space as XKB_KEY_KP_Space,
    KEY_NoSymbol as XKB_KEY_NoSymbol,
};

use crate::handlers::xkb_current_group;

use crate::application::xkb_supported;

use libc::{free, qsort, strcasecmp, strcmp, strncasecmp};

use i3_common::native_stdio::__assert_fail;

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::current_binding_mode;
use crate::application::root;
use crate::config::config;
use crate::config::current_configpath;
use crate::config::ungrab_all_keys;
use crate::log::errorfilename;
use crate::xcb::xcb_grab_buttons;

use crate::xcb::xcb_numlock_mask;

struct Native<T>(std::ptr::NonNull<T>);
impl<T> Native<T> {
    unsafe fn take(pointer: *mut T) -> Option<Self> {
        std::ptr::NonNull::new(pointer).map(Self)
    }
    fn raw(&self) -> *mut T {
        self.0.as_ptr()
    }
}
impl<T> Drop for Native<T> {
    fn drop(&mut self) {
        unsafe {
            free(self.raw().cast());
        }
    }
}
#[derive(Default)]
struct XkbStates(Vec<*mut xkb_state>);
impl XkbStates {
    unsafe fn create(&mut self, keymap: *mut xkb_keymap) -> *mut xkb_state {
        let state = xkb_state_new(keymap);
        if !state.is_null() {
            self.0.push(state);
        }
        state
    }
}
impl Drop for XkbStates {
    fn drop(&mut self) {
        for state in self.0.drain(..) {
            unsafe {
                xkb_state_unref(state);
            }
        }
    }
}
fn grab_masks(mods: u32, numlock: u32) -> [u16; 4] {
    [
        mods as u16,
        (mods | numlock) as u16,
        (mods | 2) as u16,
        (mods | numlock | 2) as u16,
    ]
}
fn filtered_event_state(state: u16) -> u32 {
    (u32::from(state) & !2 & !0x6000) | ((1 << ((state & 0x6000) >> 13)) << 16)
}
fn modifier_match(key: u32, event: u32, release: BindingRelease, is_release: bool) -> bool {
    key & 0xffff == event & 0xffff || release == BindingRelease::KeyReleaseIgnoreMods && is_release
}
#[derive(Debug, PartialEq)]
enum ReleaseAction {
    Arm,
    Run,
    Skip,
}
fn release_action(release: BindingRelease, is_release: bool) -> ReleaseAction {
    if release == BindingRelease::KeyRelease && !is_release {
        ReleaseAction::Arm
    } else if release == BindingRelease::KeyPress && is_release {
        ReleaseAction::Skip
    } else {
        ReleaseAction::Run
    }
}
fn append_button(buttons: &mut Vec<i32>, button: libc::c_long) {
    if buttons.len() < 24
        && !buttons
            .iter()
            .any(|&existing| libc::c_long::from(existing) == button)
    {
        buttons.push(button as i32);
    }
}

/// Append an initialized native malloc node to an exclusively-owned valid queue.
unsafe fn append_keycode(bind: *mut Binding, keycode: u8, modifiers: u32) {
    let pointer = smalloc(std::mem::size_of::<Binding_Keycode>()).cast::<Binding_Keycode>();
    pointer.write(Binding_Keycode {
        keycode,
        modifiers,
        keycodes: crate::application::BindingKeycodeLinks {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: (*bind).keycodes_head.tqh_last,
        },
    });
    *(*bind).keycodes_head.tqh_last = pointer;
    (*bind).keycodes_head.tqh_last = &raw mut (*pointer).keycodes.tqe_next;
}

fn translation_group(mask: u32) -> u32 {
    KeyboardGroups::from_state_mask(mask).translation_group()
}

struct BindingCopy(*mut Binding);
impl Drop for BindingCopy {
    fn drop(&mut self) {
        unsafe {
            binding_free(self.0);
        }
    }
}

struct ServerGrab(Rc<xcb::Connection>);
impl Drop for ServerGrab {
    fn drop(&mut self) {
        self.0.send_request(&x::UngrabServer {});
    }
}

#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn native_queue_lookup_preserves_press_release_pair_order_and_modifier_rearming() {
        use crate::application::{keycodes_head, BindingKeycodeLinks, BindingLinks};
        fn binding(release: BindingRelease, key: *mut Binding_Keycode) -> Box<Binding> {
            Box::new(Binding {
                input_type: InputType::Keyboard,
                release,
                border: false,
                whole_window: false,
                exclude_titlebar: false,
                keycode: 42,
                event_state_mask: 1 << 16,
                symbol: std::ptr::null_mut(),
                command: std::ptr::null_mut(),
                keycodes_head: keycodes_head {
                    tqh_first: key,
                    tqh_last: std::ptr::null_mut(),
                },
                bindings: BindingLinks {
                    tqe_next: std::ptr::null_mut(),
                    tqe_prev: std::ptr::null_mut(),
                },
            })
        }
        let mut key = Box::new(Binding_Keycode {
            keycode: 42,
            modifiers: 4,
            keycodes: BindingKeycodeLinks {
                tqe_next: std::ptr::null_mut(),
                tqe_prev: std::ptr::null_mut(),
            },
        });
        let mut release = binding(BindingRelease::KeyRelease, &raw mut *key);
        let mut press = binding(BindingRelease::KeyPress, &raw mut *key);
        let release_ptr = &raw mut *release;
        let press_ptr = &raw mut *press;
        release.bindings.tqe_next = press_ptr;
        let mut head = bindings_head {
            tqh_first: release_ptr,
            tqh_last: std::ptr::null_mut(),
        };
        unsafe {
            assert_eq!(
                find_binding(&raw mut head, 0x10004, false, 42, InputType::Keyboard),
                press_ptr
            );
            assert_eq!(release.release, BindingRelease::KeyReleaseIgnoreMods);
            assert_eq!(
                find_binding(&raw mut head, 0x10000, true, 42, InputType::Keyboard),
                release_ptr
            );
            assert!(find_binding(&raw mut head, 0x10000, false, 99, InputType::Keyboard).is_null());
            assert_eq!(release.release, BindingRelease::KeyRelease);
            assert!(find_binding(&raw mut head, 0x20004, false, 42, InputType::Keyboard).is_null());
            assert!(find_binding(&raw mut head, 0x10004, false, 42, InputType::Mouse).is_null());
        }
    }
    #[test]
    fn group_lock_and_release_policy_interactions() {
        assert!(group_matches(0, 99));
        assert!(!group_matches(1 << 16, 4));
        assert!(group_matches(5 << 16, 2));
        assert_eq!(filtered_event_state(0x6003), 0x80001);
        assert_eq!(grab_masks(4, 16), [4, 20, 6, 22]);
        assert_eq!(
            release_action(BindingRelease::KeyRelease, false),
            ReleaseAction::Arm
        );
        assert_eq!(
            release_action(BindingRelease::KeyPress, true),
            ReleaseAction::Skip
        );
        assert!(modifier_match(
            4,
            8,
            BindingRelease::KeyReleaseIgnoreMods,
            true
        ));
        assert!(!modifier_match(
            4,
            8,
            BindingRelease::KeyReleaseIgnoreMods,
            false
        ));
    }
    #[test]
    fn rmlvo_keeps_missing_vs_empty_fields_and_final_unterminated_bytes() {
        let names = rule_names(b"rules\0\0us\0variant\0option");
        assert_eq!(names.0[0].as_ref().unwrap().as_bytes(), b"rules");
        assert_eq!(names.0[1].as_ref().unwrap().as_bytes(), b"");
        assert_eq!(names.0[4].as_ref().unwrap().as_bytes(), b"option");
        let names = rule_names(b"rules\0");
        assert!(names.0[1].is_none());
    }
    #[test]
    fn button_limit_and_long_comparison_keep_legacy_truncation() {
        let mut buttons = vec![1, 2, 3];
        append_button(&mut buttons, 2);
        assert_eq!(buttons.len(), 3);
        append_button(&mut buttons, i64::from(i32::MAX) + 1);
        append_button(&mut buttons, i64::from(i32::MAX) + 1);
        assert_eq!(&buttons[3..], &[i32::MIN, i32::MIN]);
        for button in 4..50 {
            append_button(&mut buttons, button);
        }
        assert_eq!(buttons.len(), 24);
    }
}

#[cfg(test)]
mod mode_ownership_tests {
    use super::*;

    #[test]
    fn mode_event_bytes_match_native_format_for_unescaped_and_non_utf8_names() {
        for name in [
            b"".as_slice(),
            b"normal",
            b"quote\"\\\xff",
            b"prefix\0ignored",
        ] {
            let prefix = name.split(|byte| *byte == 0).next().unwrap();
            let native_name = std::ffi::CString::new(prefix).unwrap();
            for markup in [false, true] {
                let mut output = [0u8; 256];
                let length = unsafe {
                    libc::snprintf(
                        output.as_mut_ptr().cast(),
                        output.len(),
                        c"{\"change\":\"%s\", \"pango_markup\":%s}".as_ptr(),
                        native_name.as_ptr(),
                        if markup {
                            c"true".as_ptr()
                        } else {
                            c"false".as_ptr()
                        },
                    )
                };
                assert!(length >= 0);
                assert_eq!(mode_event_payload(name, markup), output[..length as usize]);
            }
        }
    }

    #[test]
    fn reorder_and_remove_owned_modes_preserves_active_aliases_and_native_names() {
        if std::env::var_os("I333_MODE_OWNER_PROBE").is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "bindings::mode_ownership_tests::reorder_and_remove_owned_modes_preserves_active_aliases_and_native_names", "--nocapture"])
                .env("I333_MODE_OWNER_PROBE", "1")
                .env_remove("DISPLAY").env_remove("I3SOCK").env_remove("WAYLAND_DISPLAY")
                .output().unwrap();
            assert!(
                output.status.success(),
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return;
        }
        unsafe {
            assert!(modes.slh_first.is_null());
            let name = vec![0xff, b'm'];
            let mode = mode_from_name(&name, true);
            assert_eq!(mode_from_name(&name, false), mode);
            assert_eq!(mode_from_name(&[0xff, b'm', 0, b'x'], false), mode);
            assert!((*mode).pango_markup);
            let first = configure_binding(
                c"bindcode".as_ptr(),
                c"".as_ptr(),
                c"42".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                c"nop first".as_ptr(),
                &name,
                true,
            );
            let second = configure_binding(
                c"bindcode".as_ptr(),
                c"Mod4".as_ptr(),
                c"42".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                c"nop second".as_ptr(),
                &name,
                true,
            );
            bindings = (*mode).bindings_ptr();
            current_binding_mode = Some((*mode).name.clone());
            reorder_bindings();
            assert_eq!(bindings, (*mode).bindings_ptr());
            assert_eq!((*bindings).tqh_first, second);
            assert_eq!((*second).bindings.tqe_prev, &raw mut (*bindings).tqh_first);
            assert_eq!((*second).bindings.tqe_next, first);
            assert_eq!(
                (*first).bindings.tqe_prev,
                &raw mut (*second).bindings.tqe_next
            );
            assert_eq!((*bindings).tqh_last, &raw mut (*first).bindings.tqe_next);
            let retained_name = (*mode).name.clone();
            remove_first_mode();
            assert!(modes.slh_first.is_null());
            assert!(bindings.is_null());
            assert_eq!(
                current_binding_mode
                    .as_deref()
                    .unwrap_or(DEFAULT_BINDING_MODE),
                b"default"
            );
            assert_eq!(retained_name, &[0xff, b'm']);
            let empty_mode = mode_from_name(b"", false);
            assert!((*empty_mode).name.is_empty());
            assert_eq!(
                (*(*empty_mode).bindings.as_ptr()).tqh_last,
                &raw mut (*(*empty_mode).bindings.as_ptr()).tqh_first
            );
            remove_first_mode();
            remove_first_mode();
        }
    }
}
