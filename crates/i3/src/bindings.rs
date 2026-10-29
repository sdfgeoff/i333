use crate::application::bindings;
use crate::commands_parser::parse_command;
use crate::config::modes;
use crate::ipc::ipc_send_binding_event;
use crate::tree::all_cons;
use i3_common::safewrappers::sasprintf;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * bindings.c: Functions for configuring, finding and, running bindings.
 */
// Mechanical file-by-file Rust translation of src/bindings.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use i3_json::compat::yajl_gen_t;
extern "C" {

    fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;

}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;

pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const FONT_TYPE_PANGO: C2RustUnnamed_1 = 2;
pub const FONT_TYPE_XCB: C2RustUnnamed_1 = 1;
pub const FONT_TYPE_NONE: C2RustUnnamed_1 = 0;
pub type i3Font = Font;

pub use crate::application::Binding;
pub use crate::application::Binding_Keycode;
pub type i3_event_state_mask_t = uint32_t;
pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const B_UPON_KEYRELEASE_IGNORE_MODS: C2RustUnnamed_4 = 2;
pub const B_UPON_KEYRELEASE: C2RustUnnamed_4 = 1;
pub const B_UPON_KEYPRESS: C2RustUnnamed_4 = 0;
pub type input_type_t = ::core::ffi::c_uint;
pub const B_MOUSE: input_type_t = 1;
pub const B_KEYBOARD: input_type_t = 0;

pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed_5 = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed_5 = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed_5 = 0;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_10 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_10 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_10 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_10 = 0;
pub type border_style_t = ::core::ffi::c_uint;
pub const BS_NORMAL: border_style_t = 2;
pub const BS_PIXEL: border_style_t = 1;
pub const BS_NONE: border_style_t = 0;
pub type layout_t = ::core::ffi::c_uint;
pub const L_SPLITH: layout_t = 6;
pub const L_SPLITV: layout_t = 5;
pub const L_OUTPUT: layout_t = 4;
pub const L_DOCKAREA: layout_t = 3;
pub const L_TABBED: layout_t = 2;
pub const L_STACKED: layout_t = 1;
pub const L_DEFAULT: layout_t = 0;
pub type fullscreen_mode_t = ::core::ffi::c_uint;
pub const CF_GLOBAL: fullscreen_mode_t = 2;
pub const CF_OUTPUT: fullscreen_mode_t = 1;
pub const CF_NONE: fullscreen_mode_t = 0;

pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_12 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_12 = 1;
pub const M_HERE: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_13 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_13 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_13 = 4;
pub const WM_TILING: C2RustUnnamed_13 = 3;
pub const WM_TILING_USER: C2RustUnnamed_13 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_13 = 1;
pub const WM_ANY: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_14 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_14 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_14 = 1;
pub const M_NODOCK: C2RustUnnamed_14 = 0;
pub const M_DONTCHECK: C2RustUnnamed_14 = -1;
pub type C2RustUnnamed_15 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_15 = 1;
pub const U_LATEST: C2RustUnnamed_15 = 0;
pub const U_DONTCHECK: C2RustUnnamed_15 = -1;

pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_16 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_16 = 1;
pub const W_NODOCK: C2RustUnnamed_16 = 0;

pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_19 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_19 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_19 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_19 = 2;
pub const A_COMMAND: C2RustUnnamed_19 = 1;
pub const A_ANY: C2RustUnnamed_19 = 0;

pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_21 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_21 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_21 = 3;
pub const CT_CON: C2RustUnnamed_21 = 2;
pub const CT_OUTPUT: C2RustUnnamed_21 = 1;
pub const CT_ROOT: C2RustUnnamed_21 = 0;
pub type smart_gaps_t = ::core::ffi::c_uint;
pub const SMART_GAPS_INVERSE_OUTER: smart_gaps_t = 2;
pub const SMART_GAPS_ON: smart_gaps_t = 1;
pub const SMART_GAPS_OFF: smart_gaps_t = 0;
pub type hide_edge_borders_mode_t = ::core::ffi::c_uint;
pub const HEBM_SMART_NO_GAPS: hide_edge_borders_mode_t = 64;
pub const HEBM_SMART: hide_edge_borders_mode_t = 32;
pub const HEBM_BOTH: hide_edge_borders_mode_t = 23;
pub const HEBM_HORIZONTAL: hide_edge_borders_mode_t = 20;
pub const HEBM_VERTICAL: hide_edge_borders_mode_t = 3;
pub const HEBM_NONE: hide_edge_borders_mode_t = 0;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const I3_XKB_GROUP_MASK_4: C2RustUnnamed_22 = 8;
pub const I3_XKB_GROUP_MASK_3: C2RustUnnamed_22 = 4;
pub const I3_XKB_GROUP_MASK_2: C2RustUnnamed_22 = 2;
pub const I3_XKB_GROUP_MASK_1: C2RustUnnamed_22 = 1;
pub const I3_XKB_GROUP_MASK_ANY: C2RustUnnamed_22 = 0;
pub type warping_t = ::core::ffi::c_uint;
pub const POINTER_WARPING_NONE: warping_t = 1;
pub const POINTER_WARPING_OUTPUT: warping_t = 0;
pub type focus_wrapping_t = ::core::ffi::c_uint;
pub const FOCUS_WRAPPING_WORKSPACE: focus_wrapping_t = 3;
pub const FOCUS_WRAPPING_FORCE: focus_wrapping_t = 2;
pub const FOCUS_WRAPPING_ON: focus_wrapping_t = 1;
pub const FOCUS_WRAPPING_OFF: focus_wrapping_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct all_cons_head {
    pub tqh_first: *mut Con,
    pub tqh_last: *mut *mut Con,
}
pub use crate::application::bindings_head;
pub type tiling_drag_t = ::core::ffi::c_uint;
pub const TILING_DRAG_MODIFIER_OR_TITLEBAR: tiling_drag_t = 3;
pub const TILING_DRAG_TITLEBAR: tiling_drag_t = 2;
pub const TILING_DRAG_MODIFIER: tiling_drag_t = 1;
pub const TILING_DRAG_OFF: tiling_drag_t = 0;

pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub const PDF_ALL: C2RustUnnamed_23 = 3;
pub const PDF_IGNORE: C2RustUnnamed_23 = 2;
pub const PDF_LEAVE_FULLSCREEN: C2RustUnnamed_23 = 1;
pub const PDF_SMART: C2RustUnnamed_23 = 0;

pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub const ALIGN_RIGHT: C2RustUnnamed_24 = 2;
pub const ALIGN_CENTER: C2RustUnnamed_24 = 1;
pub const ALIGN_LEFT: C2RustUnnamed_24 = 0;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub const FOWA_NONE: C2RustUnnamed_25 = 3;
pub const FOWA_FOCUS: C2RustUnnamed_25 = 2;
pub const FOWA_URGENT: C2RustUnnamed_25 = 1;
pub const FOWA_SMART: C2RustUnnamed_25 = 0;
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
    pub modes: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub sle_next: *mut Mode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub has_errors: bool,
    pub has_warnings: bool,
    pub line_number: ::core::ffi::c_int,
    pub line_copy: *mut ::core::ffi::c_char,
    pub filename: *const ::core::ffi::c_char,
    pub compact_error: *mut ::core::ffi::c_char,
    pub first_column: ::core::ffi::c_int,
    pub last_column: ::core::ffi::c_int,
}
pub use crate::application::ipc_client;
pub use crate::assignments::CommandResult;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct resolve {
    pub bind: *mut Binding,
    pub keysym: xkb_keysym_t,
    pub xkb_state: *mut xkb_state,
    pub xkb_state_no_shift: *mut xkb_state,
    pub xkb_state_numlock: *mut xkb_state,
    pub xkb_state_numlock_no_shift: *mut xkb_state,
}

pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 45] = unsafe {
    ::core::mem::transmute::<[u8; 45], [::core::ffi::c_char; 45]>(
        *b"void reorder_bindings_of_mode(struct Mode *)\0",
    )
};

pub const I3_IPC_EVENT_MASK: ::core::ffi::c_ulong =
    (1 as ::core::ffi::c_ulong) << 31 as ::core::ffi::c_int;
pub const I3_IPC_EVENT_MODE: ::core::ffi::c_ulong = I3_IPC_EVENT_MASK | 2 as ::core::ffi::c_ulong;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

static mut xkb_context: *mut xkb_context = ::core::ptr::null::<xkb_context>() as *mut xkb_context;
static mut xkb_keymap: *mut xkb_keymap = ::core::ptr::null::<xkb_keymap>() as *mut xkb_keymap;

pub static mut command_error_nagbar_pid: pid_t = -(1 as pid_t);

pub static mut DEFAULT_BINDING_MODE: *const ::core::ffi::c_char = c"default".as_ptr();
unsafe fn mode_from_name(name: *const ::core::ffi::c_char, pango_markup: bool) -> *mut Mode {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, name) == 0 as ::core::ffi::c_int {
            return mode;
        }
        mode = (*mode).modes.sle_next;
    }
    mode = scalloc(1 as size_t, ::core::mem::size_of::<Mode>() as size_t) as *mut Mode;
    (*mode).name = sstrdup(name);
    (*mode).pango_markup = pango_markup;
    (*mode).bindings = scalloc(
        1 as size_t,
        ::core::mem::size_of::<bindings_head>() as size_t,
    ) as *mut bindings_head;
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
        scalloc(1 as size_t, ::core::mem::size_of::<Binding>() as size_t) as *mut Binding;
    debuglog(
        c"%s:%s:%d - Binding %p bindtype %s, modifiers %s, input code %s, release %s\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"configure_binding".as_ptr(),
        64 as ::core::ffi::c_int,
        new_binding,
        bindtype,
        modifiers,
        input_code,
        release,
    );
    (*new_binding).release = (if !release.is_null() {
        B_UPON_KEYRELEASE as ::core::ffi::c_int
    } else {
        B_UPON_KEYPRESS as ::core::ffi::c_int
    }) as C2RustUnnamed_4;
    (*new_binding).border = !border.is_null();
    (*new_binding).whole_window = !whole_window.is_null();
    (*new_binding).exclude_titlebar = !exclude_titlebar.is_null();
    if strcmp(bindtype, c"bindsym".as_ptr()) == 0 as ::core::ffi::c_int {
        (*new_binding).input_type = (if strncasecmp(
            input_code,
            c"button".as_ptr(),
            (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as size_t)
                .wrapping_sub(1 as size_t),
        ) == 0 as ::core::ffi::c_int
        {
            B_MOUSE as ::core::ffi::c_int
        } else {
            B_KEYBOARD as ::core::ffi::c_int
        }) as input_type_t;
        (*new_binding).symbol = sstrdup(input_code);
    } else {
        let mut keycode: ::core::ffi::c_long = 0;
        if !parse_long(input_code, &raw mut keycode, 10 as ::core::ffi::c_int) {
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
        (*new_binding).keycode = keycode as uint32_t;
        (*new_binding).input_type = B_KEYBOARD;
    }
    (*new_binding).command = sstrdup(command);
    (*new_binding).event_state_mask = event_state_from_str(modifiers);
    let mut group_bits_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if (*new_binding).event_state_mask >> 16 as ::core::ffi::c_int
        & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
        != 0
    {
        group_bits_set += 1;
    }
    if group_bits_set > 1 as ::core::ffi::c_int {
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
unsafe fn binding_in_current_group(bind: *const Binding) -> bool {
    if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
        == I3_XKB_GROUP_MASK_ANY as ::core::ffi::c_int as i3_event_state_mask_t
    {
        return true_0 != 0;
    }
    match xkb_current_group {
        0 => {
            (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
        }
        1 => {
            (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
        }
        2 => {
            (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
        }
        3 => {
            (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
        }
        _ => {
            errorlog(
                c"ERROR: BUG: xkb_current_group (= %d) outside of [XCB_XKB_GROUP_1..XCB_XKB_GROUP_4]\n".as_ptr().cast_mut(),
                xkb_current_group,
            );
            false_0 != 0
        }
    }
}
unsafe fn grab_keycode_for_binding(
    conn_0: *mut xcb_connection_t,
    bind: *mut Binding,
    keycode: uint32_t,
) {
    let mods: ::core::ffi::c_int =
        ((*bind).event_state_mask & 0xffff as i3_event_state_mask_t) as ::core::ffi::c_int;
    debuglog(
        c"%s:%s:%d - Binding %p Grabbing keycode %d with event state mask 0x%x (mods 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"grab_keycode_for_binding".as_ptr(),
        141 as ::core::ffi::c_int,
        bind,
        keycode,
        (*bind).event_state_mask,
        mods,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        mods as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods as ::core::ffi::c_uint | xcb_numlock_mask) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods | XCB_MOD_MASK_LOCK as ::core::ffi::c_int) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
    xcb_grab_key(
        conn_0,
        0 as uint8_t,
        root,
        (mods as ::core::ffi::c_uint
            | xcb_numlock_mask
            | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as ::core::ffi::c_uint) as uint16_t,
        keycode as xcb_keycode_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
        XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
    );
}

pub unsafe fn grab_all_keys(conn_0: *mut xcb_connection_t) {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if !((*bind).input_type as ::core::ffi::c_uint
            != B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if binding_in_current_group(bind) {
                if (*bind).keycode > 0 as uint32_t {
                    grab_keycode_for_binding(conn_0, bind, (*bind).keycode);
                } else {
                    let mut binding_keycode: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode.is_null() {
                        let keycode: ::core::ffi::c_int =
                            (*binding_keycode).keycode as ::core::ffi::c_int;
                        let mods: ::core::ffi::c_int = ((*binding_keycode).modifiers
                            & 0xffff as i3_event_state_mask_t)
                            as ::core::ffi::c_int;
                        debuglog(
                            c"%s:%s:%d - Binding %p Grabbing keycode %d with mods %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"grab_all_keys".as_ptr(),
                            176 as ::core::ffi::c_int,
                            bind,
                            keycode,
                            mods,
                        );
                        xcb_grab_key(
                            conn_0,
                            0 as uint8_t,
                            root,
                            mods as uint16_t,
                            keycode as xcb_keycode_t,
                            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
                            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as uint8_t,
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
    let mut buttons: *mut ::core::ffi::c_int = bindings_get_buttons_to_grab();
    xcb_grab_server(conn_0);
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    con = all_cons.tqh_first as *mut Con;
    while !con.is_null() {
        if !(*con).window.is_null() {
            xcb_ungrab_button(
                conn_0,
                XCB_BUTTON_INDEX_ANY as ::core::ffi::c_int as uint8_t,
                (*(*con).window).id,
                XCB_BUTTON_MASK_ANY as ::core::ffi::c_int as uint16_t,
            );
            xcb_grab_buttons(conn_0, (*(*con).window).id, buttons);
        }
        con = (*con).all_cons.tqe_next as *mut Con;
    }
    free(buttons as *mut ::core::ffi::c_void);
    buttons = ::core::ptr::null_mut::<::core::ffi::c_int>();
    xcb_ungrab_server(conn_0);
}
unsafe fn get_binding(
    state_filtered: i3_event_state_mask_t,
    is_release: bool,
    input_code: uint16_t,
    input_type: input_type_t,
) -> *mut Binding {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut result: *mut Binding = ::core::ptr::null_mut::<Binding>();
    if !is_release {
        bind = (*bindings).tqh_first as *mut Binding;
        while !bind.is_null() {
            if !((*bind).input_type as ::core::ffi::c_uint != input_type as ::core::ffi::c_uint) {
                if (*bind).release as ::core::ffi::c_uint
                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*bind).release = B_UPON_KEYRELEASE;
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
    }
    let xkb_group_state: uint32_t = state_filtered as uint32_t & 0xffff0000 as uint32_t;
    let modifiers_state: uint32_t = state_filtered as uint32_t & 0xffff as uint32_t;
    let mut current_block_25: u64;
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if !((*bind).input_type as ::core::ffi::c_uint != input_type as ::core::ffi::c_uint) {
            let xkb_group_mask: uint32_t =
                (*bind).event_state_mask as uint32_t & 0xffff0000 as uint32_t;
            let groups_match: bool = xkb_group_state & xkb_group_mask == xkb_group_mask;
            if !groups_match {
                debuglog(
                    c"%s:%s:%d - skipping binding %p because XKB groups do not match\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/bindings.c".as_ptr(),
                    c"get_binding".as_ptr(),
                    237 as ::core::ffi::c_int,
                    bind,
                );
            } else {
                let mut found_keycode: bool = false_0 != 0;
                if input_type as ::core::ffi::c_uint
                    == B_KEYBOARD as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*bind).symbol.is_null()
                {
                    let input_keycode: xcb_keycode_t = input_code as xcb_keycode_t;
                    let mut binding_keycode: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode.is_null() {
                        let modifiers_mask: uint32_t =
                            (*binding_keycode).modifiers as uint32_t & 0xffff as uint32_t;
                        let mods_match: bool = modifiers_mask == modifiers_state;
                        debuglog(
                            c"%s:%s:%d - binding_keycode->modifiers = %d, modifiers_mask = %d, modifiers_state = %d, mods_match = %s\n".as_ptr().cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"get_binding".as_ptr(),
                            252 as ::core::ffi::c_int,
                            (*binding_keycode).modifiers,
                            modifiers_mask,
                            modifiers_state,
                            if mods_match as ::core::ffi::c_int != 0 {
                                c"yes".as_ptr()
                            } else {
                                c"no".as_ptr()
                            },
                        );
                        if (*binding_keycode).keycode as ::core::ffi::c_int
                            == input_keycode as ::core::ffi::c_int
                            && (mods_match as ::core::ffi::c_int != 0
                                || (*bind).release as ::core::ffi::c_uint
                                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                    && is_release as ::core::ffi::c_int != 0)
                        {
                            found_keycode = true_0 != 0;
                            break;
                        } else {
                            binding_keycode = (*binding_keycode).keycodes.tqe_next;
                        }
                    }
                    current_block_25 = 14136749492126903395;
                } else if (*bind).keycode != input_code as uint32_t {
                    current_block_25 = 2868539653012386629;
                } else {
                    let mut binding_keycode_0: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode_0 = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode_0.is_null() {
                        let modifiers_mask_0: uint32_t =
                            (*binding_keycode_0).modifiers as uint32_t & 0xffff as uint32_t;
                        let mods_match_0: bool = modifiers_mask_0 == modifiers_state;
                        debuglog(
                            c"%s:%s:%d - binding_keycode->modifiers = %d, modifiers_mask = %d, modifiers_state = %d, mods_match = %s\n".as_ptr().cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"get_binding".as_ptr(),
                            270 as ::core::ffi::c_int,
                            (*binding_keycode_0).modifiers,
                            modifiers_mask_0,
                            modifiers_state,
                            if mods_match_0 as ::core::ffi::c_int != 0 {
                                c"yes".as_ptr()
                            } else {
                                c"no".as_ptr()
                            },
                        );
                        if mods_match_0 as ::core::ffi::c_int != 0
                            || (*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                && is_release as ::core::ffi::c_int != 0
                        {
                            found_keycode = true_0 != 0;
                            break;
                        } else {
                            binding_keycode_0 = (*binding_keycode_0).keycodes.tqe_next;
                        }
                    }
                    current_block_25 = 14136749492126903395;
                }
                match current_block_25 {
                    2868539653012386629 => {}
                    _ => {
                        if found_keycode {
                            if (*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYRELEASE as ::core::ffi::c_int as ::core::ffi::c_uint
                                && !is_release
                            {
                                (*bind).release = B_UPON_KEYRELEASE_IGNORE_MODS;
                                debuglog(
                                    c"%s:%s:%d - marked bind %p as B_UPON_KEYRELEASE_IGNORE_MODS\n"
                                        .as_ptr()
                                        .cast_mut(),
                                    c"../src/bindings.c".as_ptr(),
                                    c"get_binding".as_ptr(),
                                    287 as ::core::ffi::c_int,
                                    bind,
                                );
                                if !result.is_null() {
                                    break;
                                }
                            } else if !((*bind).release as ::core::ffi::c_uint
                                == B_UPON_KEYPRESS as ::core::ffi::c_int as ::core::ffi::c_uint
                                && is_release as ::core::ffi::c_int != 0)
                            {
                                if is_release {
                                    return bind;
                                } else if result.is_null() {
                                    result = bind;
                                }
                            }
                        }
                    }
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
    result
}

pub unsafe fn get_binding_from_xcb_event(event: *mut xcb_generic_event_t) -> *mut Binding {
    let is_release: bool = (*event).response_type as ::core::ffi::c_int == XCB_KEY_RELEASE
        || (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_RELEASE;
    let input_type: input_type_t = (if (*event).response_type as ::core::ffi::c_int
        == XCB_BUTTON_RELEASE
        || (*event).response_type as ::core::ffi::c_int == XCB_BUTTON_PRESS
    {
        B_MOUSE as ::core::ffi::c_int
    } else {
        B_KEYBOARD as ::core::ffi::c_int
    }) as input_type_t;
    let event_state: uint16_t = (*(event as *mut xcb_key_press_event_t)).state;
    let event_detail: uint16_t = (*(event as *mut xcb_key_press_event_t)).detail as uint16_t;
    let mut state_filtered: i3_event_state_mask_t = (event_state as ::core::ffi::c_int
        & !(XCB_MOD_MASK_LOCK as ::core::ffi::c_int))
        as i3_event_state_mask_t;
    debuglog(
        c"%s:%s:%d - (removed capslock, state = 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"get_binding_from_xcb_event".as_ptr(),
        329 as ::core::ffi::c_int,
        state_filtered,
    );
    match (event_state as ::core::ffi::c_int & 0x6000 as ::core::ffi::c_int)
        >> 13 as ::core::ffi::c_int
    {
        0 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_1 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        1 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        2 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        3 => {
            state_filtered |= ((I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int)
                << 16 as ::core::ffi::c_int) as i3_event_state_mask_t;
        }
        _ => {}
    }
    state_filtered &= !(0x6000 as ::core::ffi::c_int) as i3_event_state_mask_t;
    debuglog(
        c"%s:%s:%d - (transformed keyboard group, state = 0x%x)\n"
            .as_ptr()
            .cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"get_binding_from_xcb_event".as_ptr(),
        354 as ::core::ffi::c_int,
        state_filtered,
    );
    get_binding(state_filtered, is_release, event_detail, input_type)
}
unsafe extern "C" fn add_keycode_if_matches(
    keymap: *mut xkb_keymap,
    key: xkb_keycode_t,
    data: *mut ::core::ffi::c_void,
) {
    let resolving: *const resolve = data as *const resolve;
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
    let binding_keycode: *mut Binding_Keycode =
        smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
    (*binding_keycode).modifiers = (*bind).event_state_mask;
    (*binding_keycode).keycode = key as xcb_keycode_t;
    (*binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
    (*binding_keycode).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
    *(*bind).keycodes_head.tqh_last = binding_keycode;
    (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode).keycodes.tqe_next;
    let binding_keycode_0: *mut Binding_Keycode =
        smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
    (*binding_keycode_0).modifiers =
        (*bind).event_state_mask | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
    (*binding_keycode_0).keycode = key as xcb_keycode_t;
    (*binding_keycode_0).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
    (*binding_keycode_0).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
    *(*bind).keycodes_head.tqh_last = binding_keycode_0;
    (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_0).keycodes.tqe_next;
    if (*bind).event_state_mask & xcb_numlock_mask as i3_event_state_mask_t
        != xcb_numlock_mask as i3_event_state_mask_t
    {
        let sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(numlock_state, key);
        if sym_numlock == (*resolving).keysym {
            let binding_keycode_1: *mut Binding_Keycode =
                smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                    as *mut Binding_Keycode;
            (*binding_keycode_1).modifiers =
                (*bind).event_state_mask | xcb_numlock_mask as i3_event_state_mask_t;
            (*binding_keycode_1).keycode = key as xcb_keycode_t;
            (*binding_keycode_1).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
            (*binding_keycode_1).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
            *(*bind).keycodes_head.tqh_last = binding_keycode_1;
            (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_1).keycodes.tqe_next;
            let binding_keycode_2: *mut Binding_Keycode =
                smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                    as *mut Binding_Keycode;
            (*binding_keycode_2).modifiers = (*bind).event_state_mask
                | xcb_numlock_mask as i3_event_state_mask_t
                | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
            (*binding_keycode_2).keycode = key as xcb_keycode_t;
            (*binding_keycode_2).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
            (*binding_keycode_2).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
            *(*bind).keycodes_head.tqh_last = binding_keycode_2;
            (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_2).keycodes.tqe_next;
        } else {
            debuglog(
                c"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n".as_ptr().cast_mut(),
                c"../src/bindings.c".as_ptr(),
                c"add_keycode_if_matches".as_ptr(),
                440 as ::core::ffi::c_int,
                key,
                sym_numlock,
            );
        }
    }
}

pub unsafe fn translate_keysyms() {
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    let mut dummy_state: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut dummy_state_numlock_no_shift: *mut xkb_state = ::core::ptr::null_mut::<xkb_state>();
    let mut has_errors: bool = false_0 != 0;
    dummy_state = xkb_state_new(xkb_keymap);
    if dummy_state.is_null()
        || {
            dummy_state_no_shift = xkb_state_new(xkb_keymap);
            dummy_state_no_shift.is_null()
        }
        || {
            dummy_state_numlock = xkb_state_new(xkb_keymap);
            dummy_state_numlock.is_null()
        }
        || {
            dummy_state_numlock_no_shift = xkb_state_new(xkb_keymap);
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
            if (*bind).input_type as ::core::ffi::c_uint
                == B_MOUSE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut button: ::core::ffi::c_long = 0;
                if !parse_long(
                    (*bind).symbol.offset(
                        (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                            .wrapping_sub(1_usize) as isize,
                    ),
                    &raw mut button,
                    10 as ::core::ffi::c_int,
                ) {
                    errorlog(
                        c"ERROR: Could not translate string to button: \"%s\"\n"
                            .as_ptr()
                            .cast_mut(),
                        (*bind).symbol,
                    );
                }
                let key: xcb_keycode_t = button as xcb_keycode_t;
                (*bind).keycode = key as uint32_t;
                debuglog(
                    c"%s:%s:%d - Binding Mouse button, Keycode = %d\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/bindings.c".as_ptr(),
                    c"translate_keysyms".as_ptr(),
                    474 as ::core::ffi::c_int,
                    key as ::core::ffi::c_int,
                );
            }
            let mut group: xkb_layout_index_t =
                XCB_XKB_GROUP_1 as ::core::ffi::c_int as xkb_layout_index_t;
            if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_2 as ::core::ffi::c_int as xkb_layout_index_t;
            } else if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_3 as ::core::ffi::c_int as xkb_layout_index_t;
            } else if (*bind).event_state_mask >> 16 as ::core::ffi::c_int
                & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
                != 0
            {
                group = XCB_XKB_GROUP_4 as ::core::ffi::c_int as xkb_layout_index_t;
            }
            debuglog(
                c"%s:%s:%d - Binding %p group = %d, event_state_mask = %d, &2 = %s, &3 = %s, &4 = %s\n".as_ptr().cast_mut(),
                c"../src/bindings.c".as_ptr(),
                c"translate_keysyms".as_ptr(),
                492 as ::core::ffi::c_int,
                bind,
                group,
                (*bind).event_state_mask,
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_2 as ::core::ffi::c_int as i3_event_state_mask_t
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_3 as ::core::ffi::c_int as i3_event_state_mask_t
                    != 0
                {
                    c"yes".as_ptr()
                } else {
                    c"no".as_ptr()
                },
                if (*bind).event_state_mask
                    & I3_XKB_GROUP_MASK_4 as ::core::ffi::c_int as i3_event_state_mask_t
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
                    ^ XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as xkb_mod_mask_t,
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
                    ^ XCB_KEY_BUT_MASK_SHIFT as ::core::ffi::c_int as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_mod_mask_t,
                0 as xkb_layout_index_t,
                0 as xkb_layout_index_t,
                group,
            );
            if (*bind).keycode > 0 as uint32_t {
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
                let binding_keycode: *mut Binding_Keycode =
                    smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                        as *mut Binding_Keycode;
                (*binding_keycode).modifiers = (*bind).event_state_mask;
                (*binding_keycode).keycode = (*bind).keycode as xcb_keycode_t;
                (*binding_keycode).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
                (*binding_keycode).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                *(*bind).keycodes_head.tqh_last = binding_keycode;
                (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode).keycodes.tqe_next;
                let binding_keycode_0: *mut Binding_Keycode =
                    smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                        as *mut Binding_Keycode;
                (*binding_keycode_0).modifiers = (*bind).event_state_mask
                    | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
                (*binding_keycode_0).keycode = (*bind).keycode as xcb_keycode_t;
                (*binding_keycode_0).keycodes.tqe_next = ::core::ptr::null_mut::<Binding_Keycode>();
                (*binding_keycode_0).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                *(*bind).keycodes_head.tqh_last = binding_keycode_0;
                (*bind).keycodes_head.tqh_last = &raw mut (*binding_keycode_0).keycodes.tqe_next;
                if (*bind).event_state_mask & xcb_numlock_mask as i3_event_state_mask_t
                    != xcb_numlock_mask as i3_event_state_mask_t
                {
                    let sym: xkb_keysym_t =
                        xkb_state_key_get_one_sym(dummy_state, (*bind).keycode as xkb_keycode_t);
                    let sym_numlock: xkb_keysym_t = xkb_state_key_get_one_sym(
                        dummy_state_numlock,
                        (*bind).keycode as xkb_keycode_t,
                    );
                    if sym == sym_numlock {
                        let binding_keycode_1: *mut Binding_Keycode =
                            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                                as *mut Binding_Keycode;
                        (*binding_keycode_1).modifiers =
                            (*bind).event_state_mask | xcb_numlock_mask as i3_event_state_mask_t;
                        (*binding_keycode_1).keycode = (*bind).keycode as xcb_keycode_t;
                        (*binding_keycode_1).keycodes.tqe_next =
                            ::core::ptr::null_mut::<Binding_Keycode>();
                        (*binding_keycode_1).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                        *(*bind).keycodes_head.tqh_last = binding_keycode_1;
                        (*bind).keycodes_head.tqh_last =
                            &raw mut (*binding_keycode_1).keycodes.tqe_next;
                        let binding_keycode_2: *mut Binding_Keycode =
                            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t)
                                as *mut Binding_Keycode;
                        (*binding_keycode_2).modifiers = (*bind).event_state_mask
                            | xcb_numlock_mask as i3_event_state_mask_t
                            | XCB_MOD_MASK_LOCK as ::core::ffi::c_int as i3_event_state_mask_t;
                        (*binding_keycode_2).keycode = (*bind).keycode as xcb_keycode_t;
                        (*binding_keycode_2).keycodes.tqe_next =
                            ::core::ptr::null_mut::<Binding_Keycode>();
                        (*binding_keycode_2).keycodes.tqe_prev = (*bind).keycodes_head.tqh_last;
                        *(*bind).keycodes_head.tqh_last = binding_keycode_2;
                        (*bind).keycodes_head.tqh_last =
                            &raw mut (*binding_keycode_2).keycodes.tqe_next;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Skipping automatic numlock fallback, key %d resolves to 0x%x with numlock\n".as_ptr().cast_mut(),
                            c"../src/bindings.c".as_ptr(),
                            c"translate_keysyms".as_ptr(),
                            560 as ::core::ffi::c_int,
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
                    let mut resolving: resolve = resolve {
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
                    let mut keycodes: *mut ::core::ffi::c_char = sstrdup(c"".as_ptr());
                    let mut num_keycodes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut binding_keycode_3: *mut Binding_Keycode =
                        ::core::ptr::null_mut::<Binding_Keycode>();
                    binding_keycode_3 = (*bind).keycodes_head.tqh_first;
                    while !binding_keycode_3.is_null() {
                        let mut tmp: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        sasprintf(
                            &raw mut tmp,
                            c"%s %d".as_ptr(),
                            keycodes,
                            (*binding_keycode_3).keycode as ::core::ffi::c_int,
                        );
                        free(keycodes as *mut ::core::ffi::c_void);
                        keycodes = tmp;
                        num_keycodes += 1;
                        let mut check: *mut Binding = ::core::ptr::null_mut::<Binding>();
                        check = (*bindings).tqh_first as *mut Binding;
                        while !check.is_null() {
                            if !(check == bind) {
                                if (*check).symbol.is_null() {
                                    if !((*check).keycode
                                        != (*binding_keycode_3).keycode as uint32_t
                                        || (*check).event_state_mask
                                            != (*binding_keycode_3).modifiers
                                        || (*check).release as ::core::ffi::c_uint
                                            != (*bind).release as ::core::ffi::c_uint)
                                    {
                                        has_errors = true_0 != 0;
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
                        618 as ::core::ffi::c_int,
                        (*bind).event_state_mask,
                        (*bind).symbol,
                        keysym,
                        keycodes,
                        num_keycodes,
                    );
                    free(keycodes as *mut ::core::ffi::c_void);
                }
            }
            bind = (*bind).bindings.tqe_next as *mut Binding;
        }
    }
    xkb_state_unref(dummy_state);
    xkb_state_unref(dummy_state_no_shift);
    xkb_state_unref(dummy_state_numlock);
    xkb_state_unref(dummy_state_numlock_no_shift);
    if has_errors {
        start_config_error_nagbar(current_configpath, true_0 != 0);
    }
}

pub unsafe fn switch_mode(new_mode: *const ::core::ffi::c_char) {
    let mut mode: *mut Mode = ::core::ptr::null_mut::<Mode>();
    debuglog(
        c"%s:%s:%d - Switching to mode %s\n".as_ptr().cast_mut(),
        c"../src/bindings.c".as_ptr(),
        c"switch_mode".as_ptr(),
        642 as ::core::ffi::c_int,
        new_mode,
    );
    mode = modes.slh_first as *mut Mode;
    while !mode.is_null() {
        if strcmp((*mode).name, new_mode) != 0 as ::core::ffi::c_int {
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
                if (*bind).release as ::core::ffi::c_uint
                    == B_UPON_KEYRELEASE_IGNORE_MODS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*bind).release = B_UPON_KEYRELEASE;
                }
                bind = (*bind).bindings.tqe_next as *mut Binding;
            }
            let mut event_msg: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut event_msg,
                c"{\"change\":\"%s\", \"pango_markup\":%s}".as_ptr(),
                (*mode).name,
                if (*mode).pango_markup as ::core::ffi::c_int != 0 {
                    c"true".as_ptr()
                } else {
                    c"false".as_ptr()
                },
            );
            ipc_send_event(c"mode".as_ptr(), I3_IPC_EVENT_MODE as uint32_t, event_msg);
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
) -> ::core::ffi::c_int {
    let first: *mut Binding = *(a as *mut *mut Binding);
    let second: *mut Binding = *(b as *mut *mut Binding);
    if (*first).event_state_mask < (*second).event_state_mask {
        return 1 as ::core::ffi::c_int;
    }
    if (*first).event_state_mask == (*second).event_state_mask {
        return 0 as ::core::ffi::c_int;
    }
    -(1 as ::core::ffi::c_int)
}
unsafe fn reorder_bindings_of_mode(mode: *mut Mode) {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current: *mut Binding = ::core::ptr::null_mut::<Binding>();
    current = (*(*mode).bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        n += 1;
        current = (*current).bindings.tqe_next as *mut Binding;
    }
    let tmp: *mut *mut Binding = scalloc(
        n as size_t,
        ::core::mem::size_of::<*mut Binding>() as size_t,
    ) as *mut *mut Binding;
    n = 0 as ::core::ffi::c_int;
    current = (*(*mode).bindings).tqh_first as *mut Binding;
    while !current.is_null() {
        let fresh5 = n;
        n = n + 1;
        let ref mut fresh6 = *tmp.offset(fresh5 as isize);
        *fresh6 = current;
        current = (*current).bindings.tqe_next as *mut Binding;
    }
    qsort(
        tmp as *mut ::core::ffi::c_void,
        n as size_t,
        ::core::mem::size_of::<*mut Binding>() as size_t,
        Some(
            reorder_binding_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
    let reordered: *mut bindings_head = scalloc(
        1 as size_t,
        ::core::mem::size_of::<bindings_head>() as size_t,
    ) as *mut bindings_head;
    (*reordered).tqh_first = ::core::ptr::null_mut::<Binding>();
    (*reordered).tqh_last = &raw mut (*reordered).tqh_first;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < n {
        current = *tmp.offset(i as isize);
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
    free(tmp as *mut ::core::ffi::c_void);
    if (*(*mode).bindings).tqh_first
        == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Binding
    {
    } else {
        __assert_fail(
            c"TAILQ_EMPTY(mode->bindings)".as_ptr(),
            c"../src/bindings.c".as_ptr(),
            713 as ::core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
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
    if (*a).input_type as ::core::ffi::c_uint != (*b).input_type as ::core::ffi::c_uint {
        return false_0 != 0;
    }
    if (*a).symbol.is_null() && !(*b).symbol.is_null()
        || !(*a).symbol.is_null() && (*b).symbol.is_null()
    {
        return false_0 != 0;
    }
    if !(*a).symbol.is_null() && strcasecmp((*a).symbol, (*b).symbol) != 0 as ::core::ffi::c_int {
        return false_0 != 0;
    }
    if (*a).keycode != (*b).keycode
        || (*a).event_state_mask != (*b).event_state_mask
        || (*a).release as ::core::ffi::c_uint != (*b).release as ::core::ffi::c_uint
    {
        return false_0 != 0;
    }
    true_0 != 0
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
                (*context).has_errors = true_0 != 0;
                if (*current).keycode != 0 as uint32_t {
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
    let ret: *mut Binding = smalloc(::core::mem::size_of::<Binding>() as size_t) as *mut Binding;
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
            smalloc(::core::mem::size_of::<Binding_Keycode>() as size_t) as *mut Binding_Keycode;
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
    let bind_cp: *mut Binding = binding_copy(bind);
    let mut modename: *mut ::core::ffi::c_char = sstrdup(current_binding_mode);
    let result: *mut CommandResult = parse_command(
        command,
        ::core::ptr::null_mut::<yajl_gen_t>(),
        ::core::ptr::null_mut::<ipc_client>(),
    );
    free(command as *mut ::core::ffi::c_void);
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
    free(modename as *mut ::core::ffi::c_void);
    modename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    binding_free(bind_cp);
    result
}
unsafe fn fill_rmlvo_from_root(xkb_names: *mut xkb_rule_names) -> ::core::ffi::c_int {
    let mut content_max_words: size_t = 256 as size_t;
    let atom_reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        xcb_intern_atom(
            conn,
            0 as uint8_t,
            strlen(c"_XKB_RULES_NAMES".as_ptr()) as uint16_t,
            c"_XKB_RULES_NAMES".as_ptr(),
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if atom_reply.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut prop_cookie: xcb_get_property_cookie_t = xcb_get_property_unchecked(
        conn,
        false_0 as uint8_t,
        root,
        (*atom_reply).atom,
        XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
        0 as uint32_t,
        content_max_words as uint32_t,
    );
    let mut prop_reply: *mut xcb_get_property_reply_t = xcb_get_property_reply(
        conn,
        prop_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if prop_reply.is_null() {
        free(atom_reply as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    if xcb_get_property_value_length(prop_reply) > 0 as ::core::ffi::c_int
        && (*prop_reply).bytes_after > 0 as uint32_t
    {
        content_max_words = (content_max_words as ::core::ffi::c_double
            + ceil((*prop_reply).bytes_after as ::core::ffi::c_double / 4.0f64))
            as size_t;
        free(prop_reply as *mut ::core::ffi::c_void);
        prop_cookie = xcb_get_property_unchecked(
            conn,
            false_0 as uint8_t,
            root,
            (*atom_reply).atom,
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as uint32_t,
            content_max_words as uint32_t,
        );
        prop_reply = xcb_get_property_reply(
            conn,
            prop_cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if prop_reply.is_null() {
            free(atom_reply as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int);
        }
    }
    if xcb_get_property_value_length(prop_reply) == 0 as ::core::ffi::c_int {
        free(atom_reply as *mut ::core::ffi::c_void);
        free(prop_reply as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    let mut walk: *const ::core::ffi::c_char =
        xcb_get_property_value(prop_reply) as *const ::core::ffi::c_char;
    let mut remaining: ::core::ffi::c_int = xcb_get_property_value_length(prop_reply);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 5 as ::core::ffi::c_int && remaining > 0 as ::core::ffi::c_int {
        let len: ::core::ffi::c_int = strnlen(walk, remaining as size_t) as ::core::ffi::c_int;
        match i {
            0 => {
                sasprintf(
                    &raw mut (*xkb_names).rules as *mut *mut ::core::ffi::c_char,
                    c"%.*s".as_ptr(),
                    len,
                    walk,
                );
            }
            1 => {
                sasprintf(
                    &raw mut (*xkb_names).model as *mut *mut ::core::ffi::c_char,
                    c"%.*s".as_ptr(),
                    len,
                    walk,
                );
            }
            2 => {
                sasprintf(
                    &raw mut (*xkb_names).layout as *mut *mut ::core::ffi::c_char,
                    c"%.*s".as_ptr(),
                    len,
                    walk,
                );
            }
            3 => {
                sasprintf(
                    &raw mut (*xkb_names).variant as *mut *mut ::core::ffi::c_char,
                    c"%.*s".as_ptr(),
                    len,
                    walk,
                );
            }
            4 => {
                sasprintf(
                    &raw mut (*xkb_names).options as *mut *mut ::core::ffi::c_char,
                    c"%.*s".as_ptr(),
                    len,
                    walk,
                );
            }
            _ => {}
        }
        debuglog(
            c"%s:%s:%d - component %d of _XKB_RULES_NAMES is \"%.*s\"\n"
                .as_ptr()
                .cast_mut(),
            c"../src/bindings.c".as_ptr(),
            c"fill_rmlvo_from_root".as_ptr(),
            966 as ::core::ffi::c_int,
            i,
            len,
            walk,
        );
        walk = walk.offset((len + 1 as ::core::ffi::c_int) as isize);
        remaining -= len + 1 as ::core::ffi::c_int;
        i += 1;
    }
    free(atom_reply as *mut ::core::ffi::c_void);
    free(prop_reply as *mut ::core::ffi::c_void);
    0 as ::core::ffi::c_int
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
            return false_0 != 0;
        }
    }
    let mut new_keymap: *mut xkb_keymap = ::core::ptr::null_mut::<xkb_keymap>();
    let mut device_id: int32_t = 0;
    if xkb_supported as ::core::ffi::c_int != 0 && {
        device_id = xkb_x11_get_core_keyboard_device_id(conn);
        device_id > -(1 as int32_t)
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
            return false_0 != 0;
        }
    } else {
        verboselog(
            c"No XKB / core keyboard device? Assembling keymap from local RMLVO.\n"
                .as_ptr()
                .cast_mut(),
        );
        let mut names: xkb_rule_names = xkb_rule_names {
            rules: ::core::ptr::null::<::core::ffi::c_char>(),
            model: ::core::ptr::null::<::core::ffi::c_char>(),
            layout: ::core::ptr::null::<::core::ffi::c_char>(),
            variant: ::core::ptr::null::<::core::ffi::c_char>(),
            options: ::core::ptr::null::<::core::ffi::c_char>(),
        };
        if fill_rmlvo_from_root(&raw mut names) == -(1 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: Could not get _XKB_RULES_NAMES atom from root window, falling back to defaults.\n".as_ptr().cast_mut(),
            );
        }
        new_keymap =
            xkb_keymap_new_from_names(xkb_context, &raw mut names, XKB_KEYMAP_COMPILE_NO_FLAGS);
        free(names.rules as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.model as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.layout as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.variant as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        free(names.options as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
        if new_keymap.is_null() {
            errorlog(
                c"ERROR: xkb_keymap_new_from_names failed\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return false_0 != 0;
        }
    }
    xkb_keymap_unref(xkb_keymap);
    xkb_keymap = new_keymap;
    true_0 != 0
}

pub unsafe fn bindings_get_buttons_to_grab() -> *mut ::core::ffi::c_int {
    let num_max: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
    let vla = num_max as usize;
    let mut buffer: Vec<::core::ffi::c_int> = ::std::vec::from_elem(0, vla);
    let mut num: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let fresh0 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh0 as isize) = 1 as ::core::ffi::c_int;
    let fresh1 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh1 as isize) = 2 as ::core::ffi::c_int;
    let fresh2 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh2 as isize) = 3 as ::core::ffi::c_int;
    let mut bind: *mut Binding = ::core::ptr::null_mut::<Binding>();
    bind = (*bindings).tqh_first as *mut Binding;
    while !bind.is_null() {
        if num + 1 as ::core::ffi::c_int == num_max {
            break;
        }
        if !((*bind).input_type as ::core::ffi::c_uint
            != B_MOUSE as ::core::ffi::c_int as ::core::ffi::c_uint
            || !(*bind).whole_window)
        {
            let mut button: ::core::ffi::c_long = 0;
            if !parse_long(
                (*bind).symbol.offset(
                    (::core::mem::size_of::<[::core::ffi::c_char; 7]>() as usize)
                        .wrapping_sub(1_usize) as isize,
                ),
                &raw mut button,
                10 as ::core::ffi::c_int,
            ) {
                errorlog(
                    c"ERROR: Could not parse button number, skipping this binding. Please report this bug in i3.\n".as_ptr().cast_mut(),
                );
            } else {
                let mut exists: bool = false_0 != 0;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < num {
                    if *buffer.as_mut_ptr().offset(i as isize) as ::core::ffi::c_long == button {
                        exists = true_0 != 0;
                        break;
                    } else {
                        i += 1;
                    }
                }
                if !exists {
                    let fresh3 = num;
                    num = num + 1;
                    *buffer.as_mut_ptr().offset(fresh3 as isize) = button as ::core::ffi::c_int;
                }
            }
        }
        bind = (*bind).bindings.tqe_next as *mut Binding;
    }
    let fresh4 = num;
    num = num + 1;
    *buffer.as_mut_ptr().offset(fresh4 as isize) = 0 as ::core::ffi::c_int;
    let buttons: *mut ::core::ffi::c_int = scalloc(
        num as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    memcpy(
        buttons as *mut ::core::ffi::c_void,
        buffer.as_mut_ptr() as *const ::core::ffi::c_void,
        (num as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    buttons
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

// Canonical Cairo backend ABI; renderer logic remains local.

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

pub use libc::{free, memcpy, qsort, strcasecmp, strcmp, strlen, strncasecmp, strnlen};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use i3_common::font::Font;
pub use i3_common::string::I3String as _i3String;

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
