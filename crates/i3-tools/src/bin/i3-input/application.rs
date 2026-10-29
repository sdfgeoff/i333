use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::string::i3string_free;
use i3_common::string::i3string_from_ucs2;
use i3_common::string::i3string_from_utf8;
use i3_common::ucs2_conversion::convert_ucs2_to_utf8;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * i3-input/main.c: Utility which lets the user input commands and sends them
 *                  to i3.
 *
 */
// Mechanical file-by-file Rust translation of i3-input/main.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
use i3_common::i333_va_list;

pub type i3String = _i3String;
pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;

pub type i3Font = Font;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;
pub const XK_BackSpace: ::core::ffi::c_int = 0xff08 as ::core::ffi::c_int;
pub const XK_Return: ::core::ffi::c_int = 0xff0d as ::core::ffi::c_int;
pub const XK_Escape: ::core::ffi::c_int = 0xff1b as ::core::ffi::c_int;
pub const XK_Mode_switch: ::core::ffi::c_int = 0xff7e as ::core::ffi::c_int;

pub static mut EXIT_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub static mut EXIT_CANCEL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

pub static mut EXIT_ERROR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut format: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut sockfd: ::core::ffi::c_int = 0;
static mut symbols: *mut xcb_key_symbols_t =
    ::core::ptr::null::<xcb_key_symbols_t>() as *mut xcb_key_symbols_t;
static mut modeswitch_active: bool = false_0 != 0;
static mut win: xcb_window_t = 0;
static mut surface: surface_t = surface_t {
    id: 0,
    gc: 0,
    owns_gc: false,
    width: 0,
    height: 0,
    surface: ::core::ptr::null::<cairo_surface_t>() as *mut cairo_surface_t,
    cr: ::core::ptr::null::<cairo_t>() as *mut cairo_t,
};
static mut glyphs_ucs: [xcb_char2b_t; 512] = [xcb_char2b_t { byte1: 0, byte2: 0 }; 512];
static mut glyphs_utf8: [*mut ::core::ffi::c_char; 512] =
    [::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char; 512];
static mut input_position: ::core::ffi::c_int = 0;
static mut font: i3Font = Font {
    type_0: i3_common::font::FontType::None,
    height: 0,
    pattern: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    specific: C2RustUnnamed {
        xcb: C2RustUnnamed_0 {
            id: 0,
            info: ::core::ptr::null::<xcb_query_font_reply_t>() as *mut xcb_query_font_reply_t,
            table: ::core::ptr::null::<xcb_charinfo_t>() as *mut xcb_charinfo_t,
        },
    },
};
static mut prompt: *mut i3String = ::core::ptr::null::<i3String>() as *mut i3String;
static mut prompt_offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut limit: ::core::ffi::c_int = 0;

pub static mut root: xcb_window_t = 0;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;

pub unsafe extern "C" fn verboselog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stdout, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn errorlog(fmt: *mut ::core::ffi::c_char, args: ...) {
    let mut args_0;
    args_0 = args.clone();
    vfprintf(stderr, fmt, i333_va_list!(args_0));
}

pub unsafe extern "C" fn debuglog(fmt: *mut ::core::ffi::c_char, args: ...) {}
unsafe fn concat_strings(
    glyphs: *mut *mut ::core::ffi::c_char,
    max: ::core::ffi::c_int,
) -> *mut u8 {
    let output: *mut u8 = scalloc((max + 1 as ::core::ffi::c_int) as usize, 4 as usize) as *mut u8;
    let mut walk: *mut u8 = output;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while c < max {
        printf(
            c"at %c\n".as_ptr(),
            *(*glyphs.offset(c as isize)).offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int,
        );
        if *(*glyphs.offset(c as isize)).offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            == '\0' as i32
        {
            memcpy(
                walk as *mut ::core::ffi::c_void,
                *glyphs.offset(c as isize) as *const ::core::ffi::c_void,
                2 as usize,
            );
            walk = walk.offset(2 as ::core::ffi::c_int as isize);
        } else {
            strcpy(walk as *mut ::core::ffi::c_char, *glyphs.offset(c as isize));
            walk = walk.offset(strlen(*glyphs.offset(c as isize)) as isize);
        }
        c += 1;
    }
    printf(
        c"output = %s\n".as_ptr(),
        output as *mut ::core::ffi::c_char,
    );
    output
}
unsafe fn handle_expose(conn_0: *mut xcb_connection_t) -> ::core::ffi::c_int {
    printf(c"expose!\n".as_ptr());
    let border_color: color_t = draw_util_hex_to_color(c"#FF0000".as_ptr());
    let fg_color: color_t = draw_util_hex_to_color(c"#FFFFFF".as_ptr());
    let bg_color: color_t = draw_util_hex_to_color(c"#000000".as_ptr());
    let text_offset: ::core::ffi::c_int =
        logical_px(2 as ::core::ffi::c_int) + logical_px(2 as ::core::ffi::c_int);
    draw_util_rectangle(
        &raw mut surface,
        border_color,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        0 as ::core::ffi::c_int as ::core::ffi::c_double,
        surface.width as ::core::ffi::c_double,
        surface.height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        &raw mut surface,
        bg_color,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        logical_px(2 as ::core::ffi::c_int) as ::core::ffi::c_double,
        (surface.width - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (surface.height - 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    if !prompt.is_null() {
        draw_util_text(
            prompt,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset,
            text_offset,
            logical_px(500 as ::core::ffi::c_int) - text_offset,
        );
    }
    if input_position > 0 as ::core::ffi::c_int {
        let input: *mut i3String = i3string_from_ucs2(
            &raw mut glyphs_ucs as *mut xcb_char2b_t,
            input_position as usize,
        );
        draw_util_text(
            input,
            &raw mut surface,
            fg_color,
            bg_color,
            text_offset + prompt_offset,
            text_offset,
            logical_px(500 as ::core::ffi::c_int) - text_offset,
        );
        i3string_free(input);
    }
    xcb_flush(conn_0);
    1 as ::core::ffi::c_int
}
unsafe fn handle_key_release(
    ignored: *mut ::core::ffi::c_void,
    conn_0: *mut xcb_connection_t,
    event: *mut xcb_key_release_event_t,
) -> ::core::ffi::c_int {
    printf(
        c"releasing %d, state raw = %d\n".as_ptr(),
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let sym: xcb_keysym_t = xcb_key_press_lookup_keysym(
        symbols,
        event as *mut xcb_key_press_event_t,
        (*event).state as ::core::ffi::c_int,
    );
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(c"Mode switch disabled\n".as_ptr());
        modeswitch_active = false_0 != 0;
    }
    1 as ::core::ffi::c_int
}
unsafe fn finish_input() {
    let command: *mut ::core::ffi::c_char = concat_strings(
        &raw mut glyphs_utf8 as *mut *mut ::core::ffi::c_char,
        input_position,
    ) as *mut ::core::ffi::c_char;
    let len: usize = strlen(format) as usize;
    let mut cnt: usize = 0 as usize;
    let mut c: usize = 0 as usize;
    while c < len.wrapping_sub(1 as usize) {
        if *format.offset(c as isize) as ::core::ffi::c_int == '%' as i32
            && *format.offset(c.wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                == 's' as i32
        {
            cnt = cnt.wrapping_add(1);
        }
        c = c.wrapping_add(1);
    }
    printf(c"occurrences = %zu\n".as_ptr(), cnt);
    let input_len: usize = strlen(command) as usize;
    let full_len: usize = if input_len
        > strlen(format)
            .wrapping_sub((2 as usize).wrapping_mul(cnt))
            .wrapping_add(input_len.wrapping_mul(cnt))
            .wrapping_add(1 as usize)
    {
        input_len
    } else {
        (strlen(format) as usize)
            .wrapping_sub((2 as usize).wrapping_mul(cnt))
            .wrapping_add(input_len.wrapping_mul(cnt))
            .wrapping_add(1 as usize)
    };
    let full: *mut ::core::ffi::c_char = scalloc(full_len, 1 as usize) as *mut ::core::ffi::c_char;
    let mut dest: *mut ::core::ffi::c_char = full;
    let mut c_0: usize = 0 as usize;
    while c_0 < len {
        if *format.offset(c_0 as isize) as ::core::ffi::c_int != '%' as i32
            || c_0 == len.wrapping_sub(1 as usize)
            || *format.offset(c_0.wrapping_add(1 as usize) as isize) as ::core::ffi::c_int
                != 's' as i32
        {
            let fresh0 = dest;
            dest = dest.offset(1);
            *fresh0 = *format.offset(c_0 as isize);
        } else {
            strncat(dest, command, input_len);
            dest = dest.offset(input_len as isize);
            c_0 = c_0.wrapping_add(1);
        }
        c_0 = c_0.wrapping_add(1);
    }
    printf(c"command = %s\n".as_ptr(), full);
    let ret: ::core::ffi::c_int =
        ipc_send_message(sockfd, strlen(full) as u32, 0 as u32, full as *mut u8);
    free(full as *mut ::core::ffi::c_void);
    exit(if ret == 0 as ::core::ffi::c_int {
        EXIT_OK
    } else {
        EXIT_ERROR
    });
}
unsafe fn handle_key_press(
    ignored: *mut ::core::ffi::c_void,
    conn_0: *mut xcb_connection_t,
    event: *mut xcb_key_press_event_t,
) -> ::core::ffi::c_int {
    printf(
        c"Keypress %d, state raw = %d\n".as_ptr(),
        (*event).detail as ::core::ffi::c_int,
        (*event).state as ::core::ffi::c_int,
    );
    let mut col: ::core::ffi::c_int =
        (*event).state as ::core::ffi::c_int & XCB_MOD_MASK_SHIFT as ::core::ffi::c_int;
    if modeswitch_active {
        col += 2 as ::core::ffi::c_int;
    }
    let sym: xcb_keysym_t = xcb_key_press_lookup_keysym(symbols, event, col);
    if sym == XK_Mode_switch as xcb_keysym_t {
        printf(c"Mode switch enabled\n".as_ptr());
        modeswitch_active = true_0 != 0;
        return 1 as ::core::ffi::c_int;
    }
    if sym == XK_Return as xcb_keysym_t {
        finish_input();
    }
    if sym == XK_BackSpace as xcb_keysym_t {
        if input_position == 0 as ::core::ffi::c_int {
            return 1 as ::core::ffi::c_int;
        }
        input_position -= 1;
        free(glyphs_utf8[input_position as usize] as *mut ::core::ffi::c_void);
        handle_expose(conn_0);
        return 1 as ::core::ffi::c_int;
    }
    if sym == XK_Escape as xcb_keysym_t {
        exit(EXIT_CANCEL);
    }
    printf(c"is_keypad_key = %d\n".as_ptr(), xcb_is_keypad_key(sym));
    printf(
        c"is_private_keypad_key = %d\n".as_ptr(),
        xcb_is_private_keypad_key(sym),
    );
    printf(c"xcb_is_cursor_key = %d\n".as_ptr(), xcb_is_cursor_key(sym));
    printf(c"xcb_is_pf_key = %d\n".as_ptr(), xcb_is_pf_key(sym));
    printf(
        c"xcb_is_function_key = %d\n".as_ptr(),
        xcb_is_function_key(sym),
    );
    printf(
        c"xcb_is_misc_function_key = %d\n".as_ptr(),
        xcb_is_misc_function_key(sym),
    );
    printf(
        c"xcb_is_modifier_key = %d\n".as_ptr(),
        xcb_is_modifier_key(sym),
    );
    if xcb_is_modifier_key(sym) != 0 || xcb_is_cursor_key(sym) != 0 {
        return 1 as ::core::ffi::c_int;
    }
    printf(c"sym = %c (%d)\n".as_ptr(), sym, sym);
    let ucs: u16 = super::keysym2ucs::keysym2ucs(sym)
        .map(|code| code as u16)
        .unwrap_or(u16::MAX);
    if ucs as i16 as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            c"Keysym could not be converted to UCS, skipping\n".as_ptr(),
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut inp: xcb_char2b_t = xcb_char2b_t { byte1: 0, byte2: 0 };
    inp.byte1 = ((ucs as ::core::ffi::c_int & 0xff00 as ::core::ffi::c_int)
        >> 2 as ::core::ffi::c_int) as u8;
    inp.byte2 =
        ((ucs as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) >> 0 as ::core::ffi::c_int) as u8;
    printf(
        c"inp.byte1 = %02x, inp.byte2 = %02x\n".as_ptr(),
        inp.byte1 as ::core::ffi::c_int,
        inp.byte2 as ::core::ffi::c_int,
    );
    let out: *mut ::core::ffi::c_char = convert_ucs2_to_utf8(&raw mut inp, 1 as usize);
    printf(c"converted to %s\n".as_ptr(), out);
    glyphs_ucs[input_position as usize] = inp;
    glyphs_utf8[input_position as usize] = out;
    input_position += 1;
    if input_position == limit {
        finish_input();
    }
    handle_expose(conn_0);
    1 as ::core::ffi::c_int
}
unsafe fn get_window_position() -> xcb_rectangle_t {
    let mut result: xcb_rectangle_t = xcb_rectangle_t {
        x: logical_px(50 as ::core::ffi::c_int) as i16,
        y: logical_px(50 as ::core::ffi::c_int) as i16,
        width: logical_px(500 as ::core::ffi::c_int) as u16,
        height: (font.height
            + 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int)
            + 2 as ::core::ffi::c_int * logical_px(2 as ::core::ffi::c_int)) as u16,
    };
    let mut supporting_wm_reply: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut input_focus: *mut xcb_get_input_focus_reply_t =
        ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    let mut geometry: *mut xcb_get_geometry_reply_t =
        ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    let mut wm_class: *mut xcb_get_property_reply_t =
        ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    let mut coordinates: *mut xcb_translate_coordinates_reply_t =
        ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    let mut A__NET_SUPPORTING_WM_CHECK: xcb_atom_t = 0;
    let nswc_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom(
        conn,
        0 as u8,
        strlen(c"_NET_SUPPORTING_WM_CHECK".as_ptr()) as u16,
        c"_NET_SUPPORTING_WM_CHECK".as_ptr(),
    );
    let nswc_reply: *mut xcb_intern_atom_reply_t = xcb_intern_atom_reply(
        conn,
        nswc_cookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if nswc_reply.is_null() {
        errorlog(
            c"[libi3] ERROR: Could not intern atom _NET_SUPPORTING_WM_CHECK\n"
                .as_ptr()
                .cast_mut(),
        );
        exit(EXIT_ERROR);
    }
    A__NET_SUPPORTING_WM_CHECK = (*nswc_reply).atom;
    free(nswc_reply as *mut ::core::ffi::c_void);
    supporting_wm_reply = xcb_get_property_reply(
        conn,
        xcb_get_property(
            conn,
            false_0 as u8,
            root,
            A__NET_SUPPORTING_WM_CHECK,
            XCB_ATOM_WINDOW as ::core::ffi::c_int as xcb_atom_t,
            0 as u32,
            32 as u32,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    let mut supporting_wm_win: *mut xcb_window_t = ::core::ptr::null_mut::<xcb_window_t>();
    if supporting_wm_reply.is_null()
        || xcb_get_property_value_length(supporting_wm_reply) == 0 as ::core::ffi::c_int
    {
        debuglog(
            c"%s:%s:%d - Could not determine EWMH support window.\n"
                .as_ptr()
                .cast_mut(),
            c"../i3-input/main.c".as_ptr(),
            c"get_window_position".as_ptr(),
            325 as ::core::ffi::c_int,
        );
    } else {
        supporting_wm_win = xcb_get_property_value(supporting_wm_reply) as *mut xcb_window_t;
    }
    xcb_grab_server(conn);
    input_focus = xcb_get_input_focus_reply(
        conn,
        xcb_get_input_focus(conn),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    );
    if input_focus.is_null() || (*input_focus).focus as ::core::ffi::c_long == XCB_NONE {
        debuglog(
            c"%s:%s:%d - Failed to receive the current input focus or no window has the input focus right now.\n".as_ptr().cast_mut(),
            c"../i3-input/main.c".as_ptr(),
            c"get_window_position".as_ptr(),
            336 as ::core::ffi::c_int,
        );
    } else if !supporting_wm_win.is_null() && (*input_focus).focus == *supporting_wm_win {
        debuglog(
            c"%s:%s:%d - Input focus is on the EWMH support window, ignoring.\n"
                .as_ptr()
                .cast_mut(),
            c"../i3-input/main.c".as_ptr(),
            c"get_window_position".as_ptr(),
            342 as ::core::ffi::c_int,
        );
    } else {
        geometry = xcb_get_geometry_reply(
            conn,
            xcb_get_geometry(conn, (*input_focus).focus as xcb_drawable_t),
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if geometry.is_null() {
            debuglog(
                c"%s:%s:%d - Failed to received window geometry.\n"
                    .as_ptr()
                    .cast_mut(),
                c"../i3-input/main.c".as_ptr(),
                c"get_window_position".as_ptr(),
                348 as ::core::ffi::c_int,
            );
        } else {
            wm_class = xcb_get_property_reply(
                conn,
                xcb_get_property(
                    conn,
                    false_0 as u8,
                    (*input_focus).focus,
                    XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                    XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                    0 as u32,
                    32 as u32,
                ),
                ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
            );
            if wm_class.is_null()
                || xcb_get_property_value_length(wm_class) == 0 as ::core::ffi::c_int
                || strcmp(
                    xcb_get_property_value(wm_class) as *const ::core::ffi::c_char,
                    c"i3-frame".as_ptr(),
                ) != 0 as ::core::ffi::c_int
            {
                coordinates = xcb_translate_coordinates_reply(
                    conn,
                    xcb_translate_coordinates(
                        conn,
                        (*input_focus).focus,
                        root,
                        (*geometry).x,
                        (*geometry).y,
                    ),
                    ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
                );
                if coordinates.is_null() {
                    debuglog(
                        c"%s:%s:%d - Failed to translate coordinates.\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../i3-input/main.c".as_ptr(),
                        c"get_window_position".as_ptr(),
                        360 as ::core::ffi::c_int,
                    );
                } else {
                    debuglog(
                        c"%s:%s:%d - Determined coordinates of window with input focus at x = %i / y = %i.\n".as_ptr().cast_mut(),
                        c"../i3-input/main.c".as_ptr(),
                        c"get_window_position".as_ptr(),
                        364 as ::core::ffi::c_int,
                        (*coordinates).dst_x as ::core::ffi::c_int,
                        (*coordinates).dst_y as ::core::ffi::c_int,
                    );
                    result.x = (result.x as ::core::ffi::c_int
                        + (*coordinates).dst_x as ::core::ffi::c_int)
                        as i16;
                    result.y = (result.y as ::core::ffi::c_int
                        + (*coordinates).dst_y as ::core::ffi::c_int)
                        as i16;
                }
            } else {
                debuglog(
                    c"%s:%s:%d - Determined coordinates of window with input focus at x = %i / y = %i.\n".as_ptr().cast_mut(),
                    c"../i3-input/main.c".as_ptr(),
                    c"get_window_position".as_ptr(),
                    368 as ::core::ffi::c_int,
                    (*geometry).x as ::core::ffi::c_int,
                    (*geometry).y as ::core::ffi::c_int,
                );
                result.x =
                    (result.x as ::core::ffi::c_int + (*geometry).x as ::core::ffi::c_int) as i16;
                result.y =
                    (result.y as ::core::ffi::c_int + (*geometry).y as ::core::ffi::c_int) as i16;
            }
        }
    }
    xcb_ungrab_server(conn);
    xcb_flush(conn);
    free(supporting_wm_reply as *mut ::core::ffi::c_void);
    supporting_wm_reply = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    free(input_focus as *mut ::core::ffi::c_void);
    input_focus = ::core::ptr::null_mut::<xcb_get_input_focus_reply_t>();
    free(geometry as *mut ::core::ffi::c_void);
    geometry = ::core::ptr::null_mut::<xcb_get_geometry_reply_t>();
    free(wm_class as *mut ::core::ffi::c_void);
    wm_class = ::core::ptr::null_mut::<xcb_get_property_reply_t>();
    free(coordinates as *mut ::core::ffi::c_void);
    coordinates = ::core::ptr::null_mut::<xcb_translate_coordinates_reply_t>();
    result
}
#[derive(Debug)]
struct InputSettings {
    socket: Option<std::ffi::CString>,
    font: Option<std::ffi::CString>,
    prompt: Option<std::ffi::CString>,
    format: std::ffi::CString,
    limit: i32,
}
impl Default for InputSettings {
    fn default() -> Self {
        Self {
            socket: None,
            font: None,
            prompt: None,
            format: c"%s".to_owned(),
            limit: 0,
        }
    }
}
const INPUT_OPTIONS: &[i3_common::cli::OptionSpec] = &[
    i3_common::cli::OptionSpec::value(b's', "socket"),
    i3_common::cli::OptionSpec::flag(b'v', "version"),
    i3_common::cli::OptionSpec::value(b'l', "limit"),
    i3_common::cli::OptionSpec::value(b'P', "prompt"),
    i3_common::cli::OptionSpec::value(b'p', "prefix"),
    i3_common::cli::OptionSpec::value(b'F', "format"),
    i3_common::cli::OptionSpec::value(b'f', "font"),
    i3_common::cli::OptionSpec::flag(b'h', "help"),
];

// The native text API owns a separate copy of the prompt. Its owner is scoped
// to the graphical session; callbacks only borrow its pointer while it is live.
struct InputPrompt(*mut i3String);
impl Drop for InputPrompt {
    fn drop(&mut self) {
        // SAFETY: i3string_from_utf8 created this uniquely owned text. The
        // event loop has finished, so no callback retains an active borrow.
        unsafe {
            prompt = std::ptr::null_mut();
            i3string_free(self.0);
        }
    }
}

unsafe fn run_window(settings: InputSettings) -> ::core::ffi::c_int {
    let socket_path = settings
        .socket
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    let pattern = settings
        .font
        .as_ref()
        .map_or(std::ptr::null(), |value| value.as_ptr());
    format = settings.format.as_ptr().cast_mut();
    limit = settings.limit;
    let _prompt_owner = settings
        .prompt
        .as_ref()
        .map(|value| InputPrompt(i3string_from_utf8(value.as_ptr())));
    prompt = _prompt_owner
        .as_ref()
        .map_or(std::ptr::null_mut(), |value| value.0);
    printf(c"using format \"%s\"\n".as_ptr(), format);
    let mut screen: ::core::ffi::c_int = 0;
    conn = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if conn.is_null() || xcb_connection_has_error(conn) != 0 {
        errx(libc::EXIT_FAILURE, c"Cannot open display".as_ptr());
    }
    sockfd = ipc_connect(socket_path);
    root_screen = xcb_aux_get_screen(conn, screen);
    root = (*root_screen).root;
    symbols = xcb_key_symbols_alloc(conn);
    init_dpi();
    font = load_font(
        if !pattern.is_null() {
            pattern as *const ::core::ffi::c_char
        } else {
            c"pango:monospace 8".as_ptr()
        },
        true_0 != 0,
    );
    set_font(&raw mut font);
    if !prompt.is_null() {
        prompt_offset = predict_text_width(prompt);
    }
    let win_pos: xcb_rectangle_t = get_window_position() as xcb_rectangle_t;
    win = xcb_generate_id(conn) as xcb_window_t;
    let mut fresh1: [u32; 3] = [
        0 as ::core::ffi::c_int as u32,
        1 as ::core::ffi::c_int as u32,
        XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int as u32,
    ];
    xcb_create_window(
        conn,
        XCB_COPY_FROM_PARENT as u8,
        win,
        root,
        win_pos.x,
        win_pos.y,
        win_pos.width,
        win_pos.height,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        XCB_WINDOW_CLASS_COPY_FROM_PARENT as ::core::ffi::c_int as xcb_visualid_t,
        (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
            | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
            | XCB_CW_EVENT_MASK as ::core::ffi::c_int) as u32,
        &raw mut fresh1 as *mut u32 as *const ::core::ffi::c_void,
    );
    xcb_map_window(conn, win);
    draw_util_surface_init(
        conn,
        &raw mut surface,
        win as xcb_drawable_t,
        get_visualtype(root_screen),
        win_pos.width as ::core::ffi::c_int,
        win_pos.height as ::core::ffi::c_int,
    );
    xcb_flush(conn);
    let mut cookie: xcb_grab_keyboard_cookie_t = xcb_grab_keyboard_cookie_t { sequence: 0 };
    let mut reply: *mut xcb_grab_keyboard_reply_t =
        ::core::ptr::null_mut::<xcb_grab_keyboard_reply_t>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (reply.is_null()
        || (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int)
        && {
            let fresh2 = count;
            count = count + 1;
            fresh2 < 500 as ::core::ffi::c_int
        }
    {
        cookie = xcb_grab_keyboard(
            conn,
            false_0 as u8,
            win,
            XCB_CURRENT_TIME as xcb_timestamp_t,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
            XCB_GRAB_MODE_ASYNC as ::core::ffi::c_int as u8,
        );
        reply = xcb_grab_keyboard_reply(
            conn,
            cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        usleep(1000 as libc::useconds_t);
    }
    if (*reply).status as ::core::ffi::c_int != XCB_GRAB_STATUS_SUCCESS as ::core::ffi::c_int {
        fprintf(
            stderr,
            c"Could not grab keyboard, status = %d\n".as_ptr(),
            (*reply).status as ::core::ffi::c_int,
        );
        exit(EXIT_ERROR);
    }
    xcb_flush(conn);
    let mut event: *mut xcb_generic_event_t = ::core::ptr::null_mut::<xcb_generic_event_t>();
    loop {
        event = xcb_wait_for_event(conn);
        if event.is_null() {
            break;
        }
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                c"X11 Error received! sequence %x\n".as_ptr(),
                (*event).sequence as ::core::ffi::c_int,
            );
            free(event as *mut ::core::ffi::c_void);
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int;
            match type_0 {
                XCB_KEY_PRESS => {
                    handle_key_press(NULL_0, conn, event as *mut xcb_key_press_event_t);
                }
                XCB_KEY_RELEASE => {
                    handle_key_release(NULL_0, conn, event as *mut xcb_key_release_event_t);
                }
                XCB_EXPOSE => {
                    if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        handle_expose(conn);
                    }
                }
                _ => {}
            }
            free(event as *mut ::core::ffi::c_void);
        }
    }
    draw_util_surface_free(conn, &raw mut surface);
    EXIT_OK
}

pub fn main() {
    use i3_common::cli::{Item, Parser};
    use std::os::unix::ffi::OsStrExt;
    let mut arguments = std::env::args_os();
    let program = arguments.next().unwrap_or_else(|| "i3-input".into());
    let mut settings = InputSettings::default();
    let parser = Parser::new(
        arguments,
        INPUT_OPTIONS,
        std::env::var_os("POSIXLY_CORRECT").is_some(),
    );
    for item in parser {
        match item {
            Item::Error(message) => eprintln!("{}: {message}", program.to_string_lossy()),
            Item::Operand(_) => {}
            Item::Option { short, value } => match short {
                b'v' => {
                    println!("i3-input {}", env!("I3_VERSION"));
                    return;
                }
                b'h' => {
                    println!("i3-input {}\ni3-input [-s <socket>] [-F <format>] [-l <limit>] [-P <prompt>] [-f <font>] [-v]\n\nExample:\n    i3-input -F 'workspace \"%s\"' -P 'Switch to workspace: '", env!("I3_VERSION"));
                    return;
                }
                b'l' => settings.limit = i3_common::cli::decimal_prefix(value.unwrap().as_bytes()),
                short => {
                    let mut bytes = value.unwrap().as_bytes().to_vec();
                    if short == b'p' {
                        eprintln!("i3-input: WARNING: the -p option is DEPRECATED in favor of the -F (format) option");
                        bytes.extend_from_slice(b"%s");
                    }
                    let value =
                        std::ffi::CString::new(bytes).expect("OS arguments have no NUL bytes");
                    match short {
                        b's' => settings.socket = Some(value),
                        b'P' => settings.prompt = Some(value),
                        b'f' => settings.font = Some(value),
                        b'F' | b'p' => settings.format = value,
                        _ => unreachable!(),
                    }
                }
            },
        }
    }
    // SAFETY: settings owns all C string arguments for the full event-loop
    // lifetime. Native X callbacks continue to follow their existing contracts.
    std::process::exit(unsafe { run_window(settings) });
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use i3_common::dpi::init_dpi;
use i3_common::dpi::logical_px;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.
pub use cairo_sys::{cairo_surface_t, cairo_t};

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_get_screen, xcb_charinfo_t, xcb_connect, xcb_connection_has_error,
    xcb_connection_t, xcb_create_window, xcb_drawable_t, xcb_expose_event_t, xcb_flush,
    xcb_generate_id, xcb_generic_error_t, xcb_generic_event_t, xcb_get_geometry,
    xcb_get_geometry_reply, xcb_get_geometry_reply_t, xcb_get_input_focus,
    xcb_get_input_focus_reply, xcb_get_input_focus_reply_t, xcb_get_property,
    xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_value,
    xcb_get_property_value_length, xcb_grab_keyboard, xcb_grab_keyboard_cookie_t,
    xcb_grab_keyboard_reply, xcb_grab_keyboard_reply_t, xcb_grab_server, xcb_intern_atom,
    xcb_intern_atom_cookie_t, xcb_intern_atom_reply, xcb_intern_atom_reply_t, xcb_is_cursor_key,
    xcb_is_function_key, xcb_is_keypad_key, xcb_is_misc_function_key, xcb_is_modifier_key,
    xcb_is_pf_key, xcb_is_private_keypad_key, xcb_key_press_event_t, xcb_key_press_lookup_keysym,
    xcb_key_release_event_t, xcb_key_symbols_alloc, xcb_key_symbols_t, xcb_keysym_t,
    xcb_map_window, xcb_query_font_reply_t, xcb_rectangle_t, xcb_screen_t, xcb_timestamp_t,
    xcb_translate_coordinates, xcb_translate_coordinates_reply, xcb_translate_coordinates_reply_t,
    xcb_ungrab_server, xcb_visualid_t, xcb_visualtype_t, xcb_wait_for_event, xcb_window_t,
    XCB_ATOM_WINDOW, XCB_ATOM_WM_CLASS, XCB_COPY_FROM_PARENT, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL,
    XCB_CW_EVENT_MASK, XCB_CW_OVERRIDE_REDIRECT, XCB_EVENT_MASK_EXPOSURE, XCB_EXPOSE,
    XCB_GET_PROPERTY_TYPE_ANY, XCB_GRAB_MODE_ASYNC, XCB_GRAB_STATUS_SUCCESS, XCB_KEY_PRESS,
    XCB_KEY_RELEASE, XCB_MOD_MASK_SHIFT, XCB_NONE, XCB_WINDOW_CLASS_COPY_FROM_PARENT,
    XCB_WINDOW_CLASS_INPUT_OUTPUT,
};

pub use libc::{exit, fprintf, free, memcpy, printf, strcmp, strcpy, strlen, strncat, usleep};

use i3_common::native_stdio::{errx, stderr, stdout, vfprintf};

// Internal calls and globals use canonical Rust identities.
use i3_common::get_visualtype::get_visualtype;
use i3_common::ipc_connect::ipc_connect;
use i3_common::ipc_send_message::ipc_send_message;

pub use i3_common::string::I3String as _i3String;

pub use i3_common::ucs2_conversion::Ucs2Char as xcb_char2b_t;
