use crate::tree::all_cons;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * A "match" is a data structure which acts like a mask or expression to match
 * certain windows or not. For example, when using commands, you can specify a
 * command like this: [title="*Firefox*"] kill. The title member of the match
 * data structure will then be filled and i3 will check each window using
 * match_matches_window() to find the windows affected by this command.
 *
 */
// Mechanical file-by-file Rust translation of src/match.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;

pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;

pub type i3String = _i3String;

pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCRATCHPAD_CHANGED: C2RustUnnamed = 2;
pub const SCRATCHPAD_FRESH: C2RustUnnamed = 1;
pub const SCRATCHPAD_NONE: C2RustUnnamed = 0;

pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
pub const FLOATING_USER_ON: C2RustUnnamed_4 = 3;
pub const FLOATING_AUTO_ON: C2RustUnnamed_4 = 2;
pub const FLOATING_USER_OFF: C2RustUnnamed_4 = 1;
pub const FLOATING_AUTO_OFF: C2RustUnnamed_4 = 0;
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

pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const M_BELOW: C2RustUnnamed_6 = 2;
pub const M_ASSIGN_WS: C2RustUnnamed_6 = 1;
pub const M_HERE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const WM_FLOATING: C2RustUnnamed_7 = 6;
pub const WM_FLOATING_USER: C2RustUnnamed_7 = 5;
pub const WM_FLOATING_AUTO: C2RustUnnamed_7 = 4;
pub const WM_TILING: C2RustUnnamed_7 = 3;
pub const WM_TILING_USER: C2RustUnnamed_7 = 2;
pub const WM_TILING_AUTO: C2RustUnnamed_7 = 1;
pub const WM_ANY: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const M_DOCK_BOTTOM: C2RustUnnamed_8 = 3;
pub const M_DOCK_TOP: C2RustUnnamed_8 = 2;
pub const M_DOCK_ANY: C2RustUnnamed_8 = 1;
pub const M_NODOCK: C2RustUnnamed_8 = 0;
pub const M_DONTCHECK: C2RustUnnamed_8 = -1;
pub type C2RustUnnamed_9 = ::core::ffi::c_int;
pub const U_OLDEST: C2RustUnnamed_9 = 1;
pub const U_LATEST: C2RustUnnamed_9 = 0;
pub const U_DONTCHECK: C2RustUnnamed_9 = -1;

pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const W_DOCK_BOTTOM: C2RustUnnamed_10 = 2;
pub const W_DOCK_TOP: C2RustUnnamed_10 = 1;
pub const W_NODOCK: C2RustUnnamed_10 = 0;

pub type C2RustUnnamed_13 = ::core::ffi::c_uint;
pub const A_TO_OUTPUT: C2RustUnnamed_13 = 16;
pub const A_TO_WORKSPACE_NUMBER: C2RustUnnamed_13 = 8;
pub const A_NO_FOCUS: C2RustUnnamed_13 = 4;
pub const A_TO_WORKSPACE: C2RustUnnamed_13 = 2;
pub const A_COMMAND: C2RustUnnamed_13 = 1;
pub const A_ANY: C2RustUnnamed_13 = 0;

pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const CT_DOCKAREA: C2RustUnnamed_15 = 5;
pub const CT_WORKSPACE: C2RustUnnamed_15 = 4;
pub const CT_FLOATING_CON: C2RustUnnamed_15 = 3;
pub const CT_CON: C2RustUnnamed_15 = 2;
pub const CT_OUTPUT: C2RustUnnamed_15 = 1;
pub const CT_ROOT: C2RustUnnamed_15 = 0;
pub type i3Window = Window;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;

pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub unsafe fn match_init(match_0: *mut Match) {
    memset(
        match_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<Match>() as size_t,
    );
    (*match_0).urgent = U_DONTCHECK;
    (*match_0).window_mode = WM_ANY;
    (*match_0).window_type = UINT32_MAX as xcb_atom_t;
}

pub unsafe fn match_is_empty(match_0: *mut Match) -> bool {
    (*match_0).title.is_null()
        && (*match_0).mark.is_null()
        && (*match_0).application.is_null()
        && (*match_0).class.is_null()
        && (*match_0).instance.is_null()
        && (*match_0).window_role.is_null()
        && (*match_0).workspace.is_null()
        && (*match_0).machine.is_null()
        && (*match_0).urgent as ::core::ffi::c_int == U_DONTCHECK as ::core::ffi::c_int
        && (*match_0).id as ::core::ffi::c_long == XCB_NONE
        && (*match_0).window_type == UINT32_MAX as xcb_atom_t
        && (*match_0).con_id.is_null()
        && (*match_0).dock as ::core::ffi::c_int == M_NODOCK as ::core::ffi::c_int
        && (*match_0).window_mode as ::core::ffi::c_uint
            == WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*match_0).match_all_windows as ::core::ffi::c_int == false_0
}

pub unsafe fn match_copy(dest: *mut Match, src: *mut Match) {
    memcpy(
        dest as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        ::core::mem::size_of::<Match>() as size_t,
    );
    if !(*src).title.is_null() {
        (*dest).title = regex_new((*(*src).title).pattern);
    }
    if !(*src).mark.is_null() {
        (*dest).mark = regex_new((*(*src).mark).pattern);
    }
    if !(*src).application.is_null() {
        (*dest).application = regex_new((*(*src).application).pattern);
    }
    if !(*src).class.is_null() {
        (*dest).class = regex_new((*(*src).class).pattern);
    }
    if !(*src).instance.is_null() {
        (*dest).instance = regex_new((*(*src).instance).pattern);
    }
    if !(*src).window_role.is_null() {
        (*dest).window_role = regex_new((*(*src).window_role).pattern);
    }
    if !(*src).workspace.is_null() {
        (*dest).workspace = regex_new((*(*src).workspace).pattern);
    }
    if !(*src).machine.is_null() {
        (*dest).machine = regex_new((*(*src).machine).pattern);
    }
}

pub unsafe fn match_matches_window(match_0: *mut Match, window: *mut i3Window) -> bool {
    verboselog(
        c"Checking window 0x%08x (class %s)\n".as_ptr().cast_mut(),
        (*window).id,
        (*window).class_class,
    );
    if !(*match_0).class.is_null() {
        let window_field_str: *const ::core::ffi::c_char = if (*window).class_class.is_null() {
            c"".as_ptr()
        } else {
            (*window).class_class as *const ::core::ffi::c_char
        };
        if strcmp((*(*match_0).class).pattern, c"__focused__".as_ptr()) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).class_class.is_null()
            && strcmp(window_field_str, (*(*focused).window).class_class) == 0 as ::core::ffi::c_int
        {
            verboselog(c"window class matches focused window\n".as_ptr().cast_mut());
        } else if regex_matches((*match_0).class, window_field_str) {
            verboselog(
                c"window class matches (%s)\n".as_ptr().cast_mut(),
                window_field_str,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).instance.is_null() {
        let window_field_str_0: *const ::core::ffi::c_char = if (*window).class_instance.is_null() {
            c"".as_ptr()
        } else {
            (*window).class_instance as *const ::core::ffi::c_char
        };
        if strcmp((*(*match_0).instance).pattern, c"__focused__".as_ptr())
            == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).class_instance.is_null()
            && strcmp(window_field_str_0, (*(*focused).window).class_instance)
                == 0 as ::core::ffi::c_int
        {
            verboselog(
                c"window instance matches focused window\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else if regex_matches((*match_0).instance, window_field_str_0) {
            verboselog(
                c"window instance matches (%s)\n".as_ptr().cast_mut(),
                window_field_str_0,
            );
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).id as ::core::ffi::c_long != XCB_NONE {
        if (*window).id == (*match_0).id {
            verboselog(
                c"match made by window id (%d)\n".as_ptr().cast_mut(),
                (*window).id,
            );
        } else {
            verboselog(c"window id does not match\n".as_ptr().cast_mut());
            return false_0 != 0;
        }
    }
    if !(*match_0).title.is_null() {
        let window_field_str_1: *const ::core::ffi::c_char = if (*window).name.is_null() {
            c"".as_ptr()
        } else {
            i3string_as_utf8((*window).name)
        };
        if strcmp((*(*match_0).title).pattern, c"__focused__".as_ptr()) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).name.is_null()
            && strcmp(
                window_field_str_1,
                i3string_as_utf8((*(*focused).window).name),
            ) == 0 as ::core::ffi::c_int
        {
            verboselog(c"window title matches focused window\n".as_ptr().cast_mut());
        } else if regex_matches((*match_0).title, window_field_str_1) {
            verboselog(
                c"window title matches (%s)\n".as_ptr().cast_mut(),
                window_field_str_1,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).window_role.is_null() {
        let window_field_str_2: *const ::core::ffi::c_char = if (*window).role.is_null() {
            c"".as_ptr()
        } else {
            (*window).role as *const ::core::ffi::c_char
        };
        if strcmp((*(*match_0).window_role).pattern, c"__focused__".as_ptr())
            == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).role.is_null()
            && strcmp(window_field_str_2, (*(*focused).window).role) == 0 as ::core::ffi::c_int
        {
            verboselog(
                c"window window_role matches focused window\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else if regex_matches((*match_0).window_role, window_field_str_2) {
            verboselog(
                c"window window_role matches (%s)\n".as_ptr().cast_mut(),
                window_field_str_2,
            );
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).window_type != UINT32_MAX as xcb_atom_t {
        if (*window).window_type == (*match_0).window_type {
            verboselog(
                c"window_type matches (%i)\n".as_ptr().cast_mut(),
                (*match_0).window_type,
            );
        } else {
            return false_0 != 0;
        }
    }
    if !(*match_0).machine.is_null() {
        let window_field_str_3: *const ::core::ffi::c_char = if (*window).machine.is_null() {
            c"".as_ptr()
        } else {
            (*window).machine as *const ::core::ffi::c_char
        };
        if strcmp((*(*match_0).machine).pattern, c"__focused__".as_ptr()) == 0 as ::core::ffi::c_int
            && !focused.is_null()
            && !(*focused).window.is_null()
            && !(*(*focused).window).machine.is_null()
            && strcmp(window_field_str_3, (*(*focused).window).machine) == 0 as ::core::ffi::c_int
        {
            verboselog(
                c"window machine matches focused window\n"
                    .as_ptr()
                    .cast_mut(),
            );
        } else if regex_matches((*match_0).machine, window_field_str_3) {
            verboselog(
                c"window machine matches (%s)\n".as_ptr().cast_mut(),
                window_field_str_3,
            );
        } else {
            return false_0 != 0;
        }
    }
    let mut con: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*match_0).urgent as ::core::ffi::c_int == U_LATEST as ::core::ffi::c_int {
        if (*window).urgent.tv_sec == 0 as __time_t {
            return false_0 != 0;
        }
        con = all_cons.tqh_first as *mut Con;
        while !con.is_null() {
            if !(*con).window.is_null()
                && (if (*(*con).window).urgent.tv_sec == (*window).urgent.tv_sec {
                    ((*(*con).window).urgent.tv_usec > (*window).urgent.tv_usec)
                        as ::core::ffi::c_int
                } else {
                    ((*(*con).window).urgent.tv_sec > (*window).urgent.tv_sec) as ::core::ffi::c_int
                }) != 0
            {
                return false_0 != 0;
            }
            con = (*con).all_cons.tqe_next as *mut Con;
        }
        verboselog(c"urgent matches latest\n".as_ptr().cast_mut());
    }
    if (*match_0).urgent as ::core::ffi::c_int == U_OLDEST as ::core::ffi::c_int {
        if (*window).urgent.tv_sec == 0 as __time_t {
            return false_0 != 0;
        }
        con = all_cons.tqh_first as *mut Con;
        while !con.is_null() {
            if !(*con).window.is_null()
                && (*(*con).window).urgent.tv_sec != 0 as __time_t
                && (if (*(*con).window).urgent.tv_sec == (*window).urgent.tv_sec {
                    ((*(*con).window).urgent.tv_usec < (*window).urgent.tv_usec)
                        as ::core::ffi::c_int
                } else {
                    ((*(*con).window).urgent.tv_sec < (*window).urgent.tv_sec) as ::core::ffi::c_int
                }) != 0
            {
                return false_0 != 0;
            }
            con = (*con).all_cons.tqe_next as *mut Con;
        }
        verboselog(c"urgent matches oldest\n".as_ptr().cast_mut());
    }
    if !(*match_0).workspace.is_null() {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        let ws: *mut Con = con_get_workspace(con);
        if ws.is_null() {
            return false_0 != 0;
        }
        if strcmp((*(*match_0).workspace).pattern, c"__focused__".as_ptr())
            == 0 as ::core::ffi::c_int
            && strcmp((*ws).name, (*con_get_workspace(focused)).name) == 0 as ::core::ffi::c_int
        {
            verboselog(c"workspace matches focused workspace\n".as_ptr().cast_mut());
        } else if regex_matches((*match_0).workspace, (*ws).name) {
            verboselog(c"workspace matches (%s)\n".as_ptr().cast_mut(), (*ws).name);
        } else {
            return false_0 != 0;
        }
    }
    if (*match_0).dock as ::core::ffi::c_int != M_DONTCHECK as ::core::ffi::c_int {
        if (*window).dock as ::core::ffi::c_uint
            == W_DOCK_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*match_0).dock as ::core::ffi::c_int == M_DOCK_TOP as ::core::ffi::c_int
            || (*window).dock as ::core::ffi::c_uint
                == W_DOCK_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*match_0).dock as ::core::ffi::c_int == M_DOCK_BOTTOM as ::core::ffi::c_int
            || ((*window).dock as ::core::ffi::c_uint
                == W_DOCK_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*window).dock as ::core::ffi::c_uint
                    == W_DOCK_BOTTOM as ::core::ffi::c_int as ::core::ffi::c_uint)
                && (*match_0).dock as ::core::ffi::c_int == M_DOCK_ANY as ::core::ffi::c_int
            || (*window).dock as ::core::ffi::c_uint
                == W_NODOCK as ::core::ffi::c_int as ::core::ffi::c_uint
                && (*match_0).dock as ::core::ffi::c_int == M_NODOCK as ::core::ffi::c_int
        {
            verboselog(c"dock status matches\n".as_ptr().cast_mut());
        } else {
            verboselog(c"dock status does not match\n".as_ptr().cast_mut());
            return false_0 != 0;
        }
    }
    if !(*match_0).mark.is_null() {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        let mut matched: bool = false_0 != 0;
        let mut mark: *mut mark_t = ::core::ptr::null_mut::<mark_t>();
        mark = (*con).marks_head.tqh_first as *mut mark_t;
        while !mark.is_null() {
            if regex_matches((*match_0).mark, (*mark).name) {
                matched = true_0 != 0;
                break;
            } else {
                mark = (*mark).marks.tqe_next as *mut mark_t;
            }
        }
        if matched {
            verboselog(c"mark matches\n".as_ptr().cast_mut());
        } else {
            verboselog(c"mark does not match\n".as_ptr().cast_mut());
            return false_0 != 0;
        }
    }
    if (*match_0).window_mode as ::core::ffi::c_uint
        != WM_ANY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        con = con_by_window_id((*window).id);
        if con.is_null() {
            return false_0 != 0;
        }
        match (*match_0).window_mode as ::core::ffi::c_uint {
            1 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_AUTO_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            2 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_USER_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            3 => {
                if !con_inside_floating(con).is_null() {
                    return false_0 != 0;
                }
            }
            4 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_AUTO_ON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            5 => {
                if (*con).floating as ::core::ffi::c_uint
                    != FLOATING_USER_ON as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    return false_0 != 0;
                }
            }
            6 => {
                if con_inside_floating(con).is_null() {
                    return false_0 != 0;
                }
            }
            0 => {
                if 0 as ::core::ffi::c_int != 0 {
                } else {
                    __assert_fail(
                        c"false".as_ptr(),
                        c"../src/match.c".as_ptr(),
                        262 as ::core::ffi::c_uint,
                        c"_Bool match_matches_window(Match *, i3Window *)".as_ptr(),
                    );
                };
            }
            _ => {}
        }
        verboselog(c"window_mode matches\n".as_ptr().cast_mut());
    }
    true_0 != 0
}

pub unsafe fn match_free(match_0: *mut Match) {
    free((*match_0).error as *mut ::core::ffi::c_void);
    (*match_0).error = ::core::ptr::null_mut::<::core::ffi::c_char>();
    regex_free((*match_0).title);
    regex_free((*match_0).application);
    regex_free((*match_0).class);
    regex_free((*match_0).instance);
    regex_free((*match_0).mark);
    regex_free((*match_0).window_role);
    regex_free((*match_0).workspace);
    regex_free((*match_0).machine);
}

pub unsafe fn match_parse_property(
    match_0: *mut Match,
    ctype: *const ::core::ffi::c_char,
    cvalue: *const ::core::ffi::c_char,
) {
    if match_0 != ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut Match {
    } else {
        __assert_fail(
            c"match != NULL".as_ptr(),
            c"../src/match.c".as_ptr(),
            296 as ::core::ffi::c_uint,
            c"void match_parse_property(Match *, const char *, const char *)".as_ptr(),
        );
    };
    debuglog(
        c"%s:%s:%d - ctype=*%s*, cvalue=*%s*\n".as_ptr().cast_mut(),
        c"../src/match.c".as_ptr(),
        c"match_parse_property".as_ptr(),
        297 as ::core::ffi::c_int,
        ctype,
        cvalue,
    );
    if strcmp(ctype, c"class".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).class);
        (*match_0).class = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"instance".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).instance);
        (*match_0).instance = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"window_role".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).window_role);
        (*match_0).window_role = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"con_id".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcmp(cvalue, c"__focused__".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).con_id = focused;
            return;
        }
        let mut parsed: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed, 0 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: Could not parse con id \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"invalid con_id".as_ptr());
        } else {
            (*match_0).con_id = parsed as *mut Con;
            debuglog(
                c"%s:%s:%d - id as int = %p\n".as_ptr().cast_mut(),
                c"../src/match.c".as_ptr(),
                c"match_parse_property".as_ptr(),
                329 as ::core::ffi::c_int,
                (*match_0).con_id,
            );
        }
        return;
    }
    if strcmp(ctype, c"id".as_ptr()) == 0 as ::core::ffi::c_int {
        let mut parsed_0: ::core::ffi::c_long = 0;
        if !parse_long(cvalue, &raw mut parsed_0, 0 as ::core::ffi::c_int) {
            errorlog(
                c"ERROR: Could not parse window id \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"invalid id".as_ptr());
        } else {
            (*match_0).id = parsed_0 as xcb_window_t;
            debuglog(
                c"%s:%s:%d - window id as int = %d\n".as_ptr().cast_mut(),
                c"../src/match.c".as_ptr(),
                c"match_parse_property".as_ptr(),
                341 as ::core::ffi::c_int,
                (*match_0).id,
            );
        }
        return;
    }
    if strcmp(ctype, c"window_type".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcasecmp(cvalue, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NORMAL;
        } else if strcasecmp(cvalue, c"dialog".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DIALOG;
        } else if strcasecmp(cvalue, c"utility".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_UTILITY;
        } else if strcasecmp(cvalue, c"toolbar".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLBAR;
        } else if strcasecmp(cvalue, c"splash".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_SPLASH;
        } else if strcasecmp(cvalue, c"menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_MENU;
        } else if strcasecmp(cvalue, c"dropdown_menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
        } else if strcasecmp(cvalue, c"popup_menu".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_POPUP_MENU;
        } else if strcasecmp(cvalue, c"tooltip".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_TOOLTIP;
        } else if strcasecmp(cvalue, c"notification".as_ptr()) == 0 as ::core::ffi::c_int {
            (*match_0).window_type = A__NET_WM_WINDOW_TYPE_NOTIFICATION;
        } else {
            errorlog(
                c"ERROR: unknown window_type value \"%s\"\n"
                    .as_ptr()
                    .cast_mut(),
                cvalue,
            );
            (*match_0).error = sstrdup(c"unknown window_type value".as_ptr());
        }
        return;
    }
    if strcmp(ctype, c"con_mark".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).mark);
        (*match_0).mark = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"title".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).title);
        (*match_0).title = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"urgent".as_ptr()) == 0 as ::core::ffi::c_int {
        if strcasecmp(cvalue, c"latest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"newest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"recent".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"last".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = U_LATEST;
        } else if strcasecmp(cvalue, c"oldest".as_ptr()) == 0 as ::core::ffi::c_int
            || strcasecmp(cvalue, c"first".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*match_0).urgent = U_OLDEST;
        }
        return;
    }
    if strcmp(ctype, c"workspace".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).workspace);
        (*match_0).workspace = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"machine".as_ptr()) == 0 as ::core::ffi::c_int {
        regex_free((*match_0).machine);
        (*match_0).machine = regex_new(cvalue);
        return;
    }
    if strcmp(ctype, c"tiling".as_ptr()) == 0 as ::core::ffi::c_int {
        (*match_0).window_mode = WM_TILING;
        return;
    }
    if strcmp(ctype, c"tiling_from".as_ptr()) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, c"auto".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_TILING_AUTO;
        return;
    }
    if strcmp(ctype, c"tiling_from".as_ptr()) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, c"user".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_TILING_USER;
        return;
    }
    if strcmp(ctype, c"floating".as_ptr()) == 0 as ::core::ffi::c_int {
        (*match_0).window_mode = WM_FLOATING;
        return;
    }
    if strcmp(ctype, c"floating_from".as_ptr()) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, c"auto".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_FLOATING_AUTO;
        return;
    }
    if strcmp(ctype, c"floating_from".as_ptr()) == 0 as ::core::ffi::c_int
        && !cvalue.is_null()
        && strcmp(cvalue, c"user".as_ptr()) == 0 as ::core::ffi::c_int
    {
        (*match_0).window_mode = WM_FLOATING_USER;
        return;
    }
    if strcmp(ctype, c"all".as_ptr()) == 0 as ::core::ffi::c_int {
        (*match_0).match_all_windows = true_0 != 0;
        return;
    }
    errorlog(c"ERROR: Unknown criterion: %s\n".as_ptr().cast_mut(), ctype);
}

use crate::regex::{regex_free, regex_matches, regex_new};

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::parse_long;
use i3_common::safewrappers::sstrdup;

// Canonical Cairo backend ABI; renderer logic remains local.

use xcb_sys::{xcb_atom_t, xcb_window_t, XCB_NONE};

pub use libc::{free, memcpy, memset, strcasecmp, strcmp};

use i3_common::native_stdio::__assert_fail;

use crate::log::{debuglog, errorlog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::mark_t;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Window;
pub use i3_common::string::I3String as _i3String;

// Internal calls and globals use canonical Rust identities.
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_NORMAL;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::con::con_by_window_id;
use crate::con::con_get_workspace;
use crate::con::con_inside_floating;
use crate::tree::focused;
use i3_common::string::i3string_as_utf8;
