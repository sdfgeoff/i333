use crate::application::bindings;
use crate::commands_parser::parse_command;
use crate::config::modes;
use crate::ipc::ipc_send_binding_event;
use crate::tree::all_cons;
use crate::types::BindingRelease;
use crate::types::InputType;
use i3_common::safewrappers::sasprintf;
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
pub use i3_json::compat::yajl_gen_t;

pub use crate::application::Binding;
pub use crate::application::Binding_Keycode;

pub const I3_XKB_GROUP_MASK_4: u32 = 8;
pub const I3_XKB_GROUP_MASK_3: u32 = 4;
pub const I3_XKB_GROUP_MASK_2: u32 = 2;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
pub use crate::application::bindings_head;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct modes_head {
    pub slh_first: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mode {
    pub name: *mut ::core::ffi::c_char,
    pub pango_markup: bool,
    pub bindings: *mut bindings_head,
    pub modes: ModeLinks,
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

static mut xkb_context: *mut xkb_context = ::core::ptr::null::<xkb_context>() as *mut xkb_context;
static mut xkb_keymap: *mut xkb_keymap = ::core::ptr::null::<xkb_keymap>() as *mut xkb_keymap;

pub static mut command_error_nagbar_pid: libc::pid_t = -(1 as libc::pid_t);

pub static mut DEFAULT_BINDING_MODE: *const ::core::ffi::c_char = c"default".as_ptr();
unsafe fn mode_from_name(name: *const ::core::ffi::c_char, pango_markup: bool) -> *mut Mode {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, name) == 0 {
            return mode;
        }
        mode = (*mode).modes.sle_next;
    }
    mode = scalloc(1 as usize, ::core::mem::size_of::<Mode>() as usize) as *mut Mode;
    (*mode).name = sstrdup(name);
    (*mode).pango_markup = pango_markup;
    (*mode).bindings =
        scalloc(1 as usize, ::core::mem::size_of::<bindings_head>() as usize) as *mut bindings_head;
    (*(*mode).bindings).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*(*mode).bindings).tqh_last = &raw mut (*(*mode).bindings).tqh_first;
    (*mode).modes.sle_next = modes.slh_first as *mut Mode;
    modes.slh_first = mode as *mut Mode;
    mode
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
    modename: *const ::core::ffi::c_char,
    pango_markup: bool,
) -> *mut Binding {
    let mut new_binding: *mut Binding =
        scalloc(1 as usize, ::core::mem::size_of::<Binding>() as usize) as *mut Binding;
    debuglog(
        c"%s:%s:%d - Binding %p bindtype %s, modifiers %s, input code %s, release %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"configure_binding".as_ptr(),
        64,
        new_binding,
        bindtype,
        modifiers,
        input_code,
        release,
    );
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
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize).wrapping_sub(1 as usize),
        ) == 0
        {
            InputType::Mouse
        } else {
            InputType::Keyboard
        };
        (*new_binding).symbol = sstrdup(input_code);
    } else {
        let mut keycode: ::core::ffi::c_long = 0;
        if !parse_long(input_code, &raw mut keycode, 10) {
            errorlog(
                c"ERROR: Could not parse \"%s\" as an input code, ignoring this binding.\n"
                    .as_ptr()
                    .cast_mut(),
                input_code,
            );
            free(new_binding as *mut ::core::ffi::c_void);
            new_binding = ::core::ptr::null_mut::<Binding>();
            return ::core::ptr::null_mut::<Binding>();
        }
        (*new_binding).keycode = keycode as u32;
        (*new_binding).input_type = InputType::Keyboard;
    }
    (*new_binding).command = sstrdup(command);
    (*new_binding).event_state_mask = event_state_from_str(modifiers);
    let group_bits_set = (((*new_binding).event_state_mask >> 16) & 15).count_ones();
    if group_bits_set > 1 {
        errorlog(
            c"ERROR: Keybinding has more than one Group specified, but your X server is always in precisely one group. The keybinding can never trigger.\n".as_ptr().cast_mut(),
        );
    }
    let mode: *const Mode = mode_from_name(modename, pango_markup);
    (*new_binding).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
    (*new_binding).bindings.tqe_prev = (*(*mode).bindings).tqh_last;
    *(*(*mode).bindings).tqh_last = new_binding as *mut Binding;
    (*(*mode).bindings).tqh_last = &raw mut (*new_binding).bindings.tqe_next;
    (*new_binding).keycodes_head.tqh_first = ::core::ptr::null_mut::<Binding_Keycode>();
    (*new_binding).keycodes_head.tqh_last = &raw mut (*new_binding).keycodes_head.tqh_first;
    new_binding
}
fn group_matches(mask: u32, group: u32) -> bool {
    let groups = mask >> 16;
    groups == 0 || group < 4 && groups & (1 << group) != 0
}
unsafe fn binding_in_current_group(bind: *const Binding) -> bool {
    if (*bind).event_state_mask >> 16 != 0 && xkb_current_group > 3 {
        errorlog(
            c"ERROR: BUG: xkb_current_group (= %d) outside of [XCB_XKB_GROUP_1..XCB_XKB_GROUP_4]\n"
                .as_ptr()
                .cast_mut(),
            xkb_current_group,
        );
    }
    group_matches((*bind).event_state_mask, xkb_current_group as u32)
}
unsafe fn grab_keycode_for_binding(
    conn_0: *mut xcb_connection_t,
    bind: *mut Binding,
    keycode: u32,
) {
    let mods: i32 = ((*bind).event_state_mask & 0xffff as u32) as i32;
    debuglog(
        c"%s:%s:%d - Binding %p Grabbing keycode %d with event state mask 0x%x (mods 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"grab_keycode_for_binding".as_ptr(),
        141,
        bind,
        keycode,
        (*bind).event_state_mask,
        mods,
    );
    for modifiers in grab_masks(mods as u32, xcb_numlock_mask) {
        xcb_grab_key(
            conn_0,
            0,
            root,
            modifiers,
            keycode as u8,
            XCB_GRAB_MODE_ASYNC as u8,
            XCB_GRAB_MODE_ASYNC as u8,
        );
    }
}

pub unsafe fn grab_all_keys(conn_0: *mut xcb_connection_t) {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if !((*bind).input_type != InputType::Keyboard) {
            if binding_in_current_group(bind) {
                if (*bind).keycode > 0 as u32 {
                    grab_keycode_for_binding(conn_0, bind, (*bind).keycode);
                } else {
                    let mut binding_keycode: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode.is_null() {
                        let keycode: i32 = (*binding_keycode).keycode as i32;
                        let mods: i32 = ((*binding_keycode).modifiers & 0xffff as u32) as i32;
                        debuglog(
                            c"%s:%s:%d - Binding %p Grabbing keycode %d with mods %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"grab_all_keys".as_ptr(),
                            176,
                            bind,
                            keycode,
                            mods,
                        );
                        xcb_grab_key(
                            conn_0,
                            0 as u8,
                            root,
                            mods as u16,
                            keycode as xcb_keycode_t,
                            XCB_GRAB_MODE_ASYNC as i32 as u8,
                            XCB_GRAB_MODE_ASYNC as i32 as u8,
                        );
                        binding_keycode = (*binding_keycode).keycodes.tqe_next;
                    }
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
}

pub unsafe fn regrab_all_buttons(conn_0: *mut xcb_connection_t) {
    let buttons = bindings_get_buttons_to_grab();
    let buttons_owner = Native::take(buttons);
    xcb_grab_server(conn_0);
    let server_grab = ServerGrab(conn_0);
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() {
            xcb_ungrab_button(
                conn_0,
                XCB_BUTTON_INDEX_ANY as i32 as u8,
                (*(*con).window).id,
                XCB_BUTTON_MASK_ANY as i32 as u16,
            );
            xcb_grab_buttons(conn_0, (*(*con).window).id, buttons);
        }
        con = (*con).all_cons.tqe_next as *mut Con;
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

pub unsafe fn get_binding_from_xcb_event(event: *mut xcb_generic_event_t) -> *mut Binding {
    let is_release: bool = (*event).response_type as i32 == XCB_KEY_RELEASE
        || (*event).response_type as i32 == XCB_BUTTON_RELEASE;
    let input_type: InputType = if (*event).response_type as i32 == XCB_BUTTON_RELEASE
        || (*event).response_type as i32 == XCB_BUTTON_PRESS
    {
        InputType::Mouse
    } else {
        InputType::Keyboard
    };
    let event_state: u16 = (*(event as *mut xcb_key_press_event_t)).state;
    let event_detail: u16 = (*(event as *mut xcb_key_press_event_t)).detail as u16;
    let mut state_filtered: u32 = (event_state as i32 & !(XCB_MOD_MASK_LOCK as i32)) as u32;
    debuglog(
        c"%s:%s:%d - (removed capslock, state = 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"get_binding_from_xcb_event".as_ptr(),
        329,
        state_filtered,
    );
    state_filtered = filtered_event_state(event_state);
    debuglog(
        c"%s:%s:%d - (transformed keyboard group, state = 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"get_binding_from_xcb_event".as_ptr(),
        354,
        state_filtered,
    );
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
    append_keycode(bind, key as xcb_keycode_t, (*bind).event_state_mask);
    append_keycode(
        bind,
        key as xcb_keycode_t,
        (*bind).event_state_mask | XCB_MOD_MASK_LOCK as i32 as u32,
    );
    if (*bind).event_state_mask & xcb_numlock_mask as u32 != xcb_numlock_mask as u32 {
        let sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(numlock_state, key);
        if sym_numlock == (*resolving).keysym {
            append_keycode(
                bind,
                key as xcb_keycode_t,
                (*bind).event_state_mask | xcb_numlock_mask as u32,
            );
            append_keycode(
                bind,
                key as xcb_keycode_t,
                (*bind).event_state_mask
                    | xcb_numlock_mask as u32
                    | XCB_MOD_MASK_LOCK as i32 as u32,
            );
        } else {
            debuglog(
                c"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n".as_ptr().cast_mut(),
                c"../src/bindings.c".as_ptr(),
                c"add_keycode_if_matches".as_ptr(),
                440,
                key,
                sym_numlock,
            );
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
        errorlog(
            c"ERROR: Could not create XKB state, cannot translate keysyms.\n"
                .as_ptr()
                .cast_mut(),
        );
    } else {
        bind = ::core::ptr::null_mut::<Binding>();
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if (*bind).input_type == InputType::Mouse {
                let mut button: ::core::ffi::c_long = 0;
                if !parse_long(
                    (*bind).symbol.offset(
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                            .wrapping_sub(1_usize) as isize,
                    ),
                    &raw mut button,
                    10,
                ) {
                    errorlog(
                        c"ERROR: Could not translate string to button: \"%s\"\n"
                            .as_ptr()
                            .cast_mut(),
                        (*bind).symbol,
                    );
                }
                let key: xcb_keycode_t = button as xcb_keycode_t;
                (*bind).keycode = key as u32;
                debuglog(
                    c"%s:%s:%d - Binding Mouse button, Keycode = %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/bindings.c".as_ptr(),
                    c"translate_keysyms".as_ptr(),
                    474,
                    key as i32,
                );
            }
            let group = translation_group((*bind).event_state_mask);
            debuglog(
                c"%s:%s:%d - Binding %p group = %d, event_state_mask = %d, &2 = %s, &3 = %s, &4 = %s\n".as_ptr().cast_mut(),
                c"../src/bindings.c".as_ptr(),
                c"translate_keysyms".as_ptr(),
                492,
                bind,
                group,
                (*bind).event_state_mask,
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_2 as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_3 as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_4 as i32 as u32
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                },
            );
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
                    ^ XCB_KEY_BUT_MASK_SHIFT as i32 as xkb_mod_mask_t,
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
                    ^ XCB_KEY_BUT_MASK_SHIFT as i32 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            if (*bind).keycode > 0 as u32 {
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
                append_keycode(
                    bind,
                    (*bind).keycode as xcb_keycode_t,
                    (*bind).event_state_mask,
                );
                append_keycode(
                    bind,
                    (*bind).keycode as xcb_keycode_t,
                    (*bind).event_state_mask | XCB_MOD_MASK_LOCK as i32 as u32,
                );
                if (*bind).event_state_mask & xcb_numlock_mask as u32 != xcb_numlock_mask as u32 {
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
                            (*bind).event_state_mask | xcb_numlock_mask | XCB_MOD_MASK_LOCK as u32,
                        );
                    } else {
                        debuglog(
                            c"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n".as_ptr().cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"translate_keysyms".as_ptr(),
                            560,
                            (*bind).keycode,
                            sym_numlock,
                        );
                    }
                }
            } else {
                let keysym: xkb_keysym_t =
                    xkb_keysym_from_name((*bind).symbol, XKB_KEYSYM_NO_FLAGS) as xkb_keysym_t;
                if keysym == XKB_KEY_NoSymbol as xkb_keysym_t {
                    errorlog(
                        c"ERROR: Could not translate string to key symbol: \"%s\"\n"
                            .as_ptr()
                            .cast_mut(),
                        (*bind).symbol,
                    );
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
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode_3 = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode_3.is_null() {
                        use std::fmt::Write;
                        write!(&mut keycodes, " {}", (*binding_keycode_3).keycode).unwrap();
                        num_keycodes += 1;
                        let mut check: *mut Binding = ::core::ptr::null_mut::<Binding>();
                        check = (*bindings).tqh_first as *mut Binding;
                        while !check.is_null() {
                            if !(check == bind) {
                                if (*check).symbol.is_null() {
                                    if !((*check).keycode != (*binding_keycode_3).keycode as u32
                                        || (*check).event_state_mask
                                            != (*binding_keycode_3).modifiers
                                        || (*check).release as u32 != (*bind).release as u32)
                                    {
                                        has_errors = true;
                                        errorlog(
                                            c"ERROR: Duplicate keybinding in config file:\n  keysym = %s, keycode = %d, state_mask = 0x%x\n".as_ptr().cast_mut(),
                                            (*bind).symbol,
                                            (*check).keycode,
                                            (*bind).event_state_mask,
                                        );
                                    }
                                }
                            }
                            check = (*check).bindings.tqe_next as *mut Binding;
                        }
                        binding_keycode_3 = (*binding_keycode_3).keycodes.tqe_next;
                    }
                    debuglog(
                        c"%s:%s:%d - state=0x%x, cfg=\"%s\", sym=0x%x → keycodes%s (%d)\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/bindings.c".as_ptr(),
                        c"translate_keysyms".as_ptr(),
                        618,
                        (*bind).event_state_mask,
                        (*bind).symbol,
                        keysym,
                        std::ffi::CString::new(keycodes).unwrap().as_ptr(),
                        num_keycodes,
                    );
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
    }

    drop(temporary_states);
    if has_errors {
        start_config_error_nagbar(current_configpath, true);
    }
}

pub unsafe fn switch_mode(new_mode: *const ::core::ffi::c_char) {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    debuglog(
        c"%s:%s:%d - Switching to mode %s\n".as_ptr().cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"switch_mode".as_ptr(),
        642,
        new_mode,
    );
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, new_mode) != 0 {
            mode = (*mode).modes.sle_next;
        } else {
            ungrab_all_keys(conn);
            bindings = (*mode).bindings as *mut bindings_head;
            current_binding_mode = (*mode).name;
            translate_keysyms();
            grab_all_keys(conn);
            regrab_all_buttons(conn);
            let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
            bind = (*bindings).tqh_first as *mut Binding;
            while !bind.is_null() {
                if (*bind).release == BindingRelease::KeyReleaseIgnoreMods {
                    (*bind).release = BindingRelease::KeyRelease;
                }
                bind = (*bind).bindings.tqe_next as *mut Binding;
            }
            let mut event_msg: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut event_msg,
                c"{\"change\":\"%s\", \"pango_markup\":%s}".as_ptr(),
                (*mode).name,
                if (*mode).pango_markup as i32 != 0 {
                    c"true".as_ptr()
                } else {
                    c"false".as_ptr()
                },
            );
            ipc_send_event(c"mode".as_ptr(), I3_IPC_EVENT_MODE as u32, event_msg);
            free(event_msg as *mut ::core::ffi::c_void);
            event_msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return;
        }
    }
    errorlog(c"ERROR: Mode not found\n".as_ptr().cast_mut());
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
    let mut current = (*(*mode).bindings).tqh_first;
    while !current.is_null() {
        tmp.push(current);
        current = (*current).bindings.tqe_next;
    }
    let n = tmp.len();
    qsort(
        tmp.as_mut_ptr().cast(),
        n as usize,
        ::core::mem::size_of::<*mut Binding>() as usize,
        Some(
            reorder_binding_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> i32,
        ),
    );
    let reordered: *mut bindings_head =
        scalloc(1 as usize, ::core::mem::size_of::<bindings_head>() as usize) as *mut bindings_head;
    (*reordered).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*reordered).tqh_last = &raw mut (*reordered).tqh_first;
    let mut i = 0usize;
    while i < n {
        current = tmp[i];
        if !(*current).bindings.tqe_next.is_null() {
            (*(*current).bindings.tqe_next).bindings.tqe_prev = (*current).bindings.tqe_prev;
        } else {
            (*(*mode).bindings).tqh_last = (*current).bindings.tqe_prev;
        }
        *(*current).bindings.tqe_prev = (*current).bindings.tqe_next;
        (*current).bindings.tqe_next = ::core::ptr::null_mut::<Binding>();
        (*current).bindings.tqe_prev = (*reordered).tqh_last;
        *(*reordered).tqh_last = current as *mut Binding;
        (*reordered).tqh_last = &raw mut (*current).bindings.tqe_next;
        i += 1;
    }

    if (*(*mode).bindings).tqh_first
        == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Binding
    {
    } else {
        __assert_fail(
            c"TAILQ_EMPTY(mode->bindings)".as_ptr(),
            c"../src/bindings.c".as_ptr(),
            713 as u32,
            c"void reorder_bindings_of_mode(struct Mode *)".as_ptr(),
        );
    };
    free((*mode).bindings as *mut ::core::ffi::c_void);
    (*mode).bindings = reordered;
}

pub unsafe fn reorder_bindings() {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        let current_mode: bool = (*mode).bindings == bindings;
        reorder_bindings_of_mode(mode);
        if current_mode {
            bindings = (*mode).bindings as *mut bindings_head;
        }
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
    let mut current: *mut Binding = ::core::ptr::null_mut::<Binding>();
    current = (*bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if bind == current {
                break;
            }
            if binding_same_key(bind, current) {
                (*context).has_errors = true;
                if (*current).keycode != 0 as u32 {
                    errorlog(
                        c"ERROR: Duplicate keybinding in config file:\n  state mask 0x%x with keycode %d, command \"%s\"\n".as_ptr().cast_mut(),
                        (*current).event_state_mask,
                        (*current).keycode,
                        (*current).command,
                    );
                } else {
                    errorlog(
                        c"ERROR: Duplicate keybinding in config file:\n  state mask 0x%x with keysym %s, command \"%s\"\n".as_ptr().cast_mut(),
                        (*current).event_state_mask,
                        (*current).symbol,
                        (*current).command,
                    );
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
        current = (*current).bindings.tqe_next as *mut Binding;
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
    let mut binding_keycode: *mut Binding_Keycode = ::core::ptr::null_mut::<Binding_Keycode>();
    binding_keycode = (*bind).keycodes_head.tqh_first;
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
    let mut command: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if con.is_null() {
        command = sstrdup((*bind).command);
    } else {
        sasprintf(
            &raw mut command,
            c"[con_id=\"%p\"] %s".as_ptr(),
            con,
            (*bind).command,
        );
    }
    let command_owner = Native::take(command);
    let bind_cp = binding_copy(bind);
    let binding_owner = BindingCopy(bind_cp);
    let mut modename: *mut ::core::ffi::c_char = sstrdup(current_binding_mode);
    let mode_owner = Native::take(modename);
    let result: *mut CommandResult = parse_command(
        command,
        ::core::ptr::null_mut::<yajl_gen_t>(),
        ::core::ptr::null_mut::<ipc_client>(),
    );
    drop(command_owner);
    if (*result).needs_tree_render {
        tree_render();
    }
    if (*result).parse_error {
        let mut pageraction: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut pageraction,
            c"i3-sensible-pager \"%s\"\n".as_ptr(),
            errorfilename,
        );
        let mut argv: [*mut ::core::ffi::c_char; 11] = [
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            c"-f".as_ptr().cast_mut(),
            config.font.pattern,
            c"-t".as_ptr().cast_mut(),
            c"error".as_ptr().cast_mut(),
            c"-m".as_ptr().cast_mut(),
            c"The configured command for this shortcut could not be run successfully."
                .as_ptr()
                .cast_mut(),
            c"-b".as_ptr().cast_mut(),
            c"show errors".as_ptr().cast_mut(),
            pageraction,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        start_nagbar(
            &raw mut command_error_nagbar_pid,
            &raw mut argv as *mut *mut ::core::ffi::c_char,
        );
        free(pageraction as *mut ::core::ffi::c_void);
    }
    ipc_send_binding_event(c"run".as_ptr(), bind_cp, modename);
    drop(mode_owner);
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
    let atom = Native::take(xcb_intern_atom_reply(
        conn,
        xcb_intern_atom(conn, 0, 16, c"_XKB_RULES_NAMES".as_ptr()),
        std::ptr::null_mut(),
    ))?;
    let mut words = 256u32;
    let query = |words| {
        xcb_get_property_reply(
            conn,
            xcb_get_property_unchecked(
                conn,
                0,
                root,
                (*atom.raw()).atom,
                XCB_GET_PROPERTY_TYPE_ANY as u32,
                0,
                words,
            ),
            std::ptr::null_mut(),
        )
    };
    let mut reply = Native::take(query(words))?;
    if xcb_get_property_value_length(reply.raw()) > 0 && (*reply.raw()).bytes_after > 0 {
        words = words.wrapping_add((*reply.raw()).bytes_after.div_ceil(4));
        reply = Native::take(query(words))?;
    }
    let header = &*reply.raw();
    let length = usize::try_from(xcb_get_property_value_length(reply.raw())).ok()?;
    if length == 0 || length > (header.length as usize).checked_mul(4)? {
        return None;
    }
    Some(rule_names(std::slice::from_raw_parts(
        xcb_get_property_value(reply.raw()).cast(),
        length,
    )))
}

pub unsafe fn load_keymap() -> bool {
    if xkb_context.is_null() {
        xkb_context = xkb_context_new(XKB_CONTEXT_NO_FLAGS);
        if xkb_context.is_null() {
            errorlog(
                c"ERROR: Could not create xkbcommon context\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return false;
        }
    }
    let mut new_keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
    let mut device_id: i32 = 0;
    if xkb_supported as i32 != 0 && {
        device_id = xkb_x11_get_core_keyboard_device_id(conn);
        device_id > -(1)
    } {
        new_keymap = xkb_x11_keymap_new_from_device(
            xkb_context,
            conn,
            device_id,
            XKB_KEYMAP_COMPILE_NO_FLAGS,
        );
        if new_keymap.is_null() {
            errorlog(
                c"ERROR: xkb_x11_keymap_new_from_device failed\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return false;
        }
    } else {
        verboselog(
            c"No XKB / core keyboard device? Assembling keymap from local RMLVO.\n"
                .as_ptr()
                .cast_mut(),
        );
        let owned_names = fill_rmlvo_from_root();
        if owned_names.is_none() {
            errorlog(c"ERROR: Could not get _XKB_RULES_NAMES atom from root window, falling back to defaults.\n".as_ptr().cast_mut());
        }
        let owned_names = owned_names.unwrap_or_default();
        let names = owned_names.native();
        new_keymap = xkb_keymap_new_from_names(xkb_context, &names, XKB_KEYMAP_COMPILE_NO_FLAGS);
        if new_keymap.is_null() {
            errorlog(
                c"ERROR: xkb_keymap_new_from_names failed\n"
                    .as_ptr()
                    .cast_mut(),
            );
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
            let mut button = 0;
            if parse_long((*bind).symbol.add(6), &raw mut button, 10) {
                append_button(&mut buttons, button);
            } else {
                errorlog(c"ERROR: Could not parse button number, skipping this binding. Please report this bug in i3.\n".as_ptr().cast_mut());
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

use xcb_sys::{
    xcb_atom_t, xcb_connection_t, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_property_cookie_t, xcb_get_property_reply, xcb_get_property_reply_t,
    xcb_get_property_unchecked, xcb_get_property_value, xcb_get_property_value_length,
    xcb_grab_key, xcb_grab_server, xcb_intern_atom, xcb_intern_atom_reply, xcb_intern_atom_reply_t,
    xcb_key_press_event_t, xcb_keycode_t, xcb_ungrab_button, xcb_ungrab_server,
    XCB_BUTTON_INDEX_ANY, XCB_BUTTON_MASK_ANY, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE,
    XCB_GET_PROPERTY_TYPE_ANY, XCB_GRAB_MODE_ASYNC, XCB_KEY_BUT_MASK_SHIFT, XCB_KEY_RELEASE,
    XCB_MOD_MASK_LOCK, XCB_XKB_GROUP_1, XCB_XKB_GROUP_2, XCB_XKB_GROUP_3, XCB_XKB_GROUP_4,
};

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

use crate::log::{debuglog, errorlog, verboselog};

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
#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn native_queue_lookup_preserves_press_release_pair_order_and_modifier_rearming() {
        use crate::application::{keycodes_head, C2RustUnnamed_13, C2RustUnnamed_14};
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
                bindings: C2RustUnnamed_13 {
                    tqe_next: std::ptr::null_mut(),
                    tqe_prev: std::ptr::null_mut(),
                },
            })
        }
        let mut key = Box::new(Binding_Keycode {
            keycode: 42,
            modifiers: 4,
            keycodes: C2RustUnnamed_14 {
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

/// Append an initialized native malloc node to an exclusively-owned valid queue.
unsafe fn append_keycode(bind: *mut Binding, keycode: u8, modifiers: u32) {
    let pointer = smalloc(std::mem::size_of::<Binding_Keycode>()).cast::<Binding_Keycode>();
    pointer.write(Binding_Keycode {
        keycode,
        modifiers,
        keycodes: crate::application::C2RustUnnamed_14 {
            tqe_next: std::ptr::null_mut(),
            tqe_prev: (*bind).keycodes_head.tqh_last,
        },
    });
    *(*bind).keycodes_head.tqh_last = pointer;
    (*bind).keycodes_head.tqh_last = &raw mut (*pointer).keycodes.tqe_next;
}

fn translation_group(mask: u32) -> u32 {
    let groups = mask >> 16;
    if groups & 2 != 0 {
        1
    } else if groups & 4 != 0 {
        2
    } else if groups & 8 != 0 {
        3
    } else {
        0
    }
}

struct BindingCopy(*mut Binding);
impl Drop for BindingCopy {
    fn drop(&mut self) {
        unsafe {
            binding_free(self.0);
        }
    }
}

struct ServerGrab(*mut xcb_connection_t);
impl Drop for ServerGrab {
    fn drop(&mut self) {
        unsafe {
            xcb_ungrab_server(self.0);
        }
    }
}
