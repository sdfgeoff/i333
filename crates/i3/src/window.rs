/*
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 * Updates window attributes (X11 hints/properties).
 */
use crate::application::conn;
use crate::assignments::run_assignments;
use crate::con::{con_by_window_id, con_parse_title_format};
use crate::ewmh::ewmh_update_visible_name;
use crate::log::{debuglog, verboselog};
use crate::render::render_deco_height;
use crate::types::{reservedpx, BorderStyle, Window};
use cairo_sys::{
    cairo_image_surface_create_for_data, cairo_surface_destroy, cairo_surface_set_user_data,
    cairo_user_data_key_t, FORMAT_A_RGB32,
};
use i3_common::dpi::logical_px;
use i3_common::safewrappers::smalloc;
use i3_common::string::I3String;
use libc::free;
use std::ffi::CStr;
use std::ptr::NonNull;
use xcb_sys::{
    xcb_generic_error_t, xcb_get_geometry_reply_t, xcb_get_property_reply_t,
    xcb_icccm_get_wm_normal_hints_reply, xcb_icccm_get_wm_normal_hints_unchecked,
    xcb_icccm_get_wm_transient_for_from_reply, xcb_icccm_wm_hints_t, xcb_size_hints_t,
    xcb_window_t, XCB_ATOM_CARDINAL, XCB_ICCCM_SIZE_HINT_BASE_SIZE, XCB_ICCCM_SIZE_HINT_P_ASPECT,
    XCB_ICCCM_SIZE_HINT_P_MAX_SIZE, XCB_ICCCM_SIZE_HINT_P_MIN_SIZE, XCB_ICCCM_SIZE_HINT_P_POSITION,
    XCB_ICCCM_SIZE_HINT_P_RESIZE_INC, XCB_ICCCM_SIZE_HINT_P_SIZE, XCB_ICCCM_SIZE_HINT_US_POSITION,
    XCB_ICCCM_SIZE_HINT_US_SIZE, XCB_ICCCM_WM_HINT_INPUT,
};

macro_rules! trace {
    ($function:literal, $line:literal, $message:literal $(,)?) => {
        debuglog(
            c"%s:%s:%d - %s".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            concat!($function, "\0").as_ptr().cast::<libc::c_char>(),
            $line as libc::c_int,
            $message.as_ptr(),
        );
    };
}

macro_rules! tracef {
    ($function:literal, $line:literal, $format:literal $(, $arg:expr)* $(,)?) => {
        debuglog(concat!("%s:%s:%d - ", $format, "\0").as_ptr().cast_mut().cast(),
            c"../src/window.c".as_ptr(), concat!($function,"\0").as_ptr().cast::<libc::c_char>(),
            $line as libc::c_int $(, $arg)*);
    };
}

/// One uniquely owned malloc allocation returned by XCB. Construction requires
/// a complete native reply allocation, including the payload claimed by length.
struct PropertyReply(Option<NonNull<xcb_get_property_reply_t>>);
impl PropertyReply {
    unsafe fn take(reply: *mut xcb_get_property_reply_t) -> Self {
        Self(NonNull::new(reply))
    }
    fn raw(&self) -> *mut xcb_get_property_reply_t {
        self.0.map_or(std::ptr::null_mut(), NonNull::as_ptr)
    }
    fn view(&self) -> Property<'_> {
        unsafe { property_view(self.raw()) }
    }
}
impl Drop for PropertyReply {
    fn drop(&mut self) {
        unsafe {
            free(self.raw().cast());
        }
    }
}

#[derive(Clone, Copy)]
struct Property<'a> {
    kind: u32,
    format: u8,
    bytes: &'a [u8],
}
impl Property<'_> {
    fn word(&self, index: usize) -> Option<u32> {
        let start = index.checked_mul(4)?;
        Some(u32::from_ne_bytes(
            self.bytes
                .get(start..start.checked_add(4)?)?
                .try_into()
                .ok()?,
        ))
    }
}
/// The allocation must be a complete live XCB reply, and remain immutable for
/// the returned borrow. Validate payload arithmetic before constructing a slice;
/// never trust value_len to extend beyond the native reply's wire allocation.
unsafe fn property_view<'a>(reply: *const xcb_get_property_reply_t) -> Property<'a> {
    let Some(reply) = (unsafe { reply.as_ref() }) else {
        return Property {
            kind: 0,
            format: 0,
            bytes: &[],
        };
    };
    let length = match reply.format {
        8 | 16 | 32 => (reply.value_len as usize).checked_mul(reply.format as usize / 8),
        _ => None,
    };
    let wire_len = (reply.length as usize).checked_mul(4);
    let bytes = match (length, wire_len) {
        (Some(len), Some(wire)) if len <= wire && len <= isize::MAX as usize => unsafe {
            std::slice::from_raw_parts(
                (reply as *const xcb_get_property_reply_t).add(1).cast(),
                len,
            )
        },
        _ => {
            return Property {
                kind: 0,
                format: 0,
                bytes: &[],
            }
        }
    };
    Property {
        kind: reply.type_0,
        format: reply.format,
        bytes,
    }
}
fn string_prefix(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(bytes.len())]
}
fn class_parts(bytes: &[u8]) -> (&[u8], Option<&[u8]>) {
    let instance = string_prefix(bytes);
    let class = bytes
        .get(instance.len() + 1..)
        .filter(|rest| !rest.is_empty())
        .map(string_prefix);
    (instance, class)
}
/// Transfer a byte string to the existing malloc-owned Window field ABI.
unsafe fn replace_string(slot: *mut *mut libc::c_char, bytes: Option<&[u8]>) {
    let owned = bytes
        .map(|bytes| {
            let bytes = string_prefix(bytes);
            let ptr = unsafe { smalloc(bytes.len() + 1) }.cast::<libc::c_char>();
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr.cast(), bytes.len());
                ptr.add(bytes.len()).write(0);
            }
            ptr
        })
        .unwrap_or(std::ptr::null_mut());
    unsafe {
        free(slot.replace(owned).cast());
    }
}

/// All entry points operate on the live event-thread Window graph. Property
/// update APIs consume their reply except normal_hints, which explicitly borrows
/// it. No &mut Window is held across tree, logging, or assignment callbacks.
pub unsafe fn window_free(win: *mut Window) {
    for text in [
        (*win).class_class,
        (*win).class_instance,
        (*win).role,
        (*win).machine,
    ] {
        free(text.cast());
    }
    cairo_surface_destroy((*win).icon);
    free((*win).ran_assignments.cast());
    drop(Box::from_raw(win));
}

pub unsafe fn window_update_class(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    let value = prop.view();
    if value.bytes.is_empty() {
        trace!("window_update_class", 36, c"WM_CLASS not set.\n");
        return;
    }
    let (instance, class) = class_parts(value.bytes);
    replace_string(&raw mut (*win).class_instance, Some(instance));
    replace_string(&raw mut (*win).class_class, class);
    verboselog(
        c"WM_CLASS changed to %s (instance), %s (class)\n"
            .as_ptr()
            .cast_mut(),
        (*win).class_instance,
        (*win).class_class,
    );
}

unsafe fn set_name(win: *mut Window, bytes: &[u8]) {
    (*win).name = Some(I3String::from_bytes(string_prefix(bytes)));
    let con = con_by_window_id((*win).id);
    if !con.is_null() && !(*con).title_format.is_null() {
        let formatted = con_parse_title_format(con);
        ewmh_update_visible_name(
            (*win).id,
            formatted
                .cstr_or_report()
                .map_or(std::ptr::null(), CStr::as_ptr),
        );
    }
}
pub unsafe fn window_update_name(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    if prop.view().bytes.is_empty() {
        trace!(
            "window_update_name",
            70,
            c"_NET_WM_NAME not specified, not changing\n"
        );
        return;
    }
    set_name(win, prop.view().bytes);
    (*win).name_x_changed = true;
    verboselog(
        c"_NET_WM_NAME changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win)
            .name
            .as_ref()
            .and_then(I3String::cstr_or_report)
            .map_or(std::ptr::null(), CStr::as_ptr),
    );
    (*win).uses_net_wm_name = true;
}
pub unsafe fn window_update_name_legacy(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    if prop.view().bytes.is_empty() {
        trace!(
            "window_update_name_legacy",
            106,
            c"WM_NAME not set (_NET_WM_NAME is what you want anyways).\n"
        );
        return;
    }
    if (*win).uses_net_wm_name {
        return;
    }
    set_name(win, prop.view().bytes);
    verboselog(
        c"WM_NAME changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win)
            .name
            .as_ref()
            .and_then(I3String::cstr_or_report)
            .map_or(std::ptr::null(), CStr::as_ptr),
    );
    verboselog(c"Using legacy window title. Note that in order to get Unicode window titles in i3, the application has to set _NET_WM_NAME (UTF-8)\n".as_ptr().cast_mut());
    (*win).name_x_changed = true;
}
pub unsafe fn window_update_leader(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    let Some(leader) = prop.view().word(0) else {
        tracef!(
            "window_update_leader",
            145,
            "CLIENT_LEADER not set on window 0x%08x.\n",
            (*win).id
        );
        (*win).leader = 0;
        return;
    };
    tracef!(
        "window_update_leader",
        157,
        "Client leader changed to %08x\n",
        leader
    );
    (*win).leader = leader;
}
pub unsafe fn window_update_transient_for(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    if prop.view().bytes.is_empty() {
        tracef!(
            "window_update_transient_for",
            170,
            "TRANSIENT_FOR not set on window 0x%08x.\n",
            (*win).id
        );
        (*win).transient_for = 0;
        return;
    }
    if prop.view().word(0).is_none() {
        return;
    }
    let mut transient_for = 0;
    if xcb_icccm_get_wm_transient_for_from_reply(&raw mut transient_for, prop.raw()) != 0 {
        tracef!(
            "window_update_transient_for",
            182,
            "Transient for changed to 0x%08x (window 0x%08x)\n",
            transient_for,
            (*win).id
        );
        (*win).transient_for = transient_for;
    }
}
pub unsafe fn window_update_strut_partial(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    let value = prop.view();
    if value.bytes.is_empty() {
        trace!(
            "window_update_strut_partial",
            195,
            c"_NET_WM_STRUT_PARTIAL not set.\n"
        );
        return;
    }
    if let (Some(left), Some(right), Some(top), Some(bottom)) =
        (value.word(0), value.word(1), value.word(2), value.word(3))
    {
        tracef!(
            "window_update_strut_partial",
            207,
            "Reserved pixels changed to: left = %d, right = %d, top = %d, bottom = %d\n",
            left,
            right,
            top,
            bottom
        );
        (*win).reserved = reservedpx {
            left,
            right,
            top,
            bottom,
        };
    }
}
pub unsafe fn window_update_role(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    if prop.view().bytes.is_empty() {
        trace!("window_update_role", 220, c"WM_WINDOW_ROLE not set.\n");
        return;
    }
    replace_string(&raw mut (*win).role, Some(prop.view().bytes));
    verboselog(
        c"WM_WINDOW_ROLE changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win).role,
    );
}
pub unsafe fn window_update_machine(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    if prop.view().bytes.is_empty() {
        trace!(
            "window_update_machine",
            509,
            c"WM_CLIENT_MACHINE not set.\n"
        );
        return;
    }
    replace_string(&raw mut (*win).machine, Some(prop.view().bytes));
    verboselog(
        c"WM_CLIENT_MACHINE changed to \"%s\"\n".as_ptr().cast_mut(),
        (*win).machine,
    );
}
pub unsafe fn window_update_type(window: *mut Window, reply: *mut xcb_get_property_reply_t) {
    let reply = PropertyReply::take(reply);
    let value = reply.view();
    // The old helper assumes format=32 and can walk beyond short/malformed
    // payloads; choose only from bounded native-endian words here instead.
    let supported = [
        crate::application::A__NET_WM_WINDOW_TYPE_NORMAL,
        crate::application::A__NET_WM_WINDOW_TYPE_DIALOG,
        crate::application::A__NET_WM_WINDOW_TYPE_UTILITY,
        crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR,
        crate::application::A__NET_WM_WINDOW_TYPE_SPLASH,
        crate::application::A__NET_WM_WINDOW_TYPE_MENU,
        crate::application::A__NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
        crate::application::A__NET_WM_WINDOW_TYPE_POPUP_MENU,
        crate::application::A__NET_WM_WINDOW_TYPE_TOOLTIP,
        crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION,
    ];
    let new_type = preferred_type(value, &supported);
    drop(reply);
    let Some(new_type) = new_type else {
        trace!(
            "window_update_type",
            243,
            c"cannot read _NET_WM_WINDOW_TYPE from window.\n"
        );
        return;
    };
    (*window).window_type = new_type;
    verboselog(
        c"_NET_WM_WINDOW_TYPE changed to %i.\n".as_ptr().cast_mut(),
        new_type,
    );
    run_assignments(window);
}
fn preferred_type(property: Property<'_>, supported: &[u32]) -> Option<u32> {
    if property.format != 32 {
        return None;
    }
    property
        .bytes
        .chunks_exact(4)
        .map(|word| u32::from_ne_bytes(word.try_into().unwrap()))
        .find(|atom| *atom != 0 && supported.contains(atom))
}
fn decode_size_hints(property: Property<'_>) -> Option<xcb_size_hints_t> {
    if property.kind != xcb_sys::XCB_ATOM_WM_SIZE_HINTS || property.format != 32 {
        return None;
    }
    let signed = |index| property.word(index).unwrap_or(0) as i32;
    let modern = property.bytes.len() / 4 >= 18;
    Some(xcb_size_hints_t {
        flags: property.word(0).unwrap_or(0) & if modern { 0x3ff } else { 0xff },
        x: signed(1),
        y: signed(2),
        width: signed(3),
        height: signed(4),
        min_width: signed(5),
        min_height: signed(6),
        max_width: signed(7),
        max_height: signed(8),
        width_inc: signed(9),
        height_inc: signed(10),
        min_aspect_num: signed(11),
        min_aspect_den: signed(12),
        max_aspect_num: signed(13),
        max_aspect_den: signed(14),
        base_width: if modern { signed(15) } else { 0 },
        base_height: if modern { signed(16) } else { 0 },
        win_gravity: if modern {
            property.word(17).unwrap()
        } else {
            0
        },
    })
}
fn decode_wm_hints(property: Property<'_>) -> Option<xcb_icccm_wm_hints_t> {
    if property.kind != xcb_sys::XCB_ATOM_WM_HINTS
        || property.format != 32
        || property.bytes.len() < 8 * 4
    {
        return None;
    }
    Some(xcb_icccm_wm_hints_t {
        flags: property.word(0)? as i32,
        input: property.word(1)?,
        initial_state: property.word(2)? as i32,
        icon_pixmap: property.word(3)?,
        icon_window: property.word(4)?,
        icon_x: property.word(5)? as i32,
        icon_y: property.word(6)? as i32,
        icon_mask: property.word(7)?,
        window_group: property.word(8).unwrap_or(0),
    })
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct SizeConstraints {
    min_width: i32,
    min_height: i32,
    max_width: i32,
    max_height: i32,
    width_increment: i32,
    height_increment: i32,
    base_width: i32,
    base_height: i32,
    min_aspect_ratio: f64,
    max_aspect_ratio: f64,
}
impl SizeConstraints {
    fn update(&mut self, hints: &xcb_size_hints_t) -> bool {
        fn set<T: PartialEq + Copy>(slot: &mut T, value: T) -> bool {
            if *slot == value {
                false
            } else {
                *slot = value;
                true
            }
        }
        let mut changed = false;
        if hints.flags & XCB_ICCCM_SIZE_HINT_P_MIN_SIZE != 0 {
            changed |= set(&mut self.min_width, hints.min_width);
            changed |= set(&mut self.min_height, hints.min_height);
        }
        let max = hints.flags & XCB_ICCCM_SIZE_HINT_P_MAX_SIZE != 0;
        changed |= set(
            &mut self.max_width,
            if max { hints.max_width.max(0) } else { 0 },
        );
        changed |= set(
            &mut self.max_height,
            if max { hints.max_height.max(0) } else { 0 },
        );
        let increments = hints.flags & XCB_ICCCM_SIZE_HINT_P_RESIZE_INC != 0;
        let increment = |n| {
            if increments && (1..0xffff).contains(&n) {
                n
            } else {
                0
            }
        };
        changed |= set(&mut self.width_increment, increment(hints.width_inc));
        changed |= set(&mut self.height_increment, increment(hints.height_inc));
        // Preserve the original check of the existing base size, rather than
        // silently changing it to validate the incoming base size.
        let base = hints.flags & XCB_ICCCM_SIZE_HINT_BASE_SIZE != 0
            && self.base_width >= 0
            && self.base_height >= 0;
        changed |= set(
            &mut self.base_width,
            if base { hints.base_width } else { 0 },
        );
        changed |= set(
            &mut self.base_height,
            if base { hints.base_height } else { 0 },
        );
        if hints.flags & XCB_ICCCM_SIZE_HINT_P_ASPECT != 0
            && hints.min_aspect_num >= 0
            && hints.min_aspect_den > 0
            && hints.max_aspect_num >= 0
            && hints.max_aspect_den > 0
        {
            let min = hints.min_aspect_num as f64 / hints.min_aspect_den as f64;
            let max = hints.max_aspect_num as f64 / hints.max_aspect_den as f64;
            if (self.min_aspect_ratio - min).abs() > f64::EPSILON {
                self.min_aspect_ratio = min;
                changed = true;
            }
            if (self.max_aspect_ratio - max).abs() > f64::EPSILON {
                self.max_aspect_ratio = max;
                changed = true;
            }
        } else {
            changed |= set(&mut self.min_aspect_ratio, 0.0);
            changed |= set(&mut self.max_aspect_ratio, 0.0);
        }
        changed
    }
}
fn requested_geometry(hints: &xcb_size_hints_t) -> Option<(i16, i16, u16, u16)> {
    let positioned =
        hints.flags & (XCB_ICCCM_SIZE_HINT_US_POSITION | XCB_ICCCM_SIZE_HINT_P_POSITION) != 0;
    let sized = hints.flags & (XCB_ICCCM_SIZE_HINT_US_SIZE | XCB_ICCCM_SIZE_HINT_P_SIZE) != 0;
    (positioned && sized).then_some((
        hints.x as i16,
        hints.y as i16,
        hints.width as u16,
        hints.height as u16,
    ))
}
// Only owned hint values are borrowed while native logging runs.
unsafe fn log_size_hints(size_hints: &xcb_size_hints_t, old: SizeConstraints, has_geometry: bool) {
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MIN_SIZE != 0 {
        debuglog(
            c"%s:%s:%d - Minimum size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            282 as ::core::ffi::c_int,
            size_hints.min_width,
            size_hints.min_height,
        );
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_MAX_SIZE != 0 {
        debuglog(
            c"%s:%s:%d - Maximum size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            289 as ::core::ffi::c_int,
            size_hints.max_width,
            size_hints.max_height,
        );
    } else {
        debuglog(
            c"%s:%s:%d - Clearing maximum size\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            297 as ::core::ffi::c_int,
        );
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_RESIZE_INC != 0 {
        debuglog(
            c"%s:%s:%d - Size increments: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            304 as ::core::ffi::c_int,
            size_hints.width_inc,
            size_hints.height_inc,
        );
    } else {
        debuglog(
            c"%s:%s:%d - Clearing size increments\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            318 as ::core::ffi::c_int,
        );
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_BASE_SIZE != 0
        && old.base_width >= 0
        && old.base_height >= 0
    {
        debuglog(
            c"%s:%s:%d - Base size: %d (width) x %d (height)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            327 as ::core::ffi::c_int,
            size_hints.base_width,
            size_hints.base_height,
        );
    } else {
        debuglog(
            c"%s:%s:%d - Clearing base size\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            332 as ::core::ffi::c_int,
        );
    }
    if has_geometry && requested_geometry(size_hints).is_some() {
        debuglog(
            c"%s:%s:%d - Setting geometry x=%d y=%d w=%d h=%d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            341 as ::core::ffi::c_int,
            size_hints.x,
            size_hints.y,
            size_hints.width,
            size_hints.height,
        );
    }
    if size_hints.flags & XCB_ICCCM_SIZE_HINT_P_ASPECT != 0
        && size_hints.min_aspect_num >= 0
        && size_hints.min_aspect_den > 0
        && size_hints.max_aspect_num >= 0
        && size_hints.max_aspect_den > 0
    {
        let min_aspect = size_hints.min_aspect_num as f64 / size_hints.min_aspect_den as f64;
        let max_aspect = size_hints.max_aspect_num as f64 / size_hints.max_aspect_den as f64;
        debuglog(
            c"%s:%s:%d - Aspect ratio set: minimum %f, maximum %f\n"
                .as_ptr()
                .cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            355 as ::core::ffi::c_int,
            min_aspect,
            max_aspect,
        );
    } else {
        debuglog(
            c"%s:%s:%d - Clearing aspect ratios\n".as_ptr().cast_mut(),
            c"../src/window.c".as_ptr(),
            c"window_update_normal_hints".as_ptr(),
            365 as ::core::ffi::c_int,
        );
    }
}
/// Unlike the other update APIs, reply is BORROWED: existing handlers free it.
/// win/geom must be live event-thread graph objects; geom may be null. A null
/// reply requests an independent native ICCCM fetch, which owns its own reply.
pub unsafe fn window_update_normal_hints(
    win: *mut Window,
    reply: *mut xcb_get_property_reply_t,
    geom: *mut xcb_get_geometry_reply_t,
) -> bool {
    let mut hints: xcb_size_hints_t = std::mem::zeroed();
    let success = if reply.is_null() {
        xcb_icccm_get_wm_normal_hints_reply(
            conn,
            xcb_icccm_get_wm_normal_hints_unchecked(conn, (*win).id),
            &raw mut hints,
            std::ptr::null_mut(),
        ) != 0
    } else {
        if let Some(decoded) = decode_size_hints(property_view(reply)) {
            hints = decoded;
            true
        } else {
            false
        }
    };
    if !success {
        trace!(
            "window_update_normal_hints",
            269,
            c"Could not get WM_NORMAL_HINTS\n"
        );
        return false;
    }
    let mut state = SizeConstraints {
        min_width: (*win).min_width,
        min_height: (*win).min_height,
        max_width: (*win).max_width,
        max_height: (*win).max_height,
        width_increment: (*win).width_increment,
        height_increment: (*win).height_increment,
        base_width: (*win).base_width,
        base_height: (*win).base_height,
        min_aspect_ratio: (*win).min_aspect_ratio,
        max_aspect_ratio: (*win).max_aspect_ratio,
    };
    log_size_hints(&hints, state, !geom.is_null());
    let changed = state.update(&hints);
    (*win).min_width = state.min_width;
    (*win).min_height = state.min_height;
    (*win).max_width = state.max_width;
    (*win).max_height = state.max_height;
    (*win).width_increment = state.width_increment;
    (*win).height_increment = state.height_increment;
    (*win).base_width = state.base_width;
    (*win).base_height = state.base_height;
    if !geom.is_null() {
        if let Some((x, y, width, height)) = requested_geometry(&hints) {
            (*geom).x = x;
            (*geom).y = y;
            (*geom).width = width;
            (*geom).height = height;
        }
    }
    (*win).min_aspect_ratio = state.min_aspect_ratio;
    (*win).max_aspect_ratio = state.max_aspect_ratio;
    changed
}

fn hint_effects(hints: &xcb_icccm_wm_hints_t) -> (Option<bool>, bool) {
    (
        (hints.flags & XCB_ICCCM_WM_HINT_INPUT as i32 != 0).then_some(hints.input == 0),
        hints.flags & xcb_sys::XCB_ICCCM_WM_HINT_X_URGENCY as i32 != 0,
    )
}
pub unsafe fn window_update_hints(
    win: *mut Window,
    prop: *mut xcb_get_property_reply_t,
    urgency_hint: *mut bool,
) {
    let prop = PropertyReply::take(prop);
    if !urgency_hint.is_null() {
        *urgency_hint = false;
    }
    if prop.view().bytes.is_empty() {
        trace!("window_update_hints", 384, c"WM_HINTS not set.\n");
        return;
    }
    let Some(hints) = decode_wm_hints(prop.view()) else {
        trace!("window_update_hints", 392, c"Could not get WM_HINTS\n");
        return;
    };
    let (focus, urgent) = hint_effects(&hints);
    if let Some(doesnt_accept_focus) = focus {
        (*win).doesnt_accept_focus = doesnt_accept_focus;
        verboselog(
            c"WM_HINTS.input changed to \"%d\"\n".as_ptr().cast_mut(),
            hints.input,
        );
    }
    if !urgency_hint.is_null() {
        *urgency_hint = urgent;
    }
}
fn border_style_from_motif_value(value: u32) -> BorderStyle {
    let all = value & 1 != 0;
    let border = value & 2 != 0;
    let title = value & 8 != 0;
    match (all, title, border) {
        (true, true, true) | (false, false, false) => BorderStyle::None,
        (true, true, false) | (false, false, true) => BorderStyle::Pixel,
        _ => BorderStyle::Normal,
    }
}
fn motif_border(property: Property<'_>) -> Option<BorderStyle> {
    if property.word(0)? & 2 == 0 {
        return None;
    }
    Some(border_style_from_motif_value(property.word(2)?))
}
pub unsafe fn window_update_motif_hints(
    _win: *mut Window,
    prop: *mut xcb_get_property_reply_t,
    motif_border_style: *mut BorderStyle,
) -> bool {
    let prop = PropertyReply::take(prop);
    if prop.raw().is_null() {
        return false;
    }
    assert!(!motif_border_style.is_null(), "motif_border_style != NULL");
    let Some(border) = motif_border(prop.view()) else {
        return false;
    };
    *motif_border_style = border;
    true
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct IconCandidate {
    width: u32,
    height: u32,
    start: usize,
    pixels: usize,
}
fn select_icon(property: Property<'_>, preferred: u32) -> Option<IconCandidate> {
    select_icon_with(property, preferred, |_, _| {})
}
fn select_icon_with(
    property: Property<'_>,
    preferred: u32,
    mut found: impl FnMut(u32, u32),
) -> Option<IconCandidate> {
    if property.kind != XCB_ATOM_CARDINAL || property.format != 32 {
        return None;
    }
    let mut offset = 0usize;
    let mut selected: Option<IconCandidate> = None;
    // Native selection requires strictly more than the two dimension words.
    while property.bytes.len().checked_sub(offset)? > 8 {
        let remaining = Property {
            bytes: &property.bytes[offset..],
            ..property
        };
        let (width, height) = (remaining.word(0)?, remaining.word(1)?);
        if width == 0 || height == 0 {
            break;
        }
        let Some(pixels) = (width as usize).checked_mul(height as usize) else {
            break;
        };
        let Some(length) = pixels.checked_add(2).and_then(|n| n.checked_mul(4)) else {
            break;
        };
        if length > remaining.bytes.len() {
            break;
        }
        found(width, height);
        let candidate = IconCandidate {
            width,
            height,
            start: offset + 8,
            pixels,
        };
        let replace = selected.is_none_or(|old| {
            let at_least_preferred = width >= preferred && height >= preferred;
            let smaller = width < old.width || height < old.height;
            let larger = width > old.width || height > old.height;
            let not_preferred = old.width < preferred || old.height < preferred;
            at_least_preferred && (smaller || not_preferred)
                || !at_least_preferred && not_preferred && larger
        });
        if replace {
            selected = Some(candidate);
        }
        if selected.is_some_and(|icon| icon.width == preferred && icon.height == preferred) {
            break;
        }
        offset += length;
    }
    selected
}
fn premultiplied_argb(pixel: u32) -> u32 {
    let alpha = pixel >> 24;
    let channel = |shift: u32| ((pixel >> shift) & 0xff_u32) * alpha / 255;
    alpha << 24 | channel(16) << 16 | channel(8) << 8 | channel(0)
}
fn icon_pixels(property: Property<'_>, icon: IconCandidate) -> Vec<u32> {
    property.bytes[icon.start..icon.start + icon.pixels * 4]
        .chunks_exact(4)
        .map(|bytes| premultiplied_argb(u32::from_ne_bytes(bytes.try_into().unwrap())))
        .collect()
}

/// Own native pixel allocation until Cairo accepts its destruction callback.
struct NativePixels(NonNull<u32>);
impl NativePixels {
    fn copy(pixels: &[u32]) -> Self {
        let bytes = std::mem::size_of_val(pixels);
        let ptr = unsafe { smalloc(bytes) }.cast::<u32>();
        unsafe {
            std::ptr::copy_nonoverlapping(pixels.as_ptr(), ptr, pixels.len());
        }
        Self(NonNull::new(ptr).expect("smalloc returns nonnull"))
    }
}
impl Drop for NativePixels {
    fn drop(&mut self) {
        unsafe {
            free(self.0.as_ptr().cast());
        }
    }
}
/// Own a native Cairo reference while its pixels are attached to that
/// reference's user-data destructor. Transfer only at the Window ABI boundary.
struct OwnedIcon(NonNull<cairo_sys::cairo_surface_t>);
impl OwnedIcon {
    fn new(width: u32, height: u32, values: &[u32]) -> Option<Self> {
        let expected = (width as usize).checked_mul(height as usize)?;
        if width == 0 || height == 0 || values.len() != expected {
            return None;
        }
        let stride = i32::try_from(width.checked_mul(4)?).ok()?;
        let (width, height) = (i32::try_from(width).ok()?, i32::try_from(height).ok()?);
        let pixels = NativePixels::copy(values);
        // cairo-sys is maintained by cairo-rs. The backing allocation is owned
        // independently of the temporary Rust values passed to this constructor.
        let surface = Self(NonNull::new(unsafe {
            cairo_image_surface_create_for_data(
                pixels.0.as_ptr().cast(),
                FORMAT_A_RGB32,
                width,
                height,
                stride,
            )
        })?);
        static mut FREE_DATA: cairo_user_data_key_t = cairo_user_data_key_t { unused: 0 };
        let status = unsafe {
            cairo_surface_set_user_data(
                surface.0.as_ptr(),
                &raw mut FREE_DATA,
                pixels.0.as_ptr().cast(),
                Some(free),
            )
        };
        if status != 0 {
            return None;
        }
        std::mem::forget(pixels);
        Some(surface)
    }
    fn into_raw(self) -> *mut cairo_sys::cairo_surface_t {
        let raw = self.0.as_ptr();
        std::mem::forget(self);
        raw
    }
}
impl Drop for OwnedIcon {
    fn drop(&mut self) {
        unsafe {
            cairo_surface_destroy(self.0.as_ptr());
        }
    }
}

pub unsafe fn window_update_icon(win: *mut Window, prop: *mut xcb_get_property_reply_t) {
    let prop = PropertyReply::take(prop);
    let property = prop.view();
    let preferred = (render_deco_height() - logical_px(2)) as u32;
    if property.kind != XCB_ATOM_CARDINAL || property.format != 32 {
        trace!("window_update_icon", 528, c"_NET_WM_ICON is not set\n");
        return;
    }
    let Some(icon) = select_icon_with(property, preferred, |width, height| {
        tracef!(
            "window_update_icon",
            555,
            "Found _NET_WM_ICON of size: (%d,%d)\n",
            width,
            height
        );
    }) else {
        trace!("window_update_icon", 587, c"Could not get _NET_WM_ICON\n");
        return;
    };
    tracef!(
        "window_update_icon",
        593,
        "Using icon of size (%d,%d) (preferred size: %d)\n",
        icon.width,
        icon.height,
        preferred
    );
    let Some(surface) = OwnedIcon::new(icon.width, icon.height, &icon_pixels(property, icon))
    else {
        return;
    };
    (*win).name_x_changed = true;
    cairo_surface_destroy((*win).icon);
    (*win).icon = surface.into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn property(bytes: &[u8]) -> Property<'_> {
        Property {
            kind: XCB_ATOM_CARDINAL,
            format: 32,
            bytes,
        }
    }
    fn words(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|word| word.to_ne_bytes()).collect()
    }
    fn reply(kind: u32, format: u8, bytes: &[u8]) -> PropertyReply {
        let payload = bytes.len().div_ceil(4) * 4;
        let allocation =
            unsafe { libc::calloc(1, size_of::<xcb_get_property_reply_t>() + payload) }
                .cast::<xcb_get_property_reply_t>();
        assert!(!allocation.is_null());
        unsafe {
            (*allocation).type_0 = kind;
            (*allocation).format = format;
            (*allocation).length = (payload / 4) as u32;
            (*allocation).value_len = (bytes.len() / (usize::from(format) / 8).max(1)) as u32;
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), allocation.add(1).cast(), bytes.len());
            PropertyReply::take(allocation)
        }
    }

    #[test]
    fn class_and_string_parsing_preserve_raw_bytes_and_first_nul() {
        assert_eq!(
            class_parts(b"instance\0Class\0ignored"),
            (&b"instance"[..], Some(&b"Class"[..]))
        );
        assert_eq!(class_parts(b"instance"), (&b"instance"[..], None));
        assert_eq!(class_parts(b"instance\0"), (&b"instance"[..], None));
        assert_eq!(class_parts(b"\0\0"), (&b""[..], Some(&b""[..])));
        assert_eq!(
            class_parts(b"\xff\0\xfe"),
            (&b"\xff"[..], Some(&b"\xfe"[..]))
        );
        assert_eq!(string_prefix(b"title\0ignored"), b"title");
        // Final Window string fields still use native malloc/free ownership.
        let mut field = std::ptr::null_mut();
        unsafe {
            replace_string(&mut field, Some(b"\xff\0ignored"));
            assert_eq!(std::ffi::CStr::from_ptr(field).to_bytes(), b"\xff");
            replace_string(&mut field, Some(b""));
            assert!(!field.is_null());
            assert_eq!(std::ffi::CStr::from_ptr(field).to_bytes(), b"");
            replace_string(&mut field, None);
        }
        assert!(field.is_null());
    }

    #[test]
    fn native_reply_views_reject_payload_overclaims_and_unaligned_words() {
        assert_eq!(size_of::<xcb_get_property_reply_t>(), 32);
        let bytes = words(&[0x01020304, 0xabcdef12]);
        let owned = reply(XCB_ATOM_CARDINAL, 32, &bytes);
        assert_eq!(owned.view().bytes, bytes);
        assert_eq!(owned.view().word(1), Some(0xabcdef12));
        assert_eq!(owned.view().word(2), None);
        assert_eq!(owned.view().word(usize::MAX), None);
        let unaligned = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            property(&unaligned[1..]).word(0),
            Some(u32::from_ne_bytes([1, 2, 3, 4]))
        );
        unsafe {
            (*owned.raw()).value_len = 3;
        }
        assert!(owned.view().bytes.is_empty());
        unsafe {
            (*owned.raw()).value_len = 1;
            (*owned.raw()).format = 7;
        }
        assert!(owned.view().bytes.is_empty());
    }

    #[test]
    fn bounded_icccm_decoding_matches_native_legacy_and_modern_forms() {
        for count in 0..=19 {
            let raw: Vec<u32> = (0..count)
                .map(|i| if i == 0 { u32::MAX } else { i as u32 * 3 })
                .collect();
            let hints = words(&raw);
            let owned = reply(xcb_sys::XCB_ATOM_WM_SIZE_HINTS, 32, &hints);
            let mut native: xcb_size_hints_t = unsafe { std::mem::zeroed() };
            assert_ne!(
                unsafe {
                    xcb_sys::xcb_icccm_get_wm_size_hints_from_reply(&mut native, owned.raw())
                },
                0
            );
            let decoded = decode_size_hints(owned.view()).unwrap();
            assert_eq!(
                (
                    decoded.flags,
                    decoded.x,
                    decoded.y,
                    decoded.width,
                    decoded.height
                ),
                (
                    native.flags,
                    native.x,
                    native.y,
                    native.width,
                    native.height
                )
            );
            assert_eq!(
                (
                    decoded.min_width,
                    decoded.min_height,
                    decoded.max_width,
                    decoded.max_height,
                    decoded.width_inc,
                    decoded.height_inc,
                    decoded.min_aspect_num,
                    decoded.min_aspect_den,
                    decoded.max_aspect_num,
                    decoded.max_aspect_den,
                    decoded.base_width,
                    decoded.base_height,
                ),
                (
                    native.min_width,
                    native.min_height,
                    native.max_width,
                    native.max_height,
                    native.width_inc,
                    native.height_inc,
                    native.min_aspect_num,
                    native.min_aspect_den,
                    native.max_aspect_num,
                    native.max_aspect_den,
                    native.base_width,
                    native.base_height,
                )
            );
            assert_eq!(decoded.win_gravity, native.win_gravity);
            let owned = reply(xcb_sys::XCB_ATOM_WM_HINTS, 32, &hints);
            if count <= 9 {
                // Some native libxcb-icccm versions over-copy oversized WM_HINTS;
                // never use such a reply as a native oracle or runtime decoder.
                let mut native: xcb_icccm_wm_hints_t = unsafe { std::mem::zeroed() };
                let success = unsafe {
                    xcb_sys::xcb_icccm_get_wm_hints_from_reply(&mut native, owned.raw()) != 0
                };
                let decoded = decode_wm_hints(owned.view());
                assert_eq!(decoded.is_some(), success);
                if let Some(decoded) = decoded {
                    assert_eq!(
                        (
                            decoded.flags,
                            decoded.input,
                            decoded.initial_state,
                            decoded.icon_pixmap,
                            decoded.icon_window,
                            decoded.icon_x,
                            decoded.icon_y,
                            decoded.icon_mask,
                            decoded.window_group
                        ),
                        (
                            native.flags,
                            native.input,
                            native.initial_state,
                            native.icon_pixmap,
                            native.icon_window,
                            native.icon_x,
                            native.icon_y,
                            native.icon_mask,
                            native.window_group
                        )
                    );
                }
            } else {
                let decoded = decode_wm_hints(owned.view()).unwrap();
                assert_eq!(decoded.window_group, raw[8]);
            }
        }
    }

    #[test]
    fn motif_and_preferred_type_use_only_complete_payload_words() {
        for count in 0..12 {
            let bytes = vec![0xff; count];
            assert!(motif_border(property(&bytes)).is_none());
        }
        let bytes = words(&[2, 0, 1 | 2 | 8]);
        assert_eq!(motif_border(property(&bytes)), Some(BorderStyle::None));
        let bytes = words(&[2, 0, 2]);
        assert_eq!(motif_border(property(&bytes)), Some(BorderStyle::Pixel));
        let bytes = words(&[99, 7, 8]);
        assert_eq!(preferred_type(property(&bytes), &[8, 7]), Some(7));
        assert_eq!(
            preferred_type(
                Property {
                    format: 8,
                    ..property(&bytes)
                },
                &[7]
            ),
            None
        );
    }

    fn icon_bytes(sizes: &[(u32, u32)]) -> Vec<u8> {
        let mut data = Vec::new();
        for &(width, height) in sizes {
            data.extend(words(&[width, height]));
            data.extend(words(&vec![0xff_123456; width as usize * height as usize]));
        }
        data
    }
    #[test]
    fn icon_selection_preserves_preference_ties_and_valid_prefix_on_bad_tail() {
        let bytes = icon_bytes(&[(2, 2), (8, 8), (4, 4), (3, 3)]);
        let selected = select_icon(property(&bytes), 4).unwrap();
        assert_eq!((selected.width, selected.height), (4, 4));
        assert_eq!(
            icon_pixels(property(&bytes), selected),
            vec![0xff_123456; 16]
        );
        let mut bytes = icon_bytes(&[(2, 2)]);
        bytes.extend(words(&[u32::MAX, u32::MAX, 1]));
        let selected = select_icon(property(&bytes), 8).unwrap();
        assert_eq!((selected.width, selected.height), (2, 2));
        for words in [
            &[0, 1, 0][..],
            &[1, 0, 0],
            &[2, 2, 0],
            &[u32::MAX, u32::MAX, 0],
        ] {
            assert_eq!(select_icon(property(&self::words(words)), 4), None);
        }
        let bytes = icon_bytes(&[(4, 2), (2, 4)]);
        // Native comparison uses either dimension, rather than area or sorting.
        let selected = select_icon(property(&bytes), 8).unwrap();
        assert_eq!((selected.width, selected.height), (2, 4));
    }

    #[test]
    fn alpha_premultiplication_matches_native_integer_channel_arithmetic() {
        for alpha in 0..=255u32 {
            for value in 0..=255u32 {
                let pixel = alpha << 24 | value << 16 | (255 - value) << 8 | 127;
                let native = alpha << 24
                    | (value * alpha / 255) << 16
                    | ((255 - value) * alpha / 255) << 8
                    | (127 * alpha / 255);
                assert_eq!(premultiplied_argb(pixel), native);
            }
        }
        assert_eq!(premultiplied_argb(0x80_ff8040), 0x80_804020);
    }

    #[test]
    fn cairo_icon_owns_pixels_across_temporary_and_surface_reference_lifetimes() {
        let temporary = vec![premultiplied_argb(0x80_ff8040), 0xff_123456];
        let surface = OwnedIcon::new(2, 1, &temporary).unwrap();
        drop(temporary);
        let retained = OwnedIcon(
            NonNull::new(unsafe { cairo_sys::cairo_surface_reference(surface.0.as_ptr()) })
                .unwrap(),
        );
        drop(surface);
        unsafe {
            cairo_sys::cairo_surface_flush(retained.0.as_ptr());
            let data = cairo_sys::cairo_image_surface_get_data(retained.0.as_ptr()).cast::<u32>();
            assert_eq!(data.read(), 0x80_804020);
            assert_eq!(data.add(1).read(), 0xff_123456);
        }
        assert!(OwnedIcon::new(2, 2, &[1]).is_none());
        assert!(OwnedIcon::new(0, 1, &[]).is_none());
        assert!(OwnedIcon::new(u32::MAX, 1, &[]).is_none());
    }

    #[test]
    fn constraints_keep_legacy_base_guard_increment_bounds_and_aspect_epsilon() {
        let mut state = SizeConstraints {
            base_width: -1,
            max_width: 100,
            width_increment: 3,
            ..Default::default()
        };
        let mut hints: xcb_size_hints_t = unsafe { std::mem::zeroed() };
        hints.flags = XCB_ICCCM_SIZE_HINT_BASE_SIZE
            | XCB_ICCCM_SIZE_HINT_P_RESIZE_INC
            | XCB_ICCCM_SIZE_HINT_P_ASPECT;
        hints.base_width = 12;
        hints.base_height = 13;
        hints.width_inc = 0xffff;
        hints.height_inc = 0xfffe;
        hints.min_aspect_num = 1;
        hints.min_aspect_den = 2;
        hints.max_aspect_num = 2;
        hints.max_aspect_den = 1;
        assert!(state.update(&hints));
        assert_eq!((state.base_width, state.base_height), (0, 0));
        assert_eq!((state.width_increment, state.height_increment), (0, 0xfffe));
        assert_eq!((state.min_aspect_ratio, state.max_aspect_ratio), (0.5, 2.0));
        assert_eq!(state.max_width, 0);
        assert!(state.update(&hints)); // existing nonnegative bases now permit hints
        assert_eq!((state.base_width, state.base_height), (12, 13));
        assert!(!state.update(&hints));
        hints.flags = 0;
        assert!(state.update(&hints));
        assert_eq!((state.min_aspect_ratio, state.max_aspect_ratio), (0.0, 0.0));
        hints.flags = XCB_ICCCM_SIZE_HINT_US_POSITION | XCB_ICCCM_SIZE_HINT_P_SIZE;
        hints.x = 65537;
        hints.y = -2;
        hints.width = 65536;
        hints.height = -1;
        assert_eq!(requested_geometry(&hints), Some((1, -2, 0, 65535)));
        hints.flags = XCB_ICCCM_SIZE_HINT_US_POSITION;
        assert_eq!(requested_geometry(&hints), None);
    }
}
