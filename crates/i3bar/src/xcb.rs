use crate::config::config;
use crate::outputs::output_has_focus;
use crate::outputs::outputs;
use i3_common::draw_util::draw_util_clear_surface;
use i3_common::draw_util::draw_util_copy_surface;
use i3_common::draw_util::draw_util_hex_to_color;
use i3_common::draw_util::draw_util_rectangle;
use i3_common::draw_util::draw_util_surface_free;
use i3_common::draw_util::draw_util_surface_init;
use i3_common::draw_util::draw_util_text;
use i3_common::font::load_font;
use i3_common::font::predict_text_width;
use i3_common::font::set_font;
use i3_common::safewrappers::{sasprintf, sstrdup};
use i3_common::string::i3string_as_utf8;
use i3_common::string::i3string_free;
use i3_common::string::i3string_get_num_bytes;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see also: LICENSE)
 *
 * xcb.c: Communicating with X
 *
 */
// Mechanical file-by-file Rust translation of i3bar/src/xcb.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.
pub use libev_sys::{
    ev_io, ev_io_start, ev_io_stop, ev_loop, ev_prepare, ev_prepare_start, ev_prepare_stop,
    ev_watcher, EV_READ, EV__IOFDSET,
};

pub use i3_common::font::C2RustUnnamed;
pub use i3_common::font::C2RustUnnamed_0;
pub use i3_common::font::Font;

pub use crate::child::rect_t;
pub use i3_common::draw_util::color_t;
pub use i3_common::draw_util::surface_t;
pub type rect = rect_t;

pub use crate::child::status_block;
pub use crate::child::status_block_render_desc;
pub use crate::child::statusline_head;

pub use crate::child::i3_output;
pub use crate::child::i3_ws;
pub use crate::child::tc_head;
pub use crate::child::trayclient;
pub use crate::child::ws_head;
pub use crate::mode::Mode;

pub use crate::application::tray_output_t;

pub use crate::application::binding_t;

pub const NUM_ATOMS: usize = AtomIndex::COUNT;

pub const _NET_SYSTEM_TRAY_ORIENTATION_HORZ: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SYSTEM_TRAY_REQUEST_DOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const XEMBED_MAPPED: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const XEMBED_EMBEDDED_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_RUN_COMMAND: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const I3_IPC_MESSAGE_TYPE_SYNC: ::core::ffi::c_int = 11 as ::core::ffi::c_int;

pub static mut atom_cookies: [xcb_intern_atom_cookie_t; 12] =
    [xcb_intern_atom_cookie_t { sequence: 0 }; 12];

pub static mut atoms: [xcb_atom_t; 12] = [0; 12];

pub static mut xcb_connection: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;

pub static mut screen: ::core::ffi::c_int = 0;

pub static mut root_screen: *mut xcb_screen_t =
    ::core::ptr::null::<xcb_screen_t>() as *mut xcb_screen_t;

pub static mut xcb_root: xcb_window_t = 0;
static mut cursor: xcb_cursor_t = 0;
static mut selwin: xcb_window_t = XCB_NONE as xcb_window_t;
static mut tray_reply: *mut xcb_intern_atom_reply_t =
    ::core::ptr::null::<xcb_intern_atom_reply_t>() as *mut xcb_intern_atom_reply_t;

pub static mut conn: *mut xcb_connection_t =
    ::core::ptr::null::<xcb_connection_t>() as *mut xcb_connection_t;
static mut font: Font = Font {
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

pub static mut icon_size: ::core::ffi::c_int = 0;

pub static mut visual_type: *mut xcb_visualtype_t =
    ::core::ptr::null::<xcb_visualtype_t>() as *mut xcb_visualtype_t;

pub static mut depth: u8 = 0;

pub static mut colormap: xcb_colormap_t = 0;

pub static mut bar_height: ::core::ffi::c_int = 0;

pub static mut xkb_base: ::core::ffi::c_int = 0;

pub static mut mod_pressed: bool = 0 as ::core::ffi::c_int != 0;

pub static mut xcb_prep: *mut ev_prepare = ::core::ptr::null::<ev_prepare>() as *mut ev_prepare;

pub static mut xcb_io: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;

pub static mut xkb_io: *mut ev_io = ::core::ptr::null::<ev_io>() as *mut ev_io;
static mut binding: Mode = Mode {
    name: None,
    name_width: 0,
};

pub static mut activated_mode: bool = false;
static mut output_for_tray: *mut i3_output = ::core::ptr::null::<i3_output>() as *mut i3_output;

pub static mut colors: xcb_colors_t = xcb_colors_t {
    bar_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    bar_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    sep_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_bar_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_bar_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_sep_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    active_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    inactive_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    urgent_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    focus_ws_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_bg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_fg: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
    binding_mode_border: color_t {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
        colorpixel: 0,
    },
};
static mut ws_hoff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut ws_voff_px: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut ws_spacing_px: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut sb_hoff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut tray_loff_px: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut sep_voff_px: ::core::ffi::c_int = 4 as ::core::ffi::c_int;

pub static mut separator_symbol_width: ::core::ffi::c_int = 0;

// Native entry points require exclusive event-thread access to live display,
// output/tray/status graphs. Owned scratch values never extend graph borrows
// across native calls, drawing, reconfiguration, IPC or process callbacks.
struct Native<T>(std::ptr::NonNull<T>);
impl<T> Native<T> {
    /// Transfers unique ownership of a complete malloc-backed native allocation.
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
struct Replies(Vec<std::ptr::NonNull<libc::c_void>>);
impl Replies {
    unsafe fn take<T>(&mut self, pointer: *mut T) -> *mut T {
        if let Some(pointer) = std::ptr::NonNull::new(pointer) {
            self.0.push(pointer.cast());
        }
        pointer
    }
}
impl Drop for Replies {
    fn drop(&mut self) {
        for pointer in self.0.drain(..) {
            unsafe {
                free(pointer.as_ptr());
            }
        }
    }
}
unsafe fn property_bytes<'a>(reply: *const xcb_get_property_reply_t) -> Option<&'a [u8]> {
    let reply = reply.as_ref()?;
    let stride = match reply.format {
        8 => 1,
        16 => 2,
        32 => 4,
        _ => return None,
    };
    let length = (reply.value_len as usize).checked_mul(stride)?;
    let available = (reply.length as usize).checked_mul(4)?;
    if length > available || length > isize::MAX as usize {
        return None;
    }
    Some(std::slice::from_raw_parts(
        (reply as *const xcb_get_property_reply_t).add(1).cast(),
        length,
    ))
}
fn first_nul(bytes: &[u8]) -> &[u8] {
    bytes.split(|&byte| byte == 0).next().unwrap_or_default()
}
fn class_parts(bytes: &[u8]) -> (&[u8], Option<&[u8]>) {
    let instance = first_nul(bytes);
    (
        instance,
        bytes
            .get(instance.len() + 1..)
            .filter(|rest| !rest.is_empty())
            .map(first_nul),
    )
}
fn xembed_words(bytes: &[u8]) -> Option<[u32; 2]> {
    Some([
        u32::from_ne_bytes(bytes.get(..4)?.try_into().ok()?),
        u32::from_ne_bytes(bytes.get(4..8)?.try_into().ok()?),
    ])
}
fn separator_offset(no_separator: bool, width: u32) -> u32 {
    if no_separator {
        0
    } else {
        width / 2 + width % 2
    }
}
fn render_dimensions(
    width: u32,
    minimum: u32,
    align: crate::child::BlockAlign,
    previous: status_block_render_desc,
) -> status_block_render_desc {
    let mut result = status_block_render_desc { width, ..previous };
    if minimum <= width {
        result.x_offset = 0;
        result.x_append = 0;
    } else {
        let padding = minimum - width;
        // Left/right intentionally retain the other cached offset, matching the
        // old renderer rather than silently normalizing state on each draw.
        match align {
            crate::child::BlockAlign::Left => result.x_append = padding,
            crate::child::BlockAlign::Right => result.x_offset = padding,
            crate::child::BlockAlign::Center => {
                result.x_offset = padding / 2;
                result.x_append = padding / 2 + padding % 2;
            }
        }
    }
    result
}
fn quote_workspace(name: &std::ffi::CStr) -> std::ffi::CString {
    let mut bytes = b"workspace \"".to_vec();
    for &byte in name.to_bytes() {
        if matches!(byte, b'"' | b'\\') {
            bytes.push(b'\\');
        }
        bytes.push(byte);
    }
    bytes.push(b'"');
    std::ffi::CString::new(bytes).unwrap()
}
fn tray_width(mapped: usize, step: i32, left: i32) -> i32 {
    let mut width = 0;
    for _ in 0..mapped {
        width += step;
    }
    if width > 0 {
        width += left;
    }
    width
}
fn strut_values(
    position: crate::application::BarPosition,
    height: i32,
    bounds: rect_t,
) -> [u32; 12] {
    let mut values = [0; 12];
    match position {
        crate::application::BarPosition::Top => {
            values[2] = height as u32;
            values[8] = bounds.x as u32;
            values[9] = (bounds.x + bounds.w) as u32;
        }
        crate::application::BarPosition::Bottom => {
            values[3] = height as u32;
            values[10] = bounds.x as u32;
            values[11] = (bounds.x + bounds.w) as u32;
        }
        crate::application::BarPosition::None => {}
    }
    values
}
fn client_message(window: u32, type_: u32, data: [u32; 5]) -> xcb_client_message_event_t {
    xcb_client_message_event_t {
        response_type: XCB_CLIENT_MESSAGE as u8,
        format: 32,
        sequence: 0,
        window,
        type_0: type_,
        data: xcb_sys::xcb_client_message_data_t { data32: data },
    }
}
#[repr(C)]
struct DestroyMessage {
    event: xcb_destroy_notify_event_t,
    padding: [u8; 20],
}
fn destroy_message(window: u32) -> DestroyMessage {
    DestroyMessage {
        event: xcb_destroy_notify_event_t {
            response_type: XCB_DESTROY_NOTIFY as u8,
            pad0: 0,
            sequence: 0,
            event: window,
            window,
        },
        padding: [0; 20],
    }
}
const _: () = {
    assert!(std::mem::size_of::<xcb_client_message_event_t>() == 32);
    assert!(std::mem::size_of::<DestroyMessage>() == 32);
};
const ATOM_NAMES: [&std::ffi::CStr; 12] = [
    c"_NET_WM_WINDOW_TYPE",
    c"_NET_WM_WINDOW_TYPE_DOCK",
    c"_NET_WM_STRUT_PARTIAL",
    c"I3_SOCKET_PATH",
    c"MANAGER",
    c"_NET_SYSTEM_TRAY_ORIENTATION",
    c"_NET_SYSTEM_TRAY_VISUAL",
    c"_NET_SYSTEM_TRAY_OPCODE",
    c"_NET_SYSTEM_TRAY_COLORS",
    c"_XEMBED_INFO",
    c"_XEMBED",
    c"I3_SYNC",
];

pub unsafe fn _xcb_request_failed(
    cookie: xcb_void_cookie_t,
    err_msg: *mut ::core::ffi::c_char,
    line: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if let Some(error) = Native::take(xcb_request_check(xcb_connection, cookie)) {
        let code = i32::from((*error.raw()).error_code);
        fprintf(
            stderr,
            c"[%s:%d] ERROR: %s. X Error Code: %d\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            line,
            err_msg,
            code,
        );
        code
    } else {
        0
    }
}

unsafe fn get_sep_offset(block: *mut status_block) -> u32 {
    separator_offset((*block).no_separator, (*block).sep_block_width)
}
unsafe fn get_tray_width(trayclients: *mut tc_head) -> i32 {
    let mut count = 0;
    let mut client = (*trayclients).tqh_first;
    while !client.is_null() {
        if (*client).mapped {
            count += 1;
        }
        client = (*client).tailq.tqe_next;
    }
    tray_width(
        count,
        icon_size + logical_px(config.tray_padding),
        logical_px(tray_loff_px),
    )
}

unsafe fn draw_separator(
    output: *mut i3_output,
    x: u32,
    block: *mut status_block,
    use_focus_colors: bool,
) {
    let sep_fg: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_sep_fg
    } else {
        colors.sep_fg
    };
    let bar_bg: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_bar_bg
    } else {
        colors.bar_bg
    };
    let sep_offset: u32 = get_sep_offset(block);
    if (*block).blocks.tqe_next.is_null() || sep_offset == 0 as u32 {
        return;
    }
    let center_x: u32 = x.wrapping_sub(sep_offset);
    if config.separator_symbol.is_null() {
        draw_util_rectangle(
            &raw mut (*output).statusline_buffer,
            sep_fg,
            center_x as ::core::ffi::c_double,
            logical_px(sep_voff_px) as ::core::ffi::c_double,
            logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            (bar_height - 2 as ::core::ffi::c_int * logical_px(sep_voff_px))
                as ::core::ffi::c_double,
        );
    } else {
        let separator_x: u32 = if x.wrapping_sub((*block).sep_block_width)
            > center_x.wrapping_sub((separator_symbol_width / 2 as ::core::ffi::c_int) as u32)
        {
            x.wrapping_sub((*block).sep_block_width)
        } else {
            center_x.wrapping_sub((separator_symbol_width / 2 as ::core::ffi::c_int) as u32)
        };
        draw_util_text(
            config.separator_symbol,
            &raw mut (*output).statusline_buffer,
            sep_fg,
            bar_bg,
            separator_x as ::core::ffi::c_int,
            bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
            x.wrapping_sub(separator_x) as ::core::ffi::c_int,
        );
    };
}
unsafe fn predict_block_length(block: *mut status_block) {
    let mut text: *mut I3String = (*block).full_text;
    let mut render: *mut status_block_render_desc = &raw mut (*block).full_render;
    if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
        text = (*block).short_text;
        render = &raw mut (*block).short_render;
    }
    if i3string_get_num_bytes(text) == 0 as usize {
        (*block).render_length = 0 as u32;
        return;
    }
    (*render).width = predict_text_width(text) as u32;
    if !(*block).border.is_null() {
        (*render).width = (*render).width.wrapping_add(logical_px(
            (*block).border_left.wrapping_add((*block).border_right) as ::core::ffi::c_int,
        ) as u32);
    }
    *render = render_dimensions((*render).width, (*block).min_width, (*block).align, *render);
    (*block).render_length = (*render)
        .width
        .wrapping_add((*render).x_offset)
        .wrapping_add((*render).x_append);
}
unsafe fn predict_statusline_length() -> u32 {
    let mut width: u32 = 0 as u32;
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        predict_block_length(block);
        let block_width: u32 = (*block).render_length;
        if !(block_width == 0 as u32) {
            width = width.wrapping_add(block_width);
            if !(*block).blocks.tqe_next.is_null() {
                width = width.wrapping_add((*block).sep_block_width);
            }
        }
        block = (*block).blocks.tqe_next;
    }
    width
}
unsafe fn switch_block_to_short(block: *mut status_block) -> u32 {
    if (*block).short_text.is_null() || (*block).use_short as ::core::ffi::c_int != 0 {
        return 0 as u32;
    }
    let full: u32 = (*block).render_length;
    (*block).use_short = true;
    predict_block_length(block);
    full.wrapping_sub((*block).render_length)
}
unsafe fn adjust_statusline_length(max_length: u32) -> u32 {
    let mut width: u32 = predict_statusline_length();
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        if width < max_length {
            break;
        }
        width = width.wrapping_sub(switch_block_to_short(block));
        if !(*block).name.is_null() {
            let mut other: *mut status_block = ::core::ptr::null_mut::<status_block>();
            other = statusline_head.tqh_first;
            while !other.is_null() {
                if !(*other).name.is_null() && strcmp((*other).name, (*block).name) == 0 {
                    width = width.wrapping_sub(switch_block_to_short(other));
                }
                other = (*other).blocks.tqe_next;
            }
        }
        block = (*block).blocks.tqe_next;
    }
    width
}
unsafe fn draw_statusline(output: *mut i3_output, clip_left: u32, use_focus_colors: bool) {
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    let bar_color: color_t = if use_focus_colors as ::core::ffi::c_int != 0 {
        colors.focus_bar_bg
    } else {
        colors.bar_bg
    };
    draw_util_clear_surface(&raw mut (*output).statusline_buffer, bar_color);
    let mut x: u32 = (0 as u32).wrapping_sub(clip_left);
    block = statusline_head.tqh_first;
    while !block.is_null() {
        let mut text: *mut I3String = (*block).full_text;
        let mut render: *mut status_block_render_desc = &raw mut (*block).full_render;
        if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
            text = (*block).short_text;
            render = &raw mut (*block).short_render;
        }
        if !(i3string_get_num_bytes(text) == 0 as usize) {
            let mut fg_color: color_t = color_t {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
                colorpixel: 0,
            };
            if (*block).urgent {
                fg_color = colors.urgent_ws_fg;
            } else if !(*block).color.is_null() {
                fg_color = draw_util_hex_to_color((*block).color);
            } else if use_focus_colors {
                fg_color = colors.focus_bar_fg;
            } else {
                fg_color = colors.bar_fg;
            }
            let mut bg_color: color_t = bar_color;
            let full_render_width: ::core::ffi::c_int = (*render)
                .width
                .wrapping_add((*render).x_offset)
                .wrapping_add((*render).x_append)
                as ::core::ffi::c_int;
            let has_border: ::core::ffi::c_int = if !(*block).border.is_null() {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            if !(*block).border.is_null()
                || !(*block).background.is_null()
                || (*block).urgent as ::core::ffi::c_int != 0
            {
                let mut border_color: color_t = bar_color;
                if (*block).urgent {
                    border_color = colors.urgent_ws_border;
                    bg_color = colors.urgent_ws_bg;
                } else {
                    if !(*block).border.is_null() {
                        border_color = draw_util_hex_to_color((*block).border);
                    }
                    if !(*block).background.is_null() {
                        bg_color = draw_util_hex_to_color((*block).background);
                    }
                }
                draw_util_rectangle(
                    &raw mut (*output).statusline_buffer,
                    border_color,
                    x as ::core::ffi::c_double,
                    logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
                    full_render_width as ::core::ffi::c_double,
                    (bar_height - logical_px(2 as ::core::ffi::c_int)) as ::core::ffi::c_double,
                );
                draw_util_rectangle(
                    &raw mut (*output).statusline_buffer,
                    bg_color,
                    x.wrapping_add(
                        (has_border * logical_px((*block).border_left as ::core::ffi::c_int))
                            as u32,
                    ) as ::core::ffi::c_double,
                    (logical_px(1 as ::core::ffi::c_int)
                        + has_border * logical_px((*block).border_top as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                    (full_render_width
                        - has_border
                            * logical_px((*block).border_right.wrapping_add((*block).border_left)
                                as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                    (bar_height
                        - has_border
                            * logical_px((*block).border_bottom.wrapping_add((*block).border_top)
                                as ::core::ffi::c_int)
                        - logical_px(2 as ::core::ffi::c_int))
                        as ::core::ffi::c_double,
                );
            }
            draw_util_text(
                text,
                &raw mut (*output).statusline_buffer,
                fg_color,
                bg_color,
                x.wrapping_add((*render).x_offset).wrapping_add(
                    (has_border * logical_px((*block).border_left as ::core::ffi::c_int)) as u32,
                ) as ::core::ffi::c_int,
                bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
                (*render).width.wrapping_sub(
                    (has_border
                        * logical_px((*block).border_left.wrapping_add((*block).border_right)
                            as ::core::ffi::c_int)) as u32,
                ) as ::core::ffi::c_int,
            );
            x = x.wrapping_add(full_render_width as u32);
            if !(*block).blocks.tqe_next.is_null() {
                x = x.wrapping_add((*block).sep_block_width);
                draw_separator(output, x, block, use_focus_colors);
            }
        }
        block = (*block).blocks.tqe_next;
    }
}
unsafe fn hide_bars() {
    if config.hide_on_modifier == crate::application::BarDisplayMode::Dock
        || config.hidden_state == crate::application::HiddenState::Show
            && config.hide_on_modifier == crate::application::BarDisplayMode::Hide
    {
        return;
    }
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if (*walk).active {
            xcb_unmap_window(xcb_connection, (*walk).bar.id as xcb_window_t);
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    stop_children();
}
unsafe fn unhide_bars() {
    if config.hide_on_modifier != crate::application::BarDisplayMode::Hide {
        return;
    }
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let mut cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
    let mut mask: u32 = 0;
    let mut values: [u32; 5] = [0; 5];
    cont_children();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if !((*walk).bar.id as ::core::ffi::c_long == XCB_NONE) {
            mask = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int) as u32;
            values[0 as ::core::ffi::c_int as usize] = (*walk).rect.x as u32;
            if config.position == crate::application::BarPosition::Top {
                values[1 as ::core::ffi::c_int as usize] = (*walk).rect.y as u32;
            } else {
                values[1 as ::core::ffi::c_int as usize] =
                    ((*walk).rect.y + (*walk).rect.h - bar_height) as u32;
            }
            values[2 as ::core::ffi::c_int as usize] = (*walk).rect.w as u32;
            values[3 as ::core::ffi::c_int as usize] = bar_height as u32;
            values[4 as ::core::ffi::c_int as usize] =
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as u32;
            if config.verbose {
                printf(
                    c"[%s:%d] Reconfiguring window for output %s to %d,%d\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    435 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
            cookie = xcb_configure_window_checked(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask as u16,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            if _xcb_request_failed(
                cookie,
                c"Could not reconfigure window".as_ptr().cast_mut(),
                441 as ::core::ffi::c_int,
            ) != 0
            {
                exit(libc::EXIT_FAILURE);
            }
            xcb_map_window(xcb_connection, (*walk).bar.id as xcb_window_t);
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
}

pub unsafe fn init_colors(new_colors: *const xcb_color_strings_t) {
    colors.bar_fg = draw_util_hex_to_color(if !(*new_colors).bar_fg.is_null() {
        (*new_colors).bar_fg as *const ::core::ffi::c_char
    } else {
        c"#FFFFFF".as_ptr()
    });
    colors.bar_bg = draw_util_hex_to_color(if !(*new_colors).bar_bg.is_null() {
        (*new_colors).bar_bg as *const ::core::ffi::c_char
    } else {
        c"#000000".as_ptr()
    });
    colors.sep_fg = draw_util_hex_to_color(if !(*new_colors).sep_fg.is_null() {
        (*new_colors).sep_fg as *const ::core::ffi::c_char
    } else {
        c"#666666".as_ptr()
    });
    colors.active_ws_fg = draw_util_hex_to_color(if !(*new_colors).active_ws_fg.is_null() {
        (*new_colors).active_ws_fg as *const ::core::ffi::c_char
    } else {
        c"#FFFFFF".as_ptr()
    });
    colors.active_ws_bg = draw_util_hex_to_color(if !(*new_colors).active_ws_bg.is_null() {
        (*new_colors).active_ws_bg as *const ::core::ffi::c_char
    } else {
        c"#333333".as_ptr()
    });
    colors.active_ws_border =
        draw_util_hex_to_color(if !(*new_colors).active_ws_border.is_null() {
            (*new_colors).active_ws_border as *const ::core::ffi::c_char
        } else {
            c"#333333".as_ptr()
        });
    colors.inactive_ws_fg = draw_util_hex_to_color(if !(*new_colors).inactive_ws_fg.is_null() {
        (*new_colors).inactive_ws_fg as *const ::core::ffi::c_char
    } else {
        c"#888888".as_ptr()
    });
    colors.inactive_ws_bg = draw_util_hex_to_color(if !(*new_colors).inactive_ws_bg.is_null() {
        (*new_colors).inactive_ws_bg as *const ::core::ffi::c_char
    } else {
        c"#222222".as_ptr()
    });
    colors.inactive_ws_border =
        draw_util_hex_to_color(if !(*new_colors).inactive_ws_border.is_null() {
            (*new_colors).inactive_ws_border as *const ::core::ffi::c_char
        } else {
            c"#333333".as_ptr()
        });
    colors.urgent_ws_fg = draw_util_hex_to_color(if !(*new_colors).urgent_ws_fg.is_null() {
        (*new_colors).urgent_ws_fg as *const ::core::ffi::c_char
    } else {
        c"#FFFFFF".as_ptr()
    });
    colors.urgent_ws_bg = draw_util_hex_to_color(if !(*new_colors).urgent_ws_bg.is_null() {
        (*new_colors).urgent_ws_bg as *const ::core::ffi::c_char
    } else {
        c"#900000".as_ptr()
    });
    colors.urgent_ws_border =
        draw_util_hex_to_color(if !(*new_colors).urgent_ws_border.is_null() {
            (*new_colors).urgent_ws_border as *const ::core::ffi::c_char
        } else {
            c"#2f343a".as_ptr()
        });
    colors.focus_ws_fg = draw_util_hex_to_color(if !(*new_colors).focus_ws_fg.is_null() {
        (*new_colors).focus_ws_fg as *const ::core::ffi::c_char
    } else {
        c"#FFFFFF".as_ptr()
    });
    colors.focus_ws_bg = draw_util_hex_to_color(if !(*new_colors).focus_ws_bg.is_null() {
        (*new_colors).focus_ws_bg as *const ::core::ffi::c_char
    } else {
        c"#285577".as_ptr()
    });
    colors.focus_ws_border = draw_util_hex_to_color(if !(*new_colors).focus_ws_border.is_null() {
        (*new_colors).focus_ws_border as *const ::core::ffi::c_char
    } else {
        c"#4c7899".as_ptr()
    });
    colors.binding_mode_fg = if !(*new_colors).binding_mode_fg.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_fg)
    } else {
        colors.urgent_ws_fg
    };
    colors.binding_mode_bg = if !(*new_colors).binding_mode_bg.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_bg)
    } else {
        colors.urgent_ws_bg
    };
    colors.binding_mode_border = if !(*new_colors).binding_mode_border.is_null() {
        draw_util_hex_to_color((*new_colors).binding_mode_border)
    } else {
        colors.urgent_ws_border
    };
    colors.focus_bar_fg = if !(*new_colors).focus_bar_fg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_bar_fg)
    } else {
        colors.bar_fg
    };
    colors.focus_bar_bg = if !(*new_colors).focus_bar_bg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_bar_bg)
    } else {
        colors.bar_bg
    };
    colors.focus_sep_fg = if !(*new_colors).focus_sep_fg.is_null() {
        draw_util_hex_to_color((*new_colors).focus_sep_fg)
    } else {
        colors.sep_fg
    };
    init_tray_colors();
    xcb_flush(xcb_connection);
}
unsafe fn execute_custom_command(input_code: xcb_keycode_t, event_is_release: bool) -> bool {
    let mut binding_0: *mut binding_t = ::core::ptr::null_mut::<binding_t>();
    binding_0 = config.bindings.tqh_first as *mut binding_t;
    while !binding_0.is_null() {
        if (*binding_0).input_code != input_code as ::core::ffi::c_int
            || (*binding_0).release as ::core::ffi::c_int != event_is_release as ::core::ffi::c_int
        {
            binding_0 = (*binding_0).bindings.tqe_next as *mut binding_t;
        } else {
            i3_send_msg(
                I3_IPC_MESSAGE_TYPE_RUN_COMMAND as u32,
                (*binding_0).command_ptr(),
            );
            return true;
        }
    }
    false
}
unsafe fn child_handle_button(
    event: *mut xcb_button_press_event_t,
    output: *mut i3_output,
    statusline_x: u32,
    statusline_y: u32,
) {
    if statusline_x > (*output).statusline_width as u32 {
        return;
    }
    let mut last_block_x: u32 = 0 as u32;
    let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
    block = statusline_head.tqh_first;
    while !block.is_null() {
        let mut text: *mut I3String = ::core::ptr::null_mut::<I3String>();
        let mut render: *mut status_block_render_desc =
            ::core::ptr::null_mut::<status_block_render_desc>();
        if (*block).use_short as ::core::ffi::c_int != 0 && !(*block).short_text.is_null() {
            text = (*block).short_text;
            render = &raw mut (*block).short_render;
        } else {
            text = (*block).full_text;
            render = &raw mut (*block).full_render;
        }
        if !(i3string_get_num_bytes(text) == 0 as usize) {
            let full_render_width: u32 = (*render)
                .width
                .wrapping_add((*render).x_offset)
                .wrapping_add((*render).x_append);
            let relative_x: u32 = statusline_x.wrapping_sub(last_block_x);
            if relative_x <= full_render_width {
                let output_x: u32 = (*event).event_x as u32;
                let output_y: u32 = statusline_y.wrapping_add((*event).event_y as u32);
                send_block_clicked(
                    (*event).detail as ::core::ffi::c_int,
                    (*block).name,
                    (*block).instance,
                    (*event).root_x as ::core::ffi::c_int,
                    (*event).root_y as ::core::ffi::c_int,
                    relative_x as ::core::ffi::c_int,
                    (*event).event_y as ::core::ffi::c_int,
                    output_x as ::core::ffi::c_int,
                    output_y as ::core::ffi::c_int,
                    full_render_width as ::core::ffi::c_int,
                    bar_height,
                    (*event).state as ::core::ffi::c_int,
                );
                return;
            }
            last_block_x =
                last_block_x.wrapping_add(full_render_width.wrapping_add((*block).sep_block_width));
            if last_block_x > statusline_x {
                return;
            }
        }
        block = (*block).blocks.tqe_next;
    }
}
unsafe fn predict_button_width(name_width: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if name_width
        + 2 as ::core::ffi::c_int * logical_px(ws_hoff_px)
        + 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)
        > logical_px(config.ws_min_width)
    {
        name_width
            + 2 as ::core::ffi::c_int * logical_px(ws_hoff_px)
            + 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)
    } else {
        logical_px(config.ws_min_width)
    }
}
unsafe extern "C" fn focus_workspace(ws: *mut i3_ws) {
    let command = if (*ws).id != 0 {
        std::ffi::CString::new(format!("[con_id={}] focus workspace", (*ws).id)).unwrap()
    } else {
        let Some(name) = (*ws).canonical_name.as_deref() else {
            return;
        };
        quote_workspace(name)
    };
    i3_send_msg(I3_IPC_MESSAGE_TYPE_RUN_COMMAND as u32, command.as_ptr());
}

unsafe fn handle_button(event: *mut xcb_button_press_event_t) {
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let bar: xcb_window_t = (*event).event;
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if (*walk).bar.id == bar {
            break;
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    if walk.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] Unknown bar clicked!\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                626 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Got button %d\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            630 as ::core::ffi::c_int,
            (*event).detail as ::core::ffi::c_int,
        );
    }
    let event_is_release: bool = (*event).response_type as ::core::ffi::c_int
        & !(0x80 as ::core::ffi::c_int)
        == XCB_BUTTON_RELEASE;
    let x: ::core::ffi::c_int =
        (if (*event).event_x as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            (*event).event_x as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) - logical_px(config.padding.x as ::core::ffi::c_int) as ::core::ffi::c_int;
    if x < 0 as ::core::ffi::c_int {
        return;
    }
    let mut workspace_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cur_ws: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    let mut clicked_ws: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
    ws_walk = (*(*walk).workspaces).tqh_first as *mut i3_ws;
    while !ws_walk.is_null() {
        let w: ::core::ffi::c_int = predict_button_width((*ws_walk).name_width);
        if x >= workspace_width && x <= workspace_width + w {
            clicked_ws = ws_walk;
        }
        if (*ws_walk).visible {
            cur_ws = ws_walk;
        }
        workspace_width += w;
        if !(*ws_walk).tailq.tqe_next.is_null() {
            workspace_width += logical_px(ws_spacing_px);
        }
        ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
    }
    if child_want_click_events() as ::core::ffi::c_int != 0 && x > workspace_width {
        let tray_width: ::core::ffi::c_int =
            get_tray_width((*walk).trayclients as *mut tc_head) as ::core::ffi::c_int;
        let offset_x: ::core::ffi::c_int = (*walk).rect.w
            - (*walk).statusline_width
            - tray_width
            - logical_px((tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int * sb_hoff_px)
                as ::core::ffi::c_int;
        if x >= offset_x {
            if !event_is_release {
                let statusline_x: u32 = (x - offset_x) as u32;
                let statusline_y: u32 = ((*event).root_y as ::core::ffi::c_int
                    - (*walk).rect.y
                    - (*event).event_y as ::core::ffi::c_int)
                    as u32;
                child_handle_button(event, walk, statusline_x, statusline_y);
            }
            return;
        }
    }
    if execute_custom_command((*event).detail as xcb_keycode_t, event_is_release)
        as ::core::ffi::c_int
        != 0
        || event_is_release as ::core::ffi::c_int != 0
    {
        return;
    }
    if cur_ws.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] No workspace active?\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                690 as ::core::ffi::c_int,
            );
        }
        return;
    }
    match (*event).detail as ::core::ffi::c_int {
        XCB_BUTTON_SCROLL_UP | XCB_BUTTON_SCROLL_LEFT => {
            if cur_ws == (*(*walk).workspaces).tqh_first {
                return;
            }
            cur_ws = *(*((*cur_ws).tailq.tqe_prev as *mut ws_head)).tqh_last as *mut i3_ws;
        }
        XCB_BUTTON_SCROLL_DOWN | XCB_BUTTON_SCROLL_RIGHT => {
            if cur_ws == *(*((*(*walk).workspaces).tqh_last as *mut ws_head)).tqh_last {
                return;
            }
            cur_ws = (*cur_ws).tailq.tqe_next as *mut i3_ws;
        }
        1 => {
            cur_ws = clicked_ws;
            if cur_ws.is_null() {
                cur_ws = (*(*walk).workspaces).tqh_first as *mut i3_ws;
                while !cur_ws.is_null() {
                    if (*cur_ws).visible as ::core::ffi::c_int != 0 && !(*cur_ws).focused {
                        break;
                    }
                    cur_ws = (*cur_ws).tailq.tqe_next as *mut i3_ws;
                }
            }
            if cur_ws.is_null() {
                return;
            }
        }
        _ => return,
    }
    focus_workspace(cur_ws);
}
unsafe fn handle_visibility_notify(event: *mut xcb_visibility_notify_event_t) {
    let visible: bool =
        (*event).state as ::core::ffi::c_int != XCB_VISIBILITY_FULLY_OBSCURED as ::core::ffi::c_int;
    let mut num_visible: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            if (*output).bar.id == (*event).window {
                (*output).visible = visible;
            }
            num_visible += (*output).visible as ::core::ffi::c_int;
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
    if num_visible == 0 as ::core::ffi::c_int {
        stop_children();
    } else {
        cont_children();
    };
}
unsafe extern "C" fn reorder_trayclients_cmp(
    mut _a: *const ::core::ffi::c_void,
    mut _b: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let a: *mut trayclient = *(_a as *mut *mut trayclient);
    let b: *mut trayclient = *(_b as *mut *mut trayclient);
    let result: ::core::ffi::c_int = strcasecmp_nullable((*a).class_class, (*b).class_class);
    if result != 0 as ::core::ffi::c_int {
        result
    } else {
        strcasecmp_nullable((*a).class_instance, (*b).class_instance)
    }
}
unsafe fn configure_trayclients() {
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut client: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
            client = (*(*output).trayclients).tqh_first as *mut trayclient;
            while !client.is_null() {
                if (*client).mapped {
                    count += 1;
                }
                client = (*client).tailq.tqe_next as *mut trayclient;
            }
            let mut trayclients: Vec<*mut trayclient> = Vec::with_capacity(count as usize);
            client = (*(*output).trayclients).tqh_first;
            while !client.is_null() {
                if (*client).mapped {
                    trayclients.push(client);
                }
                client = (*client).tailq.tqe_next;
            }
            // Native comparator/tie semantics retained; Vec owns the bounded
            // scratch array while qsort rearranges only Copy pointer values.
            qsort(
                trayclients.as_mut_ptr().cast(),
                trayclients.len(),
                std::mem::size_of::<*mut trayclient>(),
                Some(reorder_trayclients_cmp),
            );
            let mut x: u32 = (*output).rect.w as u32;
            let mut idx = count;
            while idx > 0 as ::core::ffi::c_int {
                x = x.wrapping_sub((icon_size + logical_px(config.tray_padding)) as u32);
                if config.verbose {
                    printf(
                        c"[%s:%d] Configuring tray window %08x to x=%d\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        819 as ::core::ffi::c_int,
                        (*trayclients[(idx - 1) as usize]).win,
                        x,
                    );
                }
                xcb_configure_window(
                    xcb_connection,
                    (*trayclients[(idx - 1) as usize]).win,
                    XCB_CONFIG_WINDOW_X as ::core::ffi::c_int as u16,
                    &raw mut x as *const ::core::ffi::c_void,
                );
                idx -= 1;
            }
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
}
unsafe fn trayclient_and_output_from_window(
    win: xcb_window_t,
    output: *mut *mut i3_output,
) -> *mut trayclient {
    let mut o_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    o_walk = (*outputs).slh_first as *mut i3_output;
    while !o_walk.is_null() {
        if (*o_walk).active {
            let mut client: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
            client = (*(*o_walk).trayclients).tqh_first as *mut trayclient;
            while !client.is_null() {
                if (*client).win == win {
                    if !output.is_null() {
                        *output = o_walk;
                    }
                    return client;
                }
                client = (*client).tailq.tqe_next as *mut trayclient;
            }
        }
        o_walk = (*o_walk).slist.sle_next as *mut i3_output;
    }
    ::core::ptr::null_mut::<trayclient>()
}
unsafe fn trayclient_from_window(win: xcb_window_t) -> *mut trayclient {
    trayclient_and_output_from_window(win, ::core::ptr::null_mut::<*mut i3_output>())
}
unsafe fn trayclient_update_class(client: *mut trayclient) {
    let mut replies = Replies::default();
    let prop: *mut xcb_get_property_reply_t = replies.take(xcb_get_property_reply(
        conn,
        xcb_get_property_unchecked(
            xcb_connection,
            0,
            (*client).win,
            XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
            XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
            0 as u32,
            32 as u32,
        ),
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ));
    if prop.is_null() || xcb_get_property_value_length(prop) == 0 as ::core::ffi::c_int {
        if config.verbose {
            printf(
                c"[%s:%d] WM_CLASS not set.\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                867 as ::core::ffi::c_int,
            );
        }
        return;
    }
    let Some(bytes) = property_bytes(prop) else {
        return;
    };
    let (instance, class) = class_parts(bytes);
    let instance = std::ffi::CString::new(instance).unwrap();
    let class = class.map(|bytes| std::ffi::CString::new(bytes).unwrap());
    free((*client).class_instance.cast());
    free((*client).class_class.cast());
    (*client).class_instance = sstrdup(instance.as_ptr());
    (*client).class_class = class
        .as_ref()
        .map_or(std::ptr::null_mut(), |class| sstrdup(class.as_ptr()));
    if config.verbose {
        printf(
            c"[%s:%d] WM_CLASS changed to %s (instance), %s (class)\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            888 as ::core::ffi::c_int,
            (*client).class_instance,
            (*client).class_class,
        );
    }
}
unsafe fn handle_client_message(event: *mut xcb_client_message_event_t) {
    let mut replies = Replies::default();
    if (*event).type_0 == atoms[AtomIndex::I3Sync as ::core::ffi::c_int as usize] {
        let window: xcb_window_t = (*event).data.data32[0 as ::core::ffi::c_int as usize];
        let rnd: u32 = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut payload: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sasprintf(
            &raw mut payload,
            c"{\"rnd\":%d, \"window\":%d}".as_ptr(),
            rnd,
            window,
        );
        i3_send_msg(I3_IPC_MESSAGE_TYPE_SYNC as u32, payload);
        free(payload as *mut ::core::ffi::c_void);
    } else if (*event).type_0
        == atoms[AtomIndex::NetSystemTrayOpcode as ::core::ffi::c_int as usize]
        && (*event).format as ::core::ffi::c_int == 32 as ::core::ffi::c_int
    {
        if config.verbose {
            printf(
                c"[%s:%d] _NET_SYSTEM_TRAY_OPCODE received\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                912 as ::core::ffi::c_int,
            );
        }
        let op: u32 = (*event).data.data32[1 as ::core::ffi::c_int as usize];
        let mut values: [u32; 2] = [0; 2];
        if op == SYSTEM_TRAY_REQUEST_DOCK as u32 {
            let client: xcb_window_t = (*event).data.data32[2 as ::core::ffi::c_int as usize];
            let mut mask: u32 = XCB_CW_EVENT_MASK as ::core::ffi::c_int as u32;
            values[0 as ::core::ffi::c_int as usize] =
                XCB_EVENT_MASK_PROPERTY_CHANGE as ::core::ffi::c_int as u32;
            values[0 as ::core::ffi::c_int as usize] |=
                XCB_EVENT_MASK_STRUCTURE_NOTIFY as ::core::ffi::c_int as u32;
            values[0 as ::core::ffi::c_int as usize] |=
                XCB_EVENT_MASK_RESIZE_REDIRECT as ::core::ffi::c_int as u32;
            xcb_change_window_attributes(
                xcb_connection,
                client,
                mask,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            let mut map_it: bool = true;
            let mut xe_version: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut xembedc: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
            let mut error: *mut xcb_generic_error_t =
                ::core::ptr::null_mut::<xcb_generic_error_t>();
            xembedc = xcb_get_property(
                xcb_connection,
                0 as u8,
                client,
                atoms[AtomIndex::XembedInfo as ::core::ffi::c_int as usize],
                XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
                0 as u32,
                (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as u32,
            );
            let xembedr: *mut xcb_get_property_reply_t = replies.take(xcb_get_property_reply(
                xcb_connection,
                xembedc,
                &raw mut error,
            ));
            if !error.is_null() {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: Error getting _XEMBED_INFO property: error_code %d\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    950 as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
                free(error as *mut ::core::ffi::c_void);
                return;
            }
            if !xembedr.is_null() && (*xembedr).length != 0 as u32 {
                if config.verbose {
                    printf(
                        c"[%s:%d] xembed format = %d, len = %d\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        955 as ::core::ffi::c_int,
                        (*xembedr).format as ::core::ffi::c_int,
                        (*xembedr).length,
                    );
                }
                let Some(xembed) = property_bytes(xembedr).and_then(xembed_words) else {
                    return;
                };
                if config.verbose {
                    printf(
                        c"[%s:%d] xembed version = %d\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        957 as ::core::ffi::c_int,
                        xembed[0],
                    );
                }
                if config.verbose {
                    printf(
                        c"[%s:%d] xembed flags = %d\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        958 as ::core::ffi::c_int,
                        xembed[1],
                    );
                }
                map_it = xembed[1] & XEMBED_MAPPED as u32 == XEMBED_MAPPED as u32;
                xe_version = xembed[0] as ::core::ffi::c_int;
                if xe_version > 1 as ::core::ffi::c_int {
                    xe_version = 1 as ::core::ffi::c_int;
                }
            } else {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: Window %08x violates the XEMBED protocol, _XEMBED_INFO not set\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    966 as ::core::ffi::c_int,
                    client,
                );
            }
            if config.verbose {
                printf(
                    c"[%s:%d] X window %08x requested docking\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    969 as ::core::ffi::c_int,
                    client,
                );
            }
            if output_for_tray.is_null() {
                fprintf(
                    stderr,
                    c"[%s:%d] ERROR: No output found for tray\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    972 as ::core::ffi::c_int,
                );
                return;
            }
            let rcookie: xcb_void_cookie_t = xcb_reparent_window(
                xcb_connection,
                client,
                (*output_for_tray).bar.id as xcb_window_t,
                ((*output_for_tray).rect.w - icon_size - logical_px(config.tray_padding)) as i16,
                logical_px(config.tray_padding) as i16,
            );
            if _xcb_request_failed(
                rcookie,
                c"Could not reparent window. Maybe it is using an incorrect depth/visual?"
                    .as_ptr()
                    .cast_mut(),
                981 as ::core::ffi::c_int,
            ) != 0
            {
                return;
            }
            mask = (XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int) as u32;
            values[0 as ::core::ffi::c_int as usize] = icon_size as u32;
            values[1 as ::core::ffi::c_int as usize] = icon_size as u32;
            xcb_configure_window(
                xcb_connection,
                client,
                mask as u16,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            let packet = client_message(
                client,
                atoms[AtomIndex::Xembed as usize],
                [
                    XCB_CURRENT_TIME as u32,
                    XEMBED_EMBEDDED_NOTIFY as u32,
                    (*output_for_tray).bar.id,
                    xe_version as u32,
                    0,
                ],
            );
            xcb_send_event(
                xcb_connection,
                0,
                client,
                XCB_EVENT_MASK_NO_EVENT as u32,
                (&raw const packet).cast(),
            );
            xcb_change_save_set(
                xcb_connection,
                XCB_SET_MODE_INSERT as ::core::ffi::c_int as u8,
                client,
            );
            let tc: *mut trayclient =
                scalloc(1 as usize, ::core::mem::size_of::<trayclient>() as usize)
                    as *mut trayclient;
            (*tc).win = client;
            (*tc).xe_version = xe_version;
            (*tc).mapped = false;
            (*tc).tailq.tqe_next = ::core::ptr::null_mut::<trayclient>();
            (*tc).tailq.tqe_prev = (*(*output_for_tray).trayclients).tqh_last;
            *(*(*output_for_tray).trayclients).tqh_last = tc as *mut trayclient;
            (*(*output_for_tray).trayclients).tqh_last = &raw mut (*tc).tailq.tqe_next;
            trayclient_update_class(tc);
            if map_it {
                if config.verbose {
                    printf(
                        c"[%s:%d] Mapping dock client\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        1031 as ::core::ffi::c_int,
                    );
                }
                xcb_map_window(xcb_connection, client);
            } else if config.verbose {
                printf(
                    c"[%s:%d] Not mapping dock client yet\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1034 as ::core::ffi::c_int,
                );
            }
            configure_trayclients();
            draw_bars(false);
        }
    }
}
unsafe fn handle_destroy_notify(event: *mut xcb_destroy_notify_event_t) {
    if config.verbose {
        printf(
            c"[%s:%d] DestroyNotify for window = %08x, event = %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1054 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    let mut client: *mut trayclient =
        trayclient_and_output_from_window((*event).window, &raw mut output);
    if client.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] WARNING: Could not find corresponding tray window.\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1059 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Removing tray client with window ID %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1063 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    if !(*client).tailq.tqe_next.is_null() {
        (*(*client).tailq.tqe_next).tailq.tqe_prev = (*client).tailq.tqe_prev;
    } else {
        (*(*output).trayclients).tqh_last = (*client).tailq.tqe_prev;
    }
    *(*client).tailq.tqe_prev = (*client).tailq.tqe_next;
    free((*client).class_class as *mut ::core::ffi::c_void);
    free((*client).class_instance as *mut ::core::ffi::c_void);
    free(client as *mut ::core::ffi::c_void);
    client = ::core::ptr::null_mut::<trayclient>();
    configure_trayclients();
    draw_bars(false);
}
unsafe fn handle_map_notify(event: *mut xcb_map_notify_event_t) {
    if config.verbose {
        printf(
            c"[%s:%d] MapNotify for window = %08x, event = %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1080 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let client: *mut trayclient = trayclient_from_window((*event).window);
    if client.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] WARNING: Could not find corresponding tray window.\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1084 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Tray client mapped (window ID %08x). Adjusting tray.\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1088 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    (*client).mapped = true;
    configure_trayclients();
    draw_bars(false);
}
unsafe fn handle_unmap_notify(event: *mut xcb_unmap_notify_event_t) {
    if config.verbose {
        printf(
            c"[%s:%d] UnmapNotify for window = %08x, event = %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1101 as ::core::ffi::c_int,
            (*event).window,
            (*event).event,
        );
    }
    let client: *mut trayclient = trayclient_from_window((*event).window);
    if client.is_null() {
        if config.verbose {
            printf(
                c"[%s:%d] WARNING: Could not find corresponding tray window.\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1105 as ::core::ffi::c_int,
            );
        }
        return;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Tray client unmapped (window ID %08x). Adjusting tray.\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1109 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    (*client).mapped = false;
    configure_trayclients();
    draw_bars(false);
}
unsafe fn handle_property_notify(event: *mut xcb_property_notify_event_t) {
    let mut replies = Replies::default();
    if config.verbose {
        printf(
            c"[%s:%d] PropertyNotify\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1122 as ::core::ffi::c_int,
        );
    }
    if (*event).atom == atoms[AtomIndex::XembedInfo as ::core::ffi::c_int as usize]
        && (*event).state as ::core::ffi::c_int == XCB_PROPERTY_NEW_VALUE as ::core::ffi::c_int
    {
        if config.verbose {
            printf(
                c"[%s:%d] xembed_info updated\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1126 as ::core::ffi::c_int,
            );
        }
        let client: *mut trayclient = trayclient_from_window((*event).window);
        if client.is_null() {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: PropertyNotify received for unknown window %08x\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1130 as ::core::ffi::c_int,
                (*event).window,
            );
            return;
        }
        let mut xembedc: xcb_get_property_cookie_t = xcb_get_property_cookie_t { sequence: 0 };
        xembedc = xcb_get_property_unchecked(
            xcb_connection,
            0 as u8,
            (*client).win,
            atoms[AtomIndex::XembedInfo as ::core::ffi::c_int as usize],
            XCB_GET_PROPERTY_TYPE_ANY as ::core::ffi::c_int as xcb_atom_t,
            0 as u32,
            (2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as u32,
        );
        let xembedr: *mut xcb_get_property_reply_t = replies.take(xcb_get_property_reply(
            xcb_connection,
            xembedc,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        ));
        if xembedr.is_null() || (*xembedr).length == 0 as u32 {
            if config.verbose {
                printf(
                    c"[%s:%d] xembed_info unset\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1147 as ::core::ffi::c_int,
                );
            }
            return;
        }
        if config.verbose {
            printf(
                c"[%s:%d] xembed format = %d, len = %d\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1151 as ::core::ffi::c_int,
                (*xembedr).format as ::core::ffi::c_int,
                (*xembedr).length,
            );
        }
        let Some(xembed) = property_bytes(xembedr).and_then(xembed_words) else {
            return;
        };
        if config.verbose {
            printf(
                c"[%s:%d] xembed version = %d\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1153 as ::core::ffi::c_int,
                xembed[0],
            );
        }
        if config.verbose {
            printf(
                c"[%s:%d] xembed flags = %d\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1154 as ::core::ffi::c_int,
                xembed[1],
            );
        }
        let map_it: bool = xembed[1] & XEMBED_MAPPED as u32 == XEMBED_MAPPED as u32;
        if config.verbose {
            printf(
                c"[%s:%d] map state now %d\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1156 as ::core::ffi::c_int,
                map_it as ::core::ffi::c_int,
            );
        }
        if (*client).mapped as ::core::ffi::c_int != 0 && !map_it {
            xcb_unmap_window(xcb_connection, (*client).win);
        } else if !(*client).mapped && map_it as ::core::ffi::c_int != 0 {
            xcb_map_window(xcb_connection, (*client).win);
        }
    } else if (*event).atom == XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t {
        let client_0: *mut trayclient = trayclient_from_window((*event).window);
        if !client_0.is_null() {
            trayclient_update_class(client_0);
        }
    }
}
unsafe fn handle_configuration_change(window: xcb_window_t) {
    let mut trayclient: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    output = (*outputs).slh_first as *mut i3_output;
    while !output.is_null() {
        if (*output).active {
            let mut clients: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            trayclient =
                *(*((*(*output).trayclients).tqh_last as *mut tc_head)).tqh_last as *mut trayclient;
            while !trayclient.is_null() {
                if (*trayclient).mapped {
                    clients += 1;
                    if !((*trayclient).win != window) {
                        let mut rect: xcb_rectangle_t = xcb_rectangle_t {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        };
                        rect.x = ((*output).rect.w
                            - clients * (icon_size + logical_px(config.tray_padding)))
                            as i16;
                        rect.y = logical_px(config.tray_padding) as i16;
                        rect.width = icon_size as u16;
                        rect.height = icon_size as u16;
                        if config.verbose {
                            printf(
                                c"[%s:%d] This is a tray window. x = %d\n".as_ptr(),
                                c"../i3bar/src/xcb.c".as_ptr(),
                                1203 as ::core::ffi::c_int,
                                rect.x as ::core::ffi::c_int,
                            );
                        }
                        fake_configure_notify(
                            xcb_connection,
                            rect,
                            window,
                            0 as ::core::ffi::c_int,
                        );
                        return;
                    }
                }
                trayclient =
                    *(*((*trayclient).tailq.tqe_prev as *mut tc_head)).tqh_last as *mut trayclient;
            }
        }
        output = (*output).slist.sle_next as *mut i3_output;
    }
    if config.verbose {
        printf(
            c"[%s:%d] WARNING: Could not find corresponding tray window.\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1209 as ::core::ffi::c_int,
        );
    }
}
unsafe fn handle_configure_request(event: *mut xcb_configure_request_event_t) {
    if config.verbose {
        printf(
            c"[%s:%d] ConfigureRequest for window = %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1213 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    handle_configuration_change((*event).window);
}
unsafe fn handle_resize_request(event: *mut xcb_resize_request_event_t) {
    if config.verbose {
        printf(
            c"[%s:%d] ResizeRequest for window = %08x\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1218 as ::core::ffi::c_int,
            (*event).window,
        );
    }
    handle_configuration_change((*event).window);
}
unsafe extern "C" fn xcb_prep_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_prepare,
    revents: ::core::ffi::c_int,
) {
    if xcb_connection_has_error(xcb_connection) != 0 {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: X11 connection was closed unexpectedly - maybe your X server terminated / crashed?\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1231 as ::core::ffi::c_int,
        );
        exit(1 as ::core::ffi::c_int);
    }
    loop {
        let Some(event_owner) = Native::take(xcb_poll_for_event(xcb_connection)) else {
            break;
        };
        let event = event_owner.raw();
        if (*event).response_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let error: *mut xcb_generic_error_t = event as *mut xcb_generic_error_t;
            if config.verbose {
                printf(
                    c"[%s:%d] Received X11 error, sequence 0x%x, error_code = %d\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1241 as ::core::ffi::c_int,
                    (*error).sequence as ::core::ffi::c_int,
                    (*error).error_code as ::core::ffi::c_int,
                );
            }
        } else {
            let type_0: ::core::ffi::c_int =
                (*event).response_type as ::core::ffi::c_int & !(0x80 as ::core::ffi::c_int);
            if type_0 == xkb_base && xkb_base > -(1 as ::core::ffi::c_int) {
                if config.verbose {
                    printf(
                        c"[%s:%d] received an xkb event\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        1249 as ::core::ffi::c_int,
                    );
                }
                let state: *mut xcb_xkb_state_notify_event_t =
                    event as *mut xcb_xkb_state_notify_event_t;
                let mod_0: u32 = config.modifier & 0xffff as u32;
                let new_mod_pressed: bool =
                    mod_0 != 0 as u32 && (*state).mods as u32 & mod_0 == mod_0;
                if new_mod_pressed as ::core::ffi::c_int != mod_pressed as ::core::ffi::c_int {
                    mod_pressed = new_mod_pressed;
                    if (*state).xkbType as ::core::ffi::c_int == XCB_XKB_STATE_NOTIFY
                        && config.modifier as ::core::ffi::c_long != XCB_NONE
                    {
                        if mod_pressed {
                            activated_mode = false;
                            unhide_bars();
                        } else if !activated_mode {
                            hide_bars();
                        }
                    }
                }
            } else {
                match type_0 {
                    XCB_VISIBILITY_NOTIFY => {
                        handle_visibility_notify(event as *mut xcb_visibility_notify_event_t);
                    }
                    XCB_EXPOSE => {
                        if (*(event as *mut xcb_expose_event_t)).count as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            redraw_bars();
                        }
                    }
                    XCB_BUTTON_RELEASE | XCB_BUTTON_PRESS => {
                        handle_button(event as *mut xcb_button_press_event_t);
                    }
                    XCB_CLIENT_MESSAGE => {
                        handle_client_message(event as *mut xcb_client_message_event_t);
                    }
                    XCB_DESTROY_NOTIFY => {
                        handle_destroy_notify(event as *mut xcb_destroy_notify_event_t);
                    }
                    XCB_UNMAP_NOTIFY => {
                        handle_unmap_notify(event as *mut xcb_unmap_notify_event_t);
                    }
                    XCB_MAP_NOTIFY => {
                        handle_map_notify(event as *mut xcb_map_notify_event_t);
                    }
                    XCB_PROPERTY_NOTIFY => {
                        handle_property_notify(event as *mut xcb_property_notify_event_t);
                    }
                    XCB_CONFIGURE_REQUEST => {
                        handle_configure_request(event as *mut xcb_configure_request_event_t);
                    }
                    XCB_RESIZE_REQUEST => {
                        handle_resize_request(event as *mut xcb_resize_request_event_t);
                    }
                    _ => {}
                }
            }
        }
    }
    xcb_flush(xcb_connection);
}
unsafe extern "C" fn xcb_io_cb(
    loop_0: *mut ev_loop,
    watcher: *mut ev_io,
    revents: ::core::ffi::c_int,
) {
}

pub unsafe fn init_xcb_early() -> *mut ::core::ffi::c_char {
    xcb_connection = xcb_connect(::core::ptr::null::<::core::ffi::c_char>(), &raw mut screen);
    if xcb_connection_has_error(xcb_connection) != 0 {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Cannot open display\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1339 as ::core::ffi::c_int,
        );
        exit(libc::EXIT_FAILURE);
    }
    conn = xcb_connection;
    if config.verbose {
        printf(
            c"[%s:%d] Connected to xcb\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1343 as ::core::ffi::c_int,
        );
    }
    for (index, name) in ATOM_NAMES.iter().enumerate() {
        atom_cookies[index] = xcb_intern_atom(
            xcb_connection,
            0,
            name.to_bytes().len() as u16,
            name.as_ptr(),
        );
    }
    root_screen = xcb_aux_get_screen(xcb_connection, screen);
    xcb_root = (*root_screen).root;
    depth = (*root_screen).root_depth;
    colormap = (*root_screen).default_colormap;
    visual_type = if config.transparency as ::core::ffi::c_int != 0 {
        xcb_aux_find_visual_by_attrs(root_screen, -(1 as ::core::ffi::c_int) as i8, 32 as i8)
    } else {
        ::core::ptr::null_mut::<xcb_visualtype_t>()
    };
    if !visual_type.is_null() {
        depth = xcb_aux_get_depth_of_visual(root_screen, (*visual_type).visual_id);
        colormap = xcb_generate_id(xcb_connection) as xcb_colormap_t;
        let cm_cookie: xcb_void_cookie_t = xcb_create_colormap_checked(
            xcb_connection,
            XCB_COLORMAP_ALLOC_NONE as ::core::ffi::c_int as u8,
            colormap,
            xcb_root,
            (*visual_type).visual_id,
        );
        if _xcb_request_failed(
            cm_cookie,
            c"Could not allocate colormap".as_ptr().cast_mut(),
            1363 as ::core::ffi::c_int,
        ) != 0
        {
            exit(libc::EXIT_FAILURE);
        }
    } else {
        visual_type = get_visualtype(root_screen);
    }
    let mut cursor_ctx: *mut xcb_cursor_context_t = ::core::ptr::null_mut::<xcb_cursor_context_t>();
    if xcb_cursor_context_new(conn, root_screen, &raw mut cursor_ctx) < 0 as ::core::ffi::c_int {
        errx(
            libc::EXIT_FAILURE,
            c"Cannot allocate xcursor context".as_ptr(),
        );
    }
    cursor = xcb_cursor_load_cursor(cursor_ctx, c"left_ptr".as_ptr());
    xcb_cursor_context_free(cursor_ctx);
    xcb_io = smalloc(std::mem::size_of::<ev_io>()).cast();
    xcb_io.write(ev_io::new(
        xcb_get_file_descriptor(xcb_connection),
        EV_READ,
        xcb_io_cb,
    ));
    xcb_prep = smalloc(std::mem::size_of::<ev_prepare>()).cast();
    xcb_prep.write(ev_prepare::new(xcb_prep_cb));
    ev_io_start(main_loop as *mut ev_loop, xcb_io);
    ev_prepare_start(main_loop as *mut ev_loop, xcb_prep);
    get_atoms();
    let path: *mut ::core::ffi::c_char =
        root_atom_contents(c"I3_SOCKET_PATH".as_ptr(), xcb_connection, screen);
    path
}
unsafe fn register_xkb_keyevents() {
    let mut extreply: *const xcb_query_extension_reply_t =
        ::core::ptr::null::<xcb_query_extension_reply_t>();
    extreply =
        xcb_get_extension_data(conn, &raw mut xcb_xkb_id) as *const xcb_query_extension_reply_t;
    if (*extreply).present == 0 {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: xkb is not present on this server\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1405 as ::core::ffi::c_int,
        );
        exit(libc::EXIT_FAILURE);
    }
    if config.verbose {
        printf(
            c"[%s:%d] initializing xcb-xkb\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1408 as ::core::ffi::c_int,
        );
    }
    xcb_xkb_use_extension(
        conn,
        XCB_XKB_MAJOR_VERSION as u16,
        XCB_XKB_MINOR_VERSION as u16,
    );
    xcb_xkb_select_events(
        conn,
        XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
        XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int as u16,
        0 as u16,
        XCB_XKB_EVENT_TYPE_STATE_NOTIFY as ::core::ffi::c_int as u16,
        0xff as u16,
        0xff as u16,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
    xkb_base = (*extreply).first_event as ::core::ffi::c_int;
}
unsafe fn deregister_xkb_keyevents() {
    xcb_xkb_select_events(
        conn,
        XCB_XKB_ID_USE_CORE_KBD as ::core::ffi::c_int as xcb_xkb_device_spec_t,
        0 as u16,
        0 as u16,
        0 as u16,
        0xff as u16,
        0xff as u16,
        ::core::ptr::null::<::core::ffi::c_void>(),
    );
}

pub unsafe fn init_xcb_late(mut fontname: *mut ::core::ffi::c_char) {
    if fontname.is_null() {
        fontname = c"-misc-fixed-medium-r-normal--13-120-75-75-C-70-iso10646-1"
            .as_ptr()
            .cast_mut();
    }
    font = load_font(fontname, true);
    set_font(&raw mut font);
    if config.verbose {
        printf(
            c"[%s:%d] Calculated font-height: %d\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1449 as ::core::ffi::c_int,
            font.height,
        );
    }
    let default_px: ::core::ffi::c_int =
        font.height + 2 as ::core::ffi::c_int * logical_px(ws_voff_px) as ::core::ffi::c_int;
    let mut padding_scaled: ::core::ffi::c_int = logical_px(config.padding.y as ::core::ffi::c_int)
        + logical_px(config.padding.height as ::core::ffi::c_int);
    if config.bar_height > 0 as u32
        && config.padding.x == 0 as u32
        && config.padding.y == 0 as u32
        && config.padding.width == 0 as u32
        && config.padding.height == 0 as u32
    {
        padding_scaled = config.bar_height.wrapping_sub(default_px as u32) as ::core::ffi::c_int;
        if config.verbose {
            printf(
                c"[%s:%d] setting padding_scaled=%d based on bar_height=%d\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1465 as ::core::ffi::c_int,
                padding_scaled,
                config.bar_height,
            );
        }
    } else if config.verbose {
        printf(
            c"[%s:%d] padding: x=%d, y=%d -> padding_scaled=%d\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1467 as ::core::ffi::c_int,
            config.padding.x,
            config.padding.y,
            padding_scaled,
        );
    }
    bar_height = default_px + padding_scaled;
    icon_size = bar_height - 2 as ::core::ffi::c_int * logical_px(config.tray_padding);
    if !config.separator_symbol.is_null() {
        separator_symbol_width = predict_text_width(config.separator_symbol);
    }
    xcb_flush(xcb_connection);
    if config.hide_on_modifier == crate::application::BarDisplayMode::Hide {
        register_xkb_keyevents();
    }
}
unsafe fn send_tray_clientmessage() {
    let packet = client_message(
        xcb_root,
        atoms[AtomIndex::Manager as usize],
        [XCB_CURRENT_TIME as u32, (*tray_reply).atom, selwin, 0, 0],
    );
    xcb_send_event(
        xcb_connection,
        0,
        xcb_root,
        0xffffff,
        (&raw const packet).cast(),
    );
}

unsafe fn init_tray() {
    let mut replies = Replies::default();
    if config.verbose {
        printf(
            c"[%s:%d] Initializing system tray functionality\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1514 as ::core::ffi::c_int,
        );
    }
    let systray_len: usize =
        (strlen(c"_NET_SYSTEM_TRAY_S".as_ptr()) as usize).wrapping_add(11 as usize);
    let vla = systray_len as usize;
    let mut atomname: Vec<::core::ffi::c_char> = ::std::vec::from_elem(0, vla);
    snprintf(
        atomname.as_mut_ptr(),
        systray_len,
        c"_NET_SYSTEM_TRAY_S%d".as_ptr(),
        screen,
    );
    let mut tray_cookie: xcb_intern_atom_cookie_t = xcb_intern_atom_cookie_t { sequence: 0 };
    if tray_reply.is_null() {
        tray_cookie = xcb_intern_atom(
            xcb_connection,
            0 as u8,
            strlen(atomname.as_mut_ptr()) as u16,
            atomname.as_mut_ptr(),
        );
    }
    selwin = xcb_generate_id(xcb_connection) as xcb_window_t;
    let selmask: u32 = (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
        | XCB_CW_BORDER_PIXEL as ::core::ffi::c_int
        | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
        | XCB_CW_COLORMAP as ::core::ffi::c_int) as u32;
    let mut selval: [u32; 4] = [
        (*root_screen).black_pixel,
        (*root_screen).black_pixel,
        1 as ::core::ffi::c_int as u32,
        colormap,
    ];
    xcb_create_window(
        xcb_connection,
        depth,
        selwin,
        xcb_root,
        -(1 as ::core::ffi::c_int) as i16,
        -(1 as ::core::ffi::c_int) as i16,
        1 as u16,
        1 as u16,
        0 as u16,
        XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
        (*visual_type).visual_id,
        selmask,
        &raw mut selval as *mut u32 as *const ::core::ffi::c_void,
    );
    let mut orientation: u32 = _NET_SYSTEM_TRAY_ORIENTATION_HORZ as u32;
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        selwin,
        atoms[AtomIndex::NetSystemTrayOrientation as ::core::ffi::c_int as usize],
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut orientation as *const ::core::ffi::c_void,
    );
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        selwin,
        atoms[AtomIndex::NetSystemTrayVisual as ::core::ffi::c_int as usize],
        XCB_ATOM_VISUALID as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        1 as u32,
        &raw mut (*visual_type).visual_id as *const ::core::ffi::c_void,
    );
    init_tray_colors();
    if tray_reply.is_null() {
        tray_reply = xcb_intern_atom_reply(
            xcb_connection,
            tray_cookie,
            ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
        );
        if tray_reply.is_null() {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not get atom %s\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                1565 as ::core::ffi::c_int,
                atomname.as_mut_ptr(),
            );
            exit(libc::EXIT_FAILURE);
        }
    }
    xcb_set_selection_owner(
        xcb_connection,
        selwin,
        (*tray_reply).atom,
        XCB_CURRENT_TIME as xcb_timestamp_t,
    );
    let mut selcookie: xcb_get_selection_owner_cookie_t =
        xcb_get_selection_owner_cookie_t { sequence: 0 };
    let mut selreply: *mut xcb_get_selection_owner_reply_t =
        ::core::ptr::null_mut::<xcb_get_selection_owner_reply_t>();
    selcookie = xcb_get_selection_owner(xcb_connection, (*tray_reply).atom);
    selreply = replies.take(xcb_get_selection_owner_reply(
        xcb_connection,
        selcookie,
        ::core::ptr::null_mut::<*mut xcb_generic_error_t>(),
    ));
    if selreply.is_null() {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Could not get selection owner for %s\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1581 as ::core::ffi::c_int,
            atomname.as_mut_ptr(),
        );
        exit(libc::EXIT_FAILURE);
    }
    if (*selreply).owner != selwin {
        fprintf(
            stderr,
            c"[%s:%d] ERROR: Could not set the %s selection. Maybe another tray is already running?\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1588 as ::core::ffi::c_int,
            atomname.as_mut_ptr(),
        );
        return;
    }
    send_tray_clientmessage();
}

pub unsafe fn init_tray_colors() {
    let bar_fg: *const ::core::ffi::c_char = if !config.colors.bar_fg.is_null() {
        config.colors.bar_fg as *const ::core::ffi::c_char
    } else {
        c"#FFFFFF".as_ptr()
    };
    if config.verbose {
        printf(
            c"[%s:%d] Setting bar_fg = %s as _NET_SYSTEM_TRAY_COLORS\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1611 as ::core::ffi::c_int,
            bar_fg,
        );
    }
    let mut strgroups: [[::core::ffi::c_char; 3]; 3] = [
        [
            *bar_fg.offset(1 as ::core::ffi::c_int as isize),
            *bar_fg.offset(2 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *bar_fg.offset(3 as ::core::ffi::c_int as isize),
            *bar_fg.offset(4 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
        [
            *bar_fg.offset(5 as ::core::ffi::c_int as isize),
            *bar_fg.offset(6 as ::core::ffi::c_int as isize),
            '\0' as i32 as ::core::ffi::c_char,
        ],
    ];
    let r: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let g: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let b: u8 = strtol(
        &raw mut *(&raw mut strgroups as *mut [::core::ffi::c_char; 3])
            .offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        16 as ::core::ffi::c_int,
    ) as u8;
    let r16: u16 = ((r as u16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | r as ::core::ffi::c_int) as u16;
    let g16: u16 = ((g as u16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | g as ::core::ffi::c_int) as u16;
    let b16: u16 = ((b as u16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
        | b as ::core::ffi::c_int) as u16;
    let tray_colors: [u32; 12] = [
        r16 as u32, g16 as u32, b16 as u32, r16 as u32, g16 as u32, b16 as u32, r16 as u32,
        g16 as u32, b16 as u32, r16 as u32, g16 as u32, b16 as u32,
    ];
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
        selwin,
        atoms[AtomIndex::NetSystemTrayColors as ::core::ffi::c_int as usize],
        XCB_ATOM_CARDINAL as ::core::ffi::c_int as xcb_atom_t,
        32 as u8,
        12 as u32,
        &raw const tray_colors as *const u32 as *const ::core::ffi::c_void,
    );
}

pub unsafe fn clean_xcb() {
    if xcb_connection.is_null() {
        return;
    }
    free_outputs();
    free_font();
    xcb_free_cursor(xcb_connection, cursor);
    xcb_aux_sync(xcb_connection);
    if !xcb_prep.is_null() {
        ev_prepare_stop(main_loop, xcb_prep);
        free(xcb_prep.cast());
        xcb_prep = std::ptr::null_mut();
    }
    if !xcb_io.is_null() {
        ev_io_stop(main_loop, xcb_io);
        free(xcb_io.cast());
        xcb_io = std::ptr::null_mut();
    }
    if !tray_reply.is_null() {
        free(tray_reply.cast());
        tray_reply = std::ptr::null_mut();
    }
    let connection = xcb_connection;
    root_screen = std::ptr::null_mut();
    visual_type = std::ptr::null_mut();
    conn = std::ptr::null_mut();
    xcb_connection = std::ptr::null_mut();
    xcb_disconnect(connection);
}

pub unsafe fn get_atoms() {
    for (index, name) in ATOM_NAMES.iter().enumerate() {
        let Some(reply) = Native::take(xcb_intern_atom_reply(
            xcb_connection,
            atom_cookies[index],
            std::ptr::null_mut(),
        )) else {
            fprintf(
                stderr,
                c"[%s:%d] ERROR: Could not get atom %s\n".as_ptr(),
                c"../i3bar/include/xcb_atoms.def".as_ptr(),
                index as i32 + 1,
                name.as_ptr(),
            );
            exit(libc::EXIT_FAILURE);
        };
        atoms[index] = (*reply.raw()).atom;
    }
    if config.verbose {
        printf(
            c"[%s:%d] Got atoms\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            1678 as ::core::ffi::c_int,
        );
    }
}

pub unsafe fn kick_tray_clients(output: *mut i3_output) {
    if (*(*output).trayclients).tqh_first.is_null() {
        return;
    }
    let mut trayclient: *mut trayclient = ::core::ptr::null_mut::<trayclient>();
    while !(*(*output).trayclients).tqh_first.is_null() {
        trayclient = (*(*output).trayclients).tqh_first as *mut trayclient;
        xcb_unmap_window(xcb_connection, (*trayclient).win);
        xcb_reparent_window(
            xcb_connection,
            (*trayclient).win,
            xcb_root,
            0 as i16,
            0 as i16,
        );
        free((*trayclient).class_class as *mut ::core::ffi::c_void);
        free((*trayclient).class_instance as *mut ::core::ffi::c_void);
        if !(*trayclient).tailq.tqe_next.is_null() {
            (*(*trayclient).tailq.tqe_next).tailq.tqe_prev = (*trayclient).tailq.tqe_prev;
        } else {
            (*(*output).trayclients).tqh_last = (*trayclient).tailq.tqe_prev;
        }
        *(*trayclient).tailq.tqe_prev = (*trayclient).tailq.tqe_next;
        free(trayclient as *mut ::core::ffi::c_void);
        trayclient = ::core::ptr::null_mut::<trayclient>();
    }
    let packet = destroy_message(selwin);
    xcb_send_event(
        conn,
        0,
        selwin,
        XCB_EVENT_MASK_STRUCTURE_NOTIFY as u32,
        (&raw const packet).cast(),
    );
    send_tray_clientmessage();
}

pub unsafe fn destroy_window(output: *mut i3_output) {
    if output.is_null() || (*output).bar.id as ::core::ffi::c_long == XCB_NONE {
        return;
    }
    kick_tray_clients(output);
    draw_util_surface_free(xcb_connection, &raw mut (*output).bar);
    draw_util_surface_free(xcb_connection, &raw mut (*output).buffer);
    draw_util_surface_free(xcb_connection, &raw mut (*output).statusline_buffer);
    xcb_destroy_window(xcb_connection, (*output).bar.id as xcb_window_t);
    xcb_free_pixmap(xcb_connection, (*output).buffer.id as xcb_pixmap_t);
    xcb_free_pixmap(
        xcb_connection,
        (*output).statusline_buffer.id as xcb_pixmap_t,
    );
    (*output).bar.id = XCB_NONE as xcb_drawable_t;
}
unsafe fn config_strut_partial(output: *mut i3_output) -> xcb_void_cookie_t {
    let values = strut_values(config.position, bar_height, (*output).rect);
    xcb_change_property(
        xcb_connection,
        XCB_PROP_MODE_REPLACE as u8,
        (*output).bar.id,
        atoms[AtomIndex::NetWmStrutPartial as usize],
        XCB_ATOM_CARDINAL,
        32,
        12,
        values.as_ptr().cast(),
    )
}

unsafe fn get_tray_output() -> *mut i3_output {
    let mut output: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    if config.tray_outputs.tqh_first.is_null() {
        output = (*outputs).slh_first as *mut i3_output;
        while !output.is_null() {
            if (*output).active {
                return output;
            }
            output = (*output).slist.sle_next as *mut i3_output;
        }
        return ::core::ptr::null_mut::<i3_output>();
    } else if strcasecmp(
        (*config.tray_outputs.tqh_first).output_ptr(),
        c"none".as_ptr(),
    ) == 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null_mut::<i3_output>();
    }
    let mut tray_output: *mut tray_output_t = ::core::ptr::null_mut::<tray_output_t>();
    tray_output = config.tray_outputs.tqh_first as *mut tray_output_t;
    while !tray_output.is_null() {
        output = (*outputs).slh_first as *mut i3_output;
        while !output.is_null() {
            if (*output).active as ::core::ffi::c_int != 0
                && (strcasecmp((*output).name_ptr(), (*tray_output).output_ptr())
                    == 0 as ::core::ffi::c_int
                    || strcasecmp((*tray_output).output_ptr(), c"primary".as_ptr())
                        == 0 as ::core::ffi::c_int
                        && (*output).primary as ::core::ffi::c_int != 0)
            {
                return output;
            }
            output = (*output).slist.sle_next as *mut i3_output;
        }
        tray_output = (*tray_output).tray_outputs.tqe_next as *mut tray_output_t;
    }
    ::core::ptr::null_mut::<i3_output>()
}

pub unsafe fn reconfig_windows(redraw_bars_0: bool) {
    let mut mask: u32 = 0;
    let mut values: [u32; 6] = [0; 6];
    let mut walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    walk = (*outputs).slh_first as *mut i3_output;
    while !walk.is_null() {
        if !(*walk).active {
            if config.verbose {
                printf(
                    c"[%s:%d] Destroying window for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1857 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            destroy_window(walk);
        } else if (*walk).bar.id as ::core::ffi::c_long == XCB_NONE {
            if config.verbose {
                printf(
                    c"[%s:%d] Creating window for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1862 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            let bar_id: xcb_window_t = xcb_generate_id(xcb_connection) as xcb_window_t;
            let buffer_id: xcb_pixmap_t = xcb_generate_id(xcb_connection) as xcb_pixmap_t;
            let statusline_buffer_id: xcb_pixmap_t =
                xcb_generate_id(xcb_connection) as xcb_pixmap_t;
            mask = (XCB_CW_BACK_PIXEL as ::core::ffi::c_int
                | XCB_CW_BORDER_PIXEL as ::core::ffi::c_int
                | XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int
                | XCB_CW_EVENT_MASK as ::core::ffi::c_int
                | XCB_CW_COLORMAP as ::core::ffi::c_int
                | XCB_CW_CURSOR as ::core::ffi::c_int) as u32;
            values[0 as ::core::ffi::c_int as usize] = colors.bar_bg.colorpixel;
            values[1 as ::core::ffi::c_int as usize] = (*root_screen).black_pixel;
            values[2 as ::core::ffi::c_int as usize] =
                (if config.hide_on_modifier == crate::application::BarDisplayMode::Dock {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as u32;
            values[3 as ::core::ffi::c_int as usize] =
                (XCB_EVENT_MASK_EXPOSURE as ::core::ffi::c_int
                    | XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT as ::core::ffi::c_int
                    | XCB_EVENT_MASK_BUTTON_PRESS as ::core::ffi::c_int
                    | XCB_EVENT_MASK_BUTTON_RELEASE as ::core::ffi::c_int) as u32;
            if config.hide_on_modifier == crate::application::BarDisplayMode::Dock {
                values[3 as ::core::ffi::c_int as usize] |=
                    XCB_EVENT_MASK_VISIBILITY_CHANGE as ::core::ffi::c_int as u32;
                (*walk).visible = true;
            }
            values[4 as ::core::ffi::c_int as usize] = colormap as u32;
            values[5 as ::core::ffi::c_int as usize] = cursor as u32;
            let win_cookie: xcb_void_cookie_t = xcb_create_window_checked(
                xcb_connection,
                depth,
                bar_id,
                xcb_root,
                (*walk).rect.x as i16,
                ((*walk).rect.y + (*walk).rect.h - bar_height) as i16,
                (*walk).rect.w as u16,
                bar_height as u16,
                0 as u16,
                XCB_WINDOW_CLASS_INPUT_OUTPUT as ::core::ffi::c_int as u16,
                (*visual_type).visual_id,
                mask,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            let pm_cookie: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                buffer_id,
                bar_id as xcb_drawable_t,
                (*walk).rect.w as u16,
                bar_height as u16,
            );
            let slpm_cookie: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                statusline_buffer_id,
                bar_id as xcb_drawable_t,
                (*walk).rect.w as u16,
                bar_height as u16,
            );
            let mut class: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let len: ::core::ffi::c_int = sasprintf(
                &raw mut class,
                c"%s%ci3bar%c".as_ptr(),
                config.bar_id,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            let mut class_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            class_cookie = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                bar_id,
                XCB_ATOM_WM_CLASS as ::core::ffi::c_int as xcb_atom_t,
                XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
                8 as u8,
                len as u32,
                class as *const ::core::ffi::c_void,
            );
            free(class as *mut ::core::ffi::c_void);
            let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            sasprintf(
                &raw mut name,
                c"i3bar for output %s".as_ptr(),
                (*walk).name_ptr(),
            );
            let mut name_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            name_cookie = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                bar_id,
                XCB_ATOM_WM_NAME as ::core::ffi::c_int as xcb_atom_t,
                XCB_ATOM_STRING as ::core::ffi::c_int as xcb_atom_t,
                8 as u8,
                strlen(name) as u32,
                name as *const ::core::ffi::c_void,
            );
            free(name as *mut ::core::ffi::c_void);
            let dock_cookie: xcb_void_cookie_t = xcb_change_property(
                xcb_connection,
                XCB_PROP_MODE_REPLACE as ::core::ffi::c_int as u8,
                bar_id,
                atoms[AtomIndex::NetWmWindowType as ::core::ffi::c_int as usize],
                XCB_ATOM_ATOM as ::core::ffi::c_int as xcb_atom_t,
                32 as u8,
                1 as u32,
                (&raw mut atoms as *mut xcb_atom_t)
                    .offset(AtomIndex::NetWmWindowTypeDock as ::core::ffi::c_int as isize)
                    as *mut xcb_atom_t as *mut ::core::ffi::c_uchar
                    as *const ::core::ffi::c_void,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).bar,
                bar_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).buffer,
                buffer_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).statusline_buffer,
                statusline_buffer_id as xcb_drawable_t,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            let strut_cookie: xcb_void_cookie_t = config_strut_partial(walk);
            let mut map_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            if config.hide_on_modifier == crate::application::BarDisplayMode::Dock {
                map_cookie = xcb_map_window_checked(xcb_connection, bar_id);
            }
            if _xcb_request_failed(
                win_cookie,
                c"Could not create window".as_ptr().cast_mut(),
                1970 as ::core::ffi::c_int,
            ) != 0
                || _xcb_request_failed(
                    pm_cookie,
                    c"Could not create pixmap".as_ptr().cast_mut(),
                    1971 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    slpm_cookie,
                    c"Could not create statusline pixmap".as_ptr().cast_mut(),
                    1972 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    dock_cookie,
                    c"Could not set dock mode".as_ptr().cast_mut(),
                    1973 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    class_cookie,
                    c"Could not set WM_CLASS".as_ptr().cast_mut(),
                    1974 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    name_cookie,
                    c"Could not set WM_NAME".as_ptr().cast_mut(),
                    1975 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    strut_cookie,
                    c"Could not set strut".as_ptr().cast_mut(),
                    1976 as ::core::ffi::c_int,
                ) != 0
                || config.hide_on_modifier == crate::application::BarDisplayMode::Dock
                    && _xcb_request_failed(
                        map_cookie,
                        c"Could not map window".as_ptr().cast_mut(),
                        1977 as ::core::ffi::c_int,
                    ) != 0
            {
                exit(libc::EXIT_FAILURE);
            }
        } else {
            mask = (XCB_CONFIG_WINDOW_X as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_Y as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_WIDTH as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_HEIGHT as ::core::ffi::c_int
                | XCB_CONFIG_WINDOW_STACK_MODE as ::core::ffi::c_int) as u32;
            values[0 as ::core::ffi::c_int as usize] = (*walk).rect.x as u32;
            if config.position == crate::application::BarPosition::Top {
                values[1 as ::core::ffi::c_int as usize] = (*walk).rect.y as u32;
            } else {
                values[1 as ::core::ffi::c_int as usize] =
                    ((*walk).rect.y + (*walk).rect.h - bar_height) as u32;
            }
            values[2 as ::core::ffi::c_int as usize] = (*walk).rect.w as u32;
            values[3 as ::core::ffi::c_int as usize] = bar_height as u32;
            values[4 as ::core::ffi::c_int as usize] =
                XCB_STACK_MODE_ABOVE as ::core::ffi::c_int as u32;
            if config.verbose {
                printf(
                    c"[%s:%d] Reconfiguring strut partial property for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    1998 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            let strut_cookie_0: xcb_void_cookie_t = config_strut_partial(walk);
            if config.verbose {
                printf(
                    c"[%s:%d] Destroying buffer for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2001 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            xcb_free_pixmap(xcb_connection, (*walk).buffer.id as xcb_pixmap_t);
            if config.verbose {
                printf(
                    c"[%s:%d] Destroying statusline buffer for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2004 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            xcb_free_pixmap(xcb_connection, (*walk).statusline_buffer.id as xcb_pixmap_t);
            if config.verbose {
                printf(
                    c"[%s:%d] Reconfiguring window for output %s to %d,%d\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2007 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                    values[0 as ::core::ffi::c_int as usize],
                    values[1 as ::core::ffi::c_int as usize],
                );
            }
            let cfg_cookie: xcb_void_cookie_t = xcb_configure_window_checked(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask as u16,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            mask = XCB_CW_OVERRIDE_REDIRECT as ::core::ffi::c_int as u32;
            values[0 as ::core::ffi::c_int as usize] =
                (if config.hide_on_modifier == crate::application::BarDisplayMode::Dock {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as u32;
            if config.verbose {
                printf(
                    c"[%s:%d] Changing window attribute override_redirect for output %s to %d\n"
                        .as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2015 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                    values[0 as ::core::ffi::c_int as usize],
                );
            }
            let chg_cookie: xcb_void_cookie_t = xcb_change_window_attributes(
                xcb_connection,
                (*walk).bar.id as xcb_window_t,
                mask,
                &raw mut values as *mut u32 as *const ::core::ffi::c_void,
            );
            if config.verbose {
                printf(
                    c"[%s:%d] Recreating buffer for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2021 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            let pm_cookie_0: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                (*walk).buffer.id as xcb_pixmap_t,
                (*walk).bar.id,
                (*walk).rect.w as u16,
                bar_height as u16,
            );
            if config.verbose {
                printf(
                    c"[%s:%d] Recreating statusline buffer for output %s\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2029 as ::core::ffi::c_int,
                    (*walk).name_ptr(),
                );
            }
            let slpm_cookie_0: xcb_void_cookie_t = xcb_create_pixmap_checked(
                xcb_connection,
                depth,
                (*walk).statusline_buffer.id as xcb_pixmap_t,
                (*walk).bar.id,
                (*walk).rect.w as u16,
                bar_height as u16,
            );
            draw_util_surface_free(xcb_connection, &raw mut (*walk).bar);
            draw_util_surface_free(xcb_connection, &raw mut (*walk).buffer);
            draw_util_surface_free(xcb_connection, &raw mut (*walk).statusline_buffer);
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).bar,
                (*walk).bar.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).buffer,
                (*walk).buffer.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            draw_util_surface_init(
                xcb_connection,
                &raw mut (*walk).statusline_buffer,
                (*walk).statusline_buffer.id,
                ::core::ptr::null_mut::<xcb_visualtype_t>(),
                (*walk).rect.w,
                bar_height,
            );
            let mut map_cookie_0: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            let mut umap_cookie: xcb_void_cookie_t = xcb_void_cookie_t { sequence: 0 };
            if redraw_bars_0 {
                umap_cookie =
                    xcb_unmap_window_checked(xcb_connection, (*walk).bar.id as xcb_window_t);
                if config.hide_on_modifier == crate::application::BarDisplayMode::Dock {
                    cont_children();
                    map_cookie_0 =
                        xcb_map_window_checked(xcb_connection, (*walk).bar.id as xcb_window_t);
                } else {
                    stop_children();
                }
                if config.hide_on_modifier == crate::application::BarDisplayMode::Hide {
                    register_xkb_keyevents();
                } else {
                    deregister_xkb_keyevents();
                }
            }
            if _xcb_request_failed(
                cfg_cookie,
                c"Could not reconfigure window".as_ptr().cast_mut(),
                2064 as ::core::ffi::c_int,
            ) != 0
                || _xcb_request_failed(
                    chg_cookie,
                    c"Could not change window".as_ptr().cast_mut(),
                    2065 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    pm_cookie_0,
                    c"Could not create pixmap".as_ptr().cast_mut(),
                    2066 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    slpm_cookie_0,
                    c"Could not create statusline pixmap".as_ptr().cast_mut(),
                    2067 as ::core::ffi::c_int,
                ) != 0
                || _xcb_request_failed(
                    strut_cookie_0,
                    c"Could not set strut".as_ptr().cast_mut(),
                    2068 as ::core::ffi::c_int,
                ) != 0
                || redraw_bars_0 as ::core::ffi::c_int != 0
                    && (_xcb_request_failed(
                        umap_cookie,
                        c"Could not unmap window".as_ptr().cast_mut(),
                        2069 as ::core::ffi::c_int,
                    ) != 0
                        || config.hide_on_modifier == crate::application::BarDisplayMode::Dock
                            && _xcb_request_failed(
                                map_cookie_0,
                                c"Could not map window".as_ptr().cast_mut(),
                                2070 as ::core::ffi::c_int,
                            ) != 0)
            {
                exit(libc::EXIT_FAILURE);
            }
        }
        walk = (*walk).slist.sle_next as *mut i3_output;
    }
    output_for_tray = get_tray_output();
    if !output_for_tray.is_null() {
        if selwin as ::core::ffi::c_long == XCB_NONE {
            init_tray();
        }
    } else if selwin as ::core::ffi::c_long != XCB_NONE {
        if config.verbose {
            printf(
                c"[%s:%d] Destroying tray selection window\n".as_ptr(),
                c"../i3bar/src/xcb.c".as_ptr(),
                2084 as ::core::ffi::c_int,
            );
        }
        xcb_destroy_window(xcb_connection, selwin);
        selwin = XCB_NONE as xcb_window_t;
    }
}
unsafe fn draw_button(
    surface: *mut surface_t,
    fg_color: color_t,
    bg_color: color_t,
    border_color: color_t,
    x: ::core::ffi::c_int,
    width: ::core::ffi::c_int,
    text_width: ::core::ffi::c_int,
    text: *mut I3String,
) {
    let height: ::core::ffi::c_int =
        bar_height - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int);
    draw_util_rectangle(
        surface,
        border_color,
        x as ::core::ffi::c_double,
        logical_px(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
        width as ::core::ffi::c_double,
        height as ::core::ffi::c_double,
    );
    draw_util_rectangle(
        surface,
        bg_color,
        (x + logical_px(1 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int)) as ::core::ffi::c_double,
        (width - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
        (height - 2 as ::core::ffi::c_int * logical_px(1 as ::core::ffi::c_int))
            as ::core::ffi::c_double,
    );
    draw_util_text(
        text,
        surface,
        fg_color,
        bg_color,
        x + (width - text_width) / 2 as ::core::ffi::c_int,
        bar_height / 2 as ::core::ffi::c_int - font.height / 2 as ::core::ffi::c_int,
        text_width,
    );
}

pub unsafe fn draw_bars(mut unhide: bool) {
    if config.verbose {
        printf(
            c"[%s:%d] Drawing bars...\n".as_ptr(),
            c"../i3bar/src/xcb.c".as_ptr(),
            2114 as ::core::ffi::c_int,
        );
    }
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        let mut workspace_width: ::core::ffi::c_int =
            logical_px(config.padding.x as ::core::ffi::c_int);
        if !(*outputs_walk).active {
            if config.verbose {
                printf(
                    c"[%s:%d] Output %s inactive, skipping...\n".as_ptr(),
                    c"../i3bar/src/xcb.c".as_ptr(),
                    2121 as ::core::ffi::c_int,
                    (*outputs_walk).name_ptr(),
                );
            }
        } else {
            if (*outputs_walk).bar.id as ::core::ffi::c_long == XCB_NONE {
                reconfig_windows(false);
            }
            let use_focus_colors: bool = output_has_focus(outputs_walk);
            draw_util_clear_surface(
                &raw mut (*outputs_walk).buffer,
                if use_focus_colors as ::core::ffi::c_int != 0 {
                    colors.focus_bar_bg
                } else {
                    colors.bar_bg
                },
            );
            if !config.disable_ws {
                let mut ws_walk: *mut i3_ws = ::core::ptr::null_mut::<i3_ws>();
                ws_walk = (*(*outputs_walk).workspaces).tqh_first as *mut i3_ws;
                while !ws_walk.is_null() {
                    if config.verbose {
                        printf(
                            c"[%s:%d] Drawing button for WS %s at x = %d, len = %d\n".as_ptr(),
                            c"../i3bar/src/xcb.c".as_ptr(),
                            2138 as ::core::ffi::c_int,
                            (*ws_walk)
                                .name
                                .as_cstr()
                                .expect("workspace text is UTF-8")
                                .as_ptr(),
                            workspace_width,
                            (*ws_walk).name_width,
                        );
                    }
                    let mut fg_color: color_t = colors.inactive_ws_fg;
                    let mut bg_color: color_t = colors.inactive_ws_bg;
                    let mut border_color: color_t = colors.inactive_ws_border;
                    if (*ws_walk).visible {
                        if !(*ws_walk).focused {
                            fg_color = colors.active_ws_fg;
                            bg_color = colors.active_ws_bg;
                            border_color = colors.active_ws_border;
                        } else {
                            fg_color = colors.focus_ws_fg;
                            bg_color = colors.focus_ws_bg;
                            border_color = colors.focus_ws_border;
                        }
                    }
                    if (*ws_walk).urgent {
                        if config.verbose {
                            printf(
                                c"[%s:%d] WS %s is urgent!\n".as_ptr(),
                                c"../i3bar/src/xcb.c".as_ptr(),
                                2154 as ::core::ffi::c_int,
                                (*ws_walk)
                                    .name
                                    .as_cstr()
                                    .expect("workspace text is UTF-8")
                                    .as_ptr(),
                            );
                        }
                        fg_color = colors.urgent_ws_fg;
                        bg_color = colors.urgent_ws_bg;
                        border_color = colors.urgent_ws_border;
                        unhide = true;
                    }
                    let w: ::core::ffi::c_int = predict_button_width((*ws_walk).name_width);
                    draw_button(
                        &raw mut (*outputs_walk).buffer,
                        fg_color,
                        bg_color,
                        border_color,
                        workspace_width,
                        w,
                        (*ws_walk).name_width,
                        (*ws_walk).name_ptr(),
                    );
                    workspace_width += w;
                    if !(*ws_walk).tailq.tqe_next.is_null() {
                        workspace_width += logical_px(ws_spacing_px);
                    }
                    ws_walk = (*ws_walk).tailq.tqe_next as *mut i3_ws;
                }
            }
            if let Some(mode_name) = binding
                .name
                .as_ref()
                .filter(|_| !config.disable_binding_mode_indicator)
            {
                workspace_width += logical_px(ws_spacing_px);
                let w_0: ::core::ffi::c_int = predict_button_width(binding.name_width);
                draw_button(
                    &raw mut (*outputs_walk).buffer,
                    colors.binding_mode_fg,
                    colors.binding_mode_bg,
                    colors.binding_mode_border,
                    workspace_width,
                    w_0,
                    binding.name_width,
                    (mode_name as *const I3String).cast_mut(),
                );
                unhide = true;
                workspace_width += w_0;
            }
            if !statusline_head.tqh_first.is_null() {
                if config.verbose {
                    printf(
                        c"[%s:%d] Printing statusline!\n".as_ptr(),
                        c"../i3bar/src/xcb.c".as_ptr(),
                        2184 as ::core::ffi::c_int,
                    );
                }
                let tray_width: ::core::ffi::c_int =
                    get_tray_width((*outputs_walk).trayclients as *mut tc_head);
                let hoff: u32 = logical_px(
                    ((workspace_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        + (tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
                        * sb_hoff_px,
                ) as u32;
                let max_statusline_width: u32 =
                    (((*outputs_walk).rect.w - workspace_width - tray_width) as u32)
                        .wrapping_sub(hoff);
                let mut clip_left: u32 = 0 as u32;
                let mut block: *mut status_block = ::core::ptr::null_mut::<status_block>();
                block = statusline_head.tqh_first;
                while !block.is_null() {
                    (*block).use_short = false;
                    block = (*block).blocks.tqe_next;
                }
                let statusline_width: u32 = adjust_statusline_length(max_statusline_width);
                if statusline_width > max_statusline_width {
                    clip_left = statusline_width.wrapping_sub(max_statusline_width);
                }
                let visible_statusline_width: i16 = (if statusline_width < max_statusline_width {
                    statusline_width
                } else {
                    max_statusline_width
                }) as i16;
                let mut x_dest: ::core::ffi::c_int = (*outputs_walk).rect.w
                    - tray_width
                    - logical_px(
                        (tray_width > 0 as ::core::ffi::c_int) as ::core::ffi::c_int * sb_hoff_px,
                    )
                    - visible_statusline_width as ::core::ffi::c_int;
                x_dest -= logical_px(config.padding.width as ::core::ffi::c_int);
                draw_statusline(outputs_walk, clip_left, use_focus_colors);
                draw_util_copy_surface(
                    &raw mut (*outputs_walk).statusline_buffer,
                    &raw mut (*outputs_walk).buffer,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    x_dest as ::core::ffi::c_double,
                    0 as ::core::ffi::c_int as ::core::ffi::c_double,
                    visible_statusline_width as ::core::ffi::c_double,
                    bar_height as i16 as ::core::ffi::c_double,
                );
                (*outputs_walk).statusline_width = statusline_width as ::core::ffi::c_int;
            }
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
    if mod_pressed as ::core::ffi::c_int != 0
        || config.hidden_state == crate::application::HiddenState::Show
        || unhide as ::core::ffi::c_int != 0
    {
        unhide_bars();
    } else if config.hide_on_modifier == crate::application::BarDisplayMode::Hide {
        hide_bars();
    }
    redraw_bars();
}

pub unsafe fn redraw_bars() {
    let mut outputs_walk: *mut i3_output = ::core::ptr::null_mut::<i3_output>();
    outputs_walk = (*outputs).slh_first as *mut i3_output;
    while !outputs_walk.is_null() {
        if (*outputs_walk).active {
            draw_util_copy_surface(
                &raw mut (*outputs_walk).buffer,
                &raw mut (*outputs_walk).bar,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                0 as ::core::ffi::c_int as ::core::ffi::c_double,
                (*outputs_walk).rect.w as ::core::ffi::c_double,
                (*outputs_walk).rect.h as ::core::ffi::c_double,
            );
            xcb_flush(xcb_connection);
        }
        outputs_walk = (*outputs_walk).slist.sle_next as *mut i3_output;
    }
}

/// Replace the event-thread's owned mode text, releasing the previous value.
///
/// # Safety
/// Call only on the bar event thread, with no outstanding borrow of binding.
pub unsafe fn set_current_mode(current: Mode) {
    activated_mode = current.name.is_some();
    binding = current;
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::child::child_want_click_events;
use crate::child::cont_children;
use crate::child::send_block_clicked;
use crate::child::stop_children;
use crate::ipc::i3_send_msg;
use crate::outputs::free_outputs;
use i3_common::dpi::logical_px;
use i3_common::font::free_font;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::sstrndup;
use i3_common::safewrappers::strcasecmp_nullable;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use xcb_sys::{
    xcb_atom_t, xcb_aux_find_visual_by_attrs, xcb_aux_get_depth_of_visual, xcb_aux_get_screen,
    xcb_aux_sync, xcb_button_press_event_t, xcb_change_property, xcb_change_save_set,
    xcb_change_window_attributes, xcb_charinfo_t, xcb_client_message_event_t, xcb_color_strings_t,
    xcb_colormap_t, xcb_configure_request_event_t, xcb_configure_window,
    xcb_configure_window_checked, xcb_connect, xcb_connection_has_error, xcb_connection_t,
    xcb_create_colormap_checked, xcb_create_pixmap_checked, xcb_create_window,
    xcb_create_window_checked, xcb_cursor_context_free, xcb_cursor_context_new,
    xcb_cursor_context_t, xcb_cursor_load_cursor, xcb_cursor_t, xcb_destroy_notify_event_t,
    xcb_destroy_window, xcb_disconnect, xcb_drawable_t, xcb_expose_event_t, xcb_flush,
    xcb_free_cursor, xcb_free_pixmap, xcb_generate_id, xcb_generic_error_t, xcb_generic_event_t,
    xcb_get_extension_data, xcb_get_file_descriptor, xcb_get_property, xcb_get_property_cookie_t,
    xcb_get_property_reply, xcb_get_property_reply_t, xcb_get_property_unchecked,
    xcb_get_property_value, xcb_get_property_value_length, xcb_get_selection_owner,
    xcb_get_selection_owner_cookie_t, xcb_get_selection_owner_reply,
    xcb_get_selection_owner_reply_t, xcb_intern_atom, xcb_intern_atom_cookie_t,
    xcb_intern_atom_reply, xcb_intern_atom_reply_t, xcb_keycode_t, xcb_map_notify_event_t,
    xcb_map_window, xcb_map_window_checked, xcb_pixmap_t, xcb_poll_for_event,
    xcb_property_notify_event_t, xcb_query_extension_reply_t, xcb_query_font_reply_t,
    xcb_rectangle_t, xcb_reparent_window, xcb_request_check, xcb_resize_request_event_t,
    xcb_screen_t, xcb_send_event, xcb_set_selection_owner, xcb_timestamp_t,
    xcb_unmap_notify_event_t, xcb_unmap_window, xcb_unmap_window_checked,
    xcb_visibility_notify_event_t, xcb_visualtype_t, xcb_void_cookie_t, xcb_window_t,
    xcb_xkb_device_spec_t, xcb_xkb_id, xcb_xkb_select_events, xcb_xkb_state_notify_event_t,
    xcb_xkb_use_extension, XCB_ATOM_ATOM, XCB_ATOM_CARDINAL, XCB_ATOM_STRING, XCB_ATOM_VISUALID,
    XCB_ATOM_WM_CLASS, XCB_ATOM_WM_NAME, XCB_BUTTON_PRESS, XCB_BUTTON_RELEASE,
    XCB_BUTTON_SCROLL_DOWN, XCB_BUTTON_SCROLL_LEFT, XCB_BUTTON_SCROLL_RIGHT, XCB_BUTTON_SCROLL_UP,
    XCB_CLIENT_MESSAGE, XCB_COLORMAP_ALLOC_NONE, XCB_CONFIGURE_REQUEST, XCB_CONFIG_WINDOW_HEIGHT,
    XCB_CONFIG_WINDOW_STACK_MODE, XCB_CONFIG_WINDOW_WIDTH, XCB_CONFIG_WINDOW_X,
    XCB_CONFIG_WINDOW_Y, XCB_CURRENT_TIME, XCB_CW_BACK_PIXEL, XCB_CW_BORDER_PIXEL, XCB_CW_COLORMAP,
    XCB_CW_CURSOR, XCB_CW_EVENT_MASK, XCB_CW_OVERRIDE_REDIRECT, XCB_DESTROY_NOTIFY,
    XCB_EVENT_MASK_BUTTON_PRESS, XCB_EVENT_MASK_BUTTON_RELEASE, XCB_EVENT_MASK_EXPOSURE,
    XCB_EVENT_MASK_NO_EVENT, XCB_EVENT_MASK_PROPERTY_CHANGE, XCB_EVENT_MASK_RESIZE_REDIRECT,
    XCB_EVENT_MASK_STRUCTURE_NOTIFY, XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT,
    XCB_EVENT_MASK_VISIBILITY_CHANGE, XCB_EXPOSE, XCB_GET_PROPERTY_TYPE_ANY, XCB_MAP_NOTIFY,
    XCB_NONE, XCB_PROPERTY_NEW_VALUE, XCB_PROPERTY_NOTIFY, XCB_PROP_MODE_REPLACE,
    XCB_RESIZE_REQUEST, XCB_SET_MODE_INSERT, XCB_STACK_MODE_ABOVE, XCB_UNMAP_NOTIFY,
    XCB_VISIBILITY_FULLY_OBSCURED, XCB_VISIBILITY_NOTIFY, XCB_WINDOW_CLASS_INPUT_OUTPUT,
    XCB_XKB_EVENT_TYPE_STATE_NOTIFY, XCB_XKB_ID_USE_CORE_KBD, XCB_XKB_MAJOR_VERSION,
    XCB_XKB_MINOR_VERSION, XCB_XKB_STATE_NOTIFY,
};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcb_colors_t {
    pub bar_fg: color_t,
    pub bar_bg: color_t,
    pub sep_fg: color_t,
    pub focus_bar_fg: color_t,
    pub focus_bar_bg: color_t,
    pub focus_sep_fg: color_t,
    pub active_ws_fg: color_t,
    pub active_ws_bg: color_t,
    pub active_ws_border: color_t,
    pub inactive_ws_fg: color_t,
    pub inactive_ws_bg: color_t,
    pub inactive_ws_border: color_t,
    pub urgent_ws_bg: color_t,
    pub urgent_ws_fg: color_t,
    pub urgent_ws_border: color_t,
    pub focus_ws_bg: color_t,
    pub focus_ws_fg: color_t,
    pub focus_ws_border: color_t,
    pub binding_mode_bg: color_t,
    pub binding_mode_fg: color_t,
    pub binding_mode_border: color_t,
}

pub use libc::{
    exit, fprintf, free, memcpy, memmove, memset, printf, qsort, snprintf, strcasecmp, strcmp,
    strlen, strnlen, strtol,
};

use i3_common::native_stdio::{errx, stderr};

// Internal calls and globals use canonical Rust identities.
use crate::application::main_loop;
use i3_common::fake_configure_notify::fake_configure_notify;
use i3_common::get_visualtype::get_visualtype;
use i3_common::root_atom_contents::root_atom_contents;

use i3_common::string::I3String;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AtomIndex {
    NetWmWindowType = 0,
    NetWmWindowTypeDock = 1,
    NetWmStrutPartial = 2,
    I3SocketPath = 3,
    Manager = 4,
    NetSystemTrayOrientation = 5,
    NetSystemTrayVisual = 6,
    NetSystemTrayOpcode = 7,
    NetSystemTrayColors = 8,
    XembedInfo = 9,
    Xembed = 10,
    I3Sync = 11,
}

impl AtomIndex {
    pub const COUNT: usize = 12;
}

#[cfg(test)]
mod owned_tests {
    use super::*;
    #[test]
    fn rendering_keeps_cached_alignment_offsets_separator_rounding_and_tray_arithmetic() {
        let previous = status_block_render_desc {
            width: 0,
            x_offset: 5,
            x_append: 7,
        };
        let left = render_dimensions(3, 10, crate::child::BlockAlign::Left, previous);
        assert_eq!((left.width, left.x_offset, left.x_append), (3, 5, 7));
        let right = render_dimensions(3, 10, crate::child::BlockAlign::Right, previous);
        assert_eq!((right.x_offset, right.x_append), (7, 7));
        let center = render_dimensions(3, 12, crate::child::BlockAlign::Center, previous);
        assert_eq!((center.x_offset, center.x_append), (4, 5));
        let full = render_dimensions(20, 10, crate::child::BlockAlign::Left, previous);
        assert_eq!((full.x_offset, full.x_append), (0, 0));
        assert_eq!(separator_offset(false, u32::MAX), 0x80000000);
        assert_eq!(separator_offset(true, 9), 0);
        assert_eq!(tray_width(3, 12, 2), 38);
        assert_eq!(tray_width(0, 12, 2), 0);
        assert_eq!(tray_width(3, -1, 2), -3);
    }
    #[test]
    fn quoting_and_class_bytes_preserve_non_utf8_escapes_and_first_nul() {
        let input = std::ffi::CStr::from_bytes_with_nul(b"raw\xff\"\\name\0").unwrap();
        assert_eq!(
            quote_workspace(input).to_bytes(),
            b"workspace \"raw\xff\\\"\\\\name\""
        );
        assert_eq!(
            class_parts(b"inst\0Class\0ignored"),
            (&b"inst"[..], Some(&b"Class"[..]))
        );
        assert_eq!(class_parts(b"inst"), (&b"inst"[..], None));
        assert_eq!(class_parts(b"inst\0"), (&b"inst"[..], None));
        assert!(xembed_words(&[0; 7]).is_none());
        let words = [7u32.to_ne_bytes(), 1u32.to_ne_bytes()].concat();
        assert_eq!(xembed_words(&words), Some([7, 1]));
    }
    #[test]
    fn struts_and_event_packets_keep_native_word_order_and_zero_unused_bytes() {
        let bounds = rect_t {
            x: -1920,
            y: -10,
            w: 1920,
            h: 100,
        };
        let top = strut_values(crate::application::BarPosition::Top, 23, bounds);
        assert_eq!((top[2], top[8], top[9]), (23, (-1920i32) as u32, 0));
        let bottom = strut_values(crate::application::BarPosition::Bottom, 23, bounds);
        assert_eq!(
            (bottom[3], bottom[10], bottom[11]),
            (23, (-1920i32) as u32, 0)
        );
        assert_eq!(
            strut_values(crate::application::BarPosition::None, 23, bounds),
            [0; 12]
        );
        let message = client_message(42, 99, [0, u32::MAX, 2, 3, 4]);
        assert_eq!(
            (
                message.window,
                message.type_0,
                message.format,
                message.sequence
            ),
            (42, 99, 32, 0)
        );
        assert_eq!(unsafe { message.data.data32 }, [0, u32::MAX, 2, 3, 4]);
        let destroyed = destroy_message(42);
        assert_eq!((destroyed.event.event, destroyed.event.window), (42, 42));
        assert_eq!(destroyed.padding, [0; 20]);
    }
    #[test]
    fn property_views_reject_overclaims_before_slice_creation() {
        // Native malloc ownership matches the real reply adapter and destructor.
        unsafe {
            let pointer = libc::calloc(1, 40).cast::<xcb_get_property_reply_t>();
            assert!(!pointer.is_null());
            (*pointer).format = 32;
            (*pointer).length = 2;
            (*pointer).value_len = 2;
            let owner = Native::take(pointer).unwrap();
            let data = pointer.add(1).cast::<u32>();
            data.write(7);
            data.add(1).write(1);
            assert_eq!(
                property_bytes(owner.raw()).and_then(xembed_words),
                Some([7, 1])
            );
            (*pointer).value_len = 3;
            assert!(property_bytes(owner.raw()).is_none());
        }
    }
}
