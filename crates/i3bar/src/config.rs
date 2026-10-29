use i3_common::safewrappers::sasprintf;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_utf8_with_length;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * config.c: Parses the configuration (received from i3).
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/config.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_json::compat::{yajl_alloc, yajl_free, yajl_parse};
pub use i3_json::compat::{yajl_alloc_funcs, yajl_callbacks, yajl_handle, yajl_status};

pub type i3String = _i3String;

pub use crate::application::binding_t;
pub use crate::application::config_t;
pub use crate::application::tray_output_t;
pub use crate::application::Rect;

pub use crate::application::bindings_head;
pub use crate::application::tray_outputs_head;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut config: config_t = config_t {
    modifier: 0,
    bindings: bindings_head {
        tqh_first: ::core::ptr::null::<binding_t>() as *mut binding_t,
        tqh_last: ::core::ptr::null::<*mut binding_t>() as *mut *mut binding_t,
    },
    position: crate::application::BarPosition::None,
    verbose: false,
    bar_height: 0,
    padding: Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    },
    transparency: false,
    colors: xcb_color_strings_t {
        bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_bar_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_bar_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_sep_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        active_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        inactive_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        focus_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        urgent_ws_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_bg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_fg: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
        binding_mode_border: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    },
    disable_binding_mode_indicator: false,
    disable_ws: false,
    ws_min_width: 0,
    strip_ws_numbers: false,
    strip_ws_name: false,
    bar_id: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    workspace_command: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    fontname: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    separator_symbol: ::core::ptr::null::<i3String>() as *mut i3String,
    tray_outputs: tray_outputs_head {
        tqh_first: ::core::ptr::null::<tray_output_t>() as *mut tray_output_t,
        tqh_last: ::core::ptr::null::<*mut tray_output_t>() as *mut *mut tray_output_t,
    },
    tray_padding: 0,
    num_outputs: 0,
    outputs: ::core::ptr::null::<*mut ::core::ffi::c_char>() as *mut *mut ::core::ffi::c_char,
    hide_on_modifier: crate::application::BarDisplayMode::Dock,
    hidden_state: crate::application::HiddenState::Hide,
};
static mut cur_key: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut parsing_bindings: bool = false;
static mut parsing_tray_outputs: bool = false;
static mut parsing_padding: bool = false;
unsafe fn config_map_key_cb(
    params_: *mut ::core::ffi::c_void,
    keyVal: *const ::core::ffi::c_uchar,
    keyLen: usize,
) -> ::core::ffi::c_int {
    free(cur_key as *mut ::core::ffi::c_void);
    cur_key = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sasprintf(
        &raw mut cur_key,
        c"%.*s".as_ptr(),
        keyLen as ::core::ffi::c_int,
        keyVal,
    );
    if strcmp(cur_key, c"bindings".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_bindings = true_0 != 0;
    }
    if strcmp(cur_key, c"tray_outputs".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_tray_outputs = true_0 != 0;
    }
    if strcmp(cur_key, c"padding".as_ptr()) == 0 as ::core::ffi::c_int {
        parsing_padding = true_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn config_end_array_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    parsing_bindings = false_0 != 0;
    parsing_tray_outputs = false_0 != 0;
    parsing_padding = false_0 != 0;
    1 as ::core::ffi::c_int
}
unsafe fn config_null_cb(params_: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    if strcmp(cur_key, c"id".as_ptr()) == 0 {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: No such bar config. Use 'i3-msg -t get_bar_config' to get the available configs.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            63 as ::core::ffi::c_int,
        );
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Are you starting i3bar by hand? You should not:\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            64 as ::core::ffi::c_int,
        );
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Configure a 'bar' block in your i3 config and i3 will launch i3bar automatically.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            65 as ::core::ffi::c_int,
        );
        exit(libc::EXIT_FAILURE);
    }
    1 as ::core::ffi::c_int
}
unsafe fn config_string_cb(
    params_: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    mut _len: usize,
) -> ::core::ffi::c_int {
    let len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    if strcmp(cur_key, c"id".as_ptr()) == 0 || strcmp(cur_key, c"socket_path".as_ptr()) == 0 {
        return 1 as ::core::ffi::c_int;
    }
    if parsing_bindings {
        if strcmp(cur_key, c"command".as_ptr()) == 0 as ::core::ffi::c_int {
            let binding: *mut binding_t =
                *(*(config.bindings.tqh_last as *mut bindings_head)).tqh_last;
            if binding.is_null() {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: There is no binding to put the current command onto. This is a bug in i3.\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    87 as ::core::ffi::c_int,
                );
                return 0 as ::core::ffi::c_int;
            }
            if !(*binding).command.is_null() {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: The binding for input_code = %d already has a command. This is a bug in i3.\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    92 as ::core::ffi::c_int,
                    (*binding).input_code,
                );
                return 0 as ::core::ffi::c_int;
            }
            sasprintf(&raw mut (*binding).command, c"%.*s".as_ptr(), len, val);
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            100 as ::core::ffi::c_int,
            cur_key,
        );
        return 0 as ::core::ffi::c_int;
    }
    if parsing_tray_outputs {
        if config.verbose {
            printf(
                c"[%s:%d] Adding tray_output = %.*s to the list.\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                105 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let tray_output: *mut tray_output_t =
            scalloc(1 as usize, ::core::mem::size_of::<tray_output_t>() as usize)
                as *mut tray_output_t;
        sasprintf(&raw mut (*tray_output).output, c"%.*s".as_ptr(), len, val);
        (*tray_output).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
        (*tray_output).tray_outputs.tqe_prev = config.tray_outputs.tqh_last;
        *config.tray_outputs.tqh_last = tray_output as *mut tray_output_t;
        config.tray_outputs.tqh_last = &raw mut (*tray_output).tray_outputs.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"mode".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] mode = %.*s, len = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                113 as ::core::ffi::c_int,
                len,
                val,
                len,
            );
        }
        config.hide_on_modifier = if len as usize == strlen(c"dock".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"dock".as_ptr(),
                strlen(c"dock".as_ptr()),
            ) == 0
        {
            crate::application::BarDisplayMode::Dock
        } else {
            if len as usize == strlen(c"hide".as_ptr())
                && strncmp(
                    val as *const ::core::ffi::c_char,
                    c"hide".as_ptr(),
                    strlen(c"hide".as_ptr()),
                ) == 0
            {
                crate::application::BarDisplayMode::Hide
            } else {
                crate::application::BarDisplayMode::Invisible
            }
        };
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"hidden_state".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] hidden_state = %.*s, len = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                121 as ::core::ffi::c_int,
                len,
                val,
                len,
            );
        }
        config.hidden_state = if len as usize == strlen(c"hide".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"hide".as_ptr(),
                strlen(c"hide".as_ptr()),
            ) == 0
        {
            crate::application::HiddenState::Hide
        } else {
            crate::application::HiddenState::Show
        };
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"modifier".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] modifier = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                128 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        if len as usize == strlen(c"none".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"none".as_ptr(),
                strlen(c"none".as_ptr()),
            ) == 0
        {
            config.modifier = XCB_NONE as u32;
            return 1 as ::core::ffi::c_int;
        }
        if len as usize == strlen(c"shift".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"shift".as_ptr(),
                strlen(c"shift".as_ptr()),
            ) == 0
        {
            config.modifier = XCB_MOD_MASK_SHIFT as ::core::ffi::c_int as u32;
            return 1 as ::core::ffi::c_int;
        }
        if len as usize == strlen(c"ctrl".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"ctrl".as_ptr(),
                strlen(c"ctrl".as_ptr()),
            ) == 0
        {
            config.modifier = XCB_MOD_MASK_CONTROL as ::core::ffi::c_int as u32;
            return 1 as ::core::ffi::c_int;
        }
        if len as usize == strlen(c"Mod".as_ptr()).wrapping_add(1 as usize)
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"Mod".as_ptr(),
                strlen(c"Mod".as_ptr()),
            ) == 0
        {
            match *val.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
                49 => {
                    config.modifier = XCB_MOD_MASK_1 as ::core::ffi::c_int as u32;
                    return 1 as ::core::ffi::c_int;
                }
                50 => {
                    config.modifier = XCB_MOD_MASK_2 as ::core::ffi::c_int as u32;
                    return 1 as ::core::ffi::c_int;
                }
                51 => {
                    config.modifier = XCB_MOD_MASK_3 as ::core::ffi::c_int as u32;
                    return 1 as ::core::ffi::c_int;
                }
                53 => {
                    config.modifier = XCB_MOD_MASK_5 as ::core::ffi::c_int as u32;
                    return 1 as ::core::ffi::c_int;
                }
                _ => {}
            }
        }
        config.modifier = XCB_MOD_MASK_4 as ::core::ffi::c_int as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"wheel_up_cmd".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] wheel_up_cmd = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                166 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let binding_0: *mut binding_t =
            scalloc(1 as usize, ::core::mem::size_of::<binding_t>() as usize) as *mut binding_t;
        (*binding_0).input_code = 4 as ::core::ffi::c_int;
        sasprintf(&raw mut (*binding_0).command, c"%.*s".as_ptr(), len, val);
        (*binding_0).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
        (*binding_0).bindings.tqe_prev = config.bindings.tqh_last;
        *config.bindings.tqh_last = binding_0 as *mut binding_t;
        config.bindings.tqh_last = &raw mut (*binding_0).bindings.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"wheel_down_cmd".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] wheel_down_cmd = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                177 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let binding_1: *mut binding_t =
            scalloc(1 as usize, ::core::mem::size_of::<binding_t>() as usize) as *mut binding_t;
        (*binding_1).input_code = 5 as ::core::ffi::c_int;
        sasprintf(&raw mut (*binding_1).command, c"%.*s".as_ptr(), len, val);
        (*binding_1).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
        (*binding_1).bindings.tqe_prev = config.bindings.tqh_last;
        *config.bindings.tqh_last = binding_1 as *mut binding_t;
        config.bindings.tqh_last = &raw mut (*binding_1).bindings.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"position".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] position = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                186 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        config.position = if len as usize == strlen(c"top".as_ptr())
            && strncmp(
                val as *const ::core::ffi::c_char,
                c"top".as_ptr(),
                strlen(c"top".as_ptr()),
            ) == 0
        {
            crate::application::BarPosition::Top
        } else {
            crate::application::BarPosition::Bottom
        };
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"status_command".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] status_command = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                192 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(&raw mut config.command, c"%.*s".as_ptr(), len, val);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"workspace_command".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] workspace_command = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                198 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.workspace_command,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"font".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] font = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                204 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        free(config.fontname as *mut ::core::ffi::c_void);
        config.fontname = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(&raw mut config.fontname, c"%.*s".as_ptr(), len, val);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"separator_symbol".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] separator = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                211 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        i3string_free(config.separator_symbol);
        config.separator_symbol = ::core::ptr::null_mut::<i3String>();
        config.separator_symbol =
            i3string_from_utf8_with_length(val as *const ::core::ffi::c_char, len as isize);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"outputs".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] +output %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                218 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let new_num_outputs: ::core::ffi::c_int = config.num_outputs + 1 as ::core::ffi::c_int;
        config.outputs = srealloc(
            config.outputs as *mut ::core::ffi::c_void,
            (::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize)
                .wrapping_mul(new_num_outputs as usize),
        ) as *mut *mut ::core::ffi::c_char;
        sasprintf(
            config.outputs.offset(config.num_outputs as isize) as *mut *mut ::core::ffi::c_char,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        config.num_outputs = new_num_outputs;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"tray_output".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] Found deprecated key tray_output %.*s.\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                229 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        let tray_output_0: *mut tray_output_t =
            scalloc(1 as usize, ::core::mem::size_of::<tray_output_t>() as usize)
                as *mut tray_output_t;
        sasprintf(&raw mut (*tray_output_0).output, c"%.*s".as_ptr(), len, val);
        (*tray_output_0).tray_outputs.tqe_next = ::core::ptr::null_mut::<tray_output_t>();
        (*tray_output_0).tray_outputs.tqe_prev = config.tray_outputs.tqh_last;
        *config.tray_outputs.tqh_last = tray_output_0 as *mut tray_output_t;
        config.tray_outputs.tqh_last = &raw mut (*tray_output_0).tray_outputs.tqe_next;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"statusline".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] statusline = bar_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                245 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(&raw mut config.colors.bar_fg, c"%.*s".as_ptr(), len, val);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"background".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] background = bar_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                246 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(&raw mut config.colors.bar_bg, c"%.*s".as_ptr(), len, val);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"separator".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] separator = sep_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                247 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(&raw mut config.colors.sep_fg, c"%.*s".as_ptr(), len, val);
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_statusline".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_statusline = focus_bar_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                248 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_bar_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_background".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_background = focus_bar_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                249 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_bar_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_separator".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_separator = focus_sep_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                250 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_sep_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_workspace_border".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_workspace_border = focus_ws_border = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                251 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_border,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_workspace_bg".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_workspace_bg = focus_ws_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                252 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"focused_workspace_text".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] focused_workspace_text = focus_ws_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                253 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.focus_ws_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"active_workspace_border".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] active_workspace_border = active_ws_border = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                254 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_border,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"active_workspace_bg".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] active_workspace_bg = active_ws_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                255 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"active_workspace_text".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] active_workspace_text = active_ws_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                256 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.active_ws_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"inactive_workspace_border".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] inactive_workspace_border = inactive_ws_border = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                257 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_border,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"inactive_workspace_bg".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] inactive_workspace_bg = inactive_ws_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                258 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"inactive_workspace_text".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] inactive_workspace_text = inactive_ws_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                259 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.inactive_ws_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"urgent_workspace_border".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] urgent_workspace_border = urgent_ws_border = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                260 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_border,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"urgent_workspace_bg".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] urgent_workspace_bg = urgent_ws_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                261 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"urgent_workspace_text".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] urgent_workspace_text = urgent_ws_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                262 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.urgent_ws_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"binding_mode_border".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] binding_mode_border = binding_mode_border = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                263 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_border,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"binding_mode_bg".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] binding_mode_bg = binding_mode_bg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                264 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_bg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"binding_mode_text".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] binding_mode_text = binding_mode_fg = %.*s\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                265 as ::core::ffi::c_int,
                len,
                val,
            );
        }
        sasprintf(
            &raw mut config.colors.binding_mode_fg,
            c"%.*s".as_ptr(),
            len,
            val,
        );
        return 1 as ::core::ffi::c_int;
    }
    printf(
        c"got unexpected string %.*s for cur_key = %s\n".as_ptr(),
        len,
        val,
        cur_key,
    );
    0 as ::core::ffi::c_int
}
unsafe fn config_boolean_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if parsing_bindings {
        if strcmp(cur_key, c"release".as_ptr()) == 0 as ::core::ffi::c_int {
            let binding: *mut binding_t =
                *(*(config.bindings.tqh_last as *mut bindings_head)).tqh_last;
            if binding.is_null() {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: There is no binding to put the current command onto. This is a bug in i3.\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    281 as ::core::ffi::c_int,
                );
                return 0 as ::core::ffi::c_int;
            }
            (*binding).release = val != 0;
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            289 as ::core::ffi::c_int,
            cur_key,
        );
    }
    if strcmp(cur_key, c"binding_mode_indicator".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] binding_mode_indicator = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                293 as ::core::ffi::c_int,
                val,
            );
        }
        config.disable_binding_mode_indicator = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"workspace_buttons".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] workspace_buttons = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                299 as ::core::ffi::c_int,
                val,
            );
        }
        config.disable_ws = val == 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"strip_workspace_numbers".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] strip_workspace_numbers = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                305 as ::core::ffi::c_int,
                val,
            );
        }
        config.strip_ws_numbers = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"strip_workspace_name".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] strip_workspace_name = %d\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                311 as ::core::ffi::c_int,
                val,
            );
        }
        config.strip_ws_name = val != 0;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"verbose".as_ptr()) == 0 {
        if !config.verbose {
            if config.verbose {
                printf(
                    c"[%s:%d] verbose = %d\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    318 as ::core::ffi::c_int,
                    val,
                );
            }
            config.verbose = val != 0;
        }
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
unsafe fn config_integer_cb(
    params_: *mut ::core::ffi::c_void,
    val: ::core::ffi::c_longlong,
) -> ::core::ffi::c_int {
    if parsing_bindings {
        if strcmp(cur_key, c"input_code".as_ptr()) == 0 as ::core::ffi::c_int {
            let binding: *mut binding_t =
                scalloc(1 as usize, ::core::mem::size_of::<binding_t>() as usize) as *mut binding_t;
            (*binding).input_code = val as ::core::ffi::c_int;
            (*binding).bindings.tqe_next = ::core::ptr::null_mut::<binding_t>();
            (*binding).bindings.tqe_prev = config.bindings.tqh_last;
            *config.bindings.tqh_last = binding as *mut binding_t;
            config.bindings.tqh_last = &raw mut (*binding).bindings.tqe_next;
            return 1 as ::core::ffi::c_int;
        }
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Unknown key \"%s\" while parsing bar bindings.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            341 as ::core::ffi::c_int,
            cur_key,
        );
        return 0 as ::core::ffi::c_int;
    }
    if parsing_padding {
        if strcmp(cur_key, c"x".as_ptr()) == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] padding.x = %lld\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    347 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.x = val as u32;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(cur_key, c"y".as_ptr()) == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] padding.y = %lld\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    352 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.y = val as u32;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(cur_key, c"width".as_ptr()) == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] padding.width = %lld\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    357 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.width = val as u32;
            return 1 as ::core::ffi::c_int;
        }
        if strcmp(cur_key, c"height".as_ptr()) == 0 as ::core::ffi::c_int {
            if config.verbose {
                printf(
                    c"[%s:%d] padding.height = %lld\n".as_ptr(),
                    c"../i3bar/src/config.c".as_ptr(),
                    362 as ::core::ffi::c_int,
                    val,
                );
            }
            config.padding.height = val as u32;
            return 1 as ::core::ffi::c_int;
        }
    }
    if strcmp(cur_key, c"bar_height".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] bar_height = %lld\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                369 as ::core::ffi::c_int,
                val,
            );
        }
        config.bar_height = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"tray_padding".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] tray_padding = %lld\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                375 as ::core::ffi::c_int,
                val,
            );
        }
        config.tray_padding = val as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"modifier".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] modifier = %lld\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                381 as ::core::ffi::c_int,
                val,
            );
        }
        config.modifier = val as u32;
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(cur_key, c"workspace_min_width".as_ptr()) == 0 {
        if config.verbose {
            printf(
                c"[%s:%d] workspace_min_width = %lld\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                387 as ::core::ffi::c_int,
                val,
            );
        }
        config.ws_min_width = val as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    0 as ::core::ffi::c_int
}
static mut outputs_callbacks: yajl_callbacks = unsafe {
    yajl_callbacks {
        yajl_null: Some(
            config_null_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        yajl_boolean: Some(
            config_boolean_cb
                as unsafe fn(*mut ::core::ffi::c_void, ::core::ffi::c_int) -> ::core::ffi::c_int,
        ),
        yajl_integer: Some(
            config_integer_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_longlong,
                ) -> ::core::ffi::c_int,
        ),
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            config_string_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: Some(
            config_map_key_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_end_map: None,
        yajl_start_array: None,
        yajl_end_array: Some(
            config_end_array_cb as unsafe fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
    }
};

pub unsafe fn parse_config_json(json: *const ::core::ffi::c_uchar, size: usize) {
    config.bindings.tqh_first = ::core::ptr::null_mut::<binding_t>();
    config.bindings.tqh_last = &raw mut config.bindings.tqh_first;
    config.tray_outputs.tqh_first = ::core::ptr::null_mut::<tray_output_t>();
    config.tray_outputs.tqh_last = &raw mut config.tray_outputs.tqh_first;
    let handle: yajl_handle = yajl_alloc(
        &raw mut outputs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    let state: yajl_status = yajl_parse(handle, json, size);
    match state as ::core::ffi::c_uint {
        1 | 2 => {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not parse config reply!\n".as_ptr(),
                c"../i3bar/src/config.c".as_ptr(),
                422 as ::core::ffi::c_int,
            );
            exit(libc::EXIT_FAILURE);
        }
        0 | _ => {}
    }
    if config.disable_ws as ::core::ffi::c_int != 0 && !config.workspace_command.is_null() {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: You have specified 'workspace_buttons no'. Your 'workspace_command %s' will be ignored.\n".as_ptr(),
            c"../i3bar/src/config.c".as_ptr(),
            428 as ::core::ffi::c_int,
            config.workspace_command,
        );
        free(config.workspace_command as *mut ::core::ffi::c_void);
        config.workspace_command = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    yajl_free(handle);
}
unsafe fn i3bar_config_string_cb(
    params_: *mut ::core::ffi::c_void,
    val: *const ::core::ffi::c_uchar,
    mut _len: usize,
) -> ::core::ffi::c_int {
    sasprintf(
        &raw mut config.bar_id,
        c"%.*s".as_ptr(),
        _len as ::core::ffi::c_int,
        val,
    );
    0 as ::core::ffi::c_int
}

pub unsafe fn parse_get_first_i3bar_config(json: *const ::core::ffi::c_uchar, size: usize) {
    let mut configs_callbacks: yajl_callbacks = yajl_callbacks {
        yajl_null: None,
        yajl_boolean: None,
        yajl_integer: None,
        yajl_double: None,
        yajl_number: None,
        yajl_string: Some(
            i3bar_config_string_cb
                as unsafe fn(
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_uchar,
                    usize,
                ) -> ::core::ffi::c_int,
        ),
        yajl_start_map: None,
        yajl_map_key: None,
        yajl_end_map: None,
        yajl_start_array: None,
        yajl_end_array: None,
    };
    let handle: yajl_handle = yajl_alloc(
        &raw mut configs_callbacks,
        ::core::ptr::null_mut::<yajl_alloc_funcs>(),
        NULL,
    );
    yajl_parse(handle, json, size);
    yajl_free(handle);
}

pub unsafe fn free_colors(colors: *mut xcb_color_strings_t) {
    free((*colors).bar_fg as *mut ::core::ffi::c_void);
    (*colors).bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).bar_bg as *mut ::core::ffi::c_void);
    (*colors).bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).sep_fg as *mut ::core::ffi::c_void);
    (*colors).sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_fg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_bar_bg as *mut ::core::ffi::c_void);
    (*colors).focus_bar_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_sep_fg as *mut ::core::ffi::c_void);
    (*colors).focus_sep_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_fg as *mut ::core::ffi::c_void);
    (*colors).active_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_bg as *mut ::core::ffi::c_void);
    (*colors).active_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).active_ws_border as *mut ::core::ffi::c_void);
    (*colors).active_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_fg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_bg as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).inactive_ws_border as *mut ::core::ffi::c_void);
    (*colors).inactive_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_fg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_bg as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).urgent_ws_border as *mut ::core::ffi::c_void);
    (*colors).urgent_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_fg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_bg as *mut ::core::ffi::c_void);
    (*colors).focus_ws_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).focus_ws_border as *mut ::core::ffi::c_void);
    (*colors).focus_ws_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_fg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_fg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_bg as *mut ::core::ffi::c_void);
    (*colors).binding_mode_bg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*colors).binding_mode_border as *mut ::core::ffi::c_void);
    (*colors).binding_mode_border = ::core::ptr::null_mut::<::core::ffi::c_char>();
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::srealloc;

use xcb_sys::{
    xcb_color_strings_t, XCB_MOD_MASK_1, XCB_MOD_MASK_2, XCB_MOD_MASK_3, XCB_MOD_MASK_4,
    XCB_MOD_MASK_5, XCB_MOD_MASK_CONTROL, XCB_MOD_MASK_SHIFT, XCB_NONE,
};

pub use libc::{exit, fprintf, free, printf, strcmp, strlen, strncmp};

use i3_common::native_stdio::stderr;

pub use i3_common::string::I3String as _i3String;
