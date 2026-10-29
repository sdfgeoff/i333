/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * This is LEGACY code (we support RandR, which can do much more than
 * Xinerama), but necessary for the poor users of the nVidia binary
 * driver which does not support RandR in 2011 *sigh*.
 *
 */
#![deny(unsafe_op_in_unsafe_fn)]

use crate::application::conn;
use crate::log::{debuglog, errorlog};
use crate::output::output_primary_name;
use crate::randr::{create_root_output, init_ws_for_output, output_init_con, outputs};
use crate::types::{output_name, xoutput, xoutput as Output, Rect};
use i3_common::safewrappers::{scalloc, sstrdup};
use std::ffi::CString;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicI32, Ordering};
use xcb_sys::{
    xcb_connection_t, xcb_get_extension_data, xcb_xinerama_id, xcb_xinerama_is_active,
    xcb_xinerama_is_active_reply, xcb_xinerama_query_screens_reply,
    xcb_xinerama_query_screens_reply_t, xcb_xinerama_query_screens_screen_info,
    xcb_xinerama_query_screens_unchecked, xcb_xinerama_screen_info_t,
};

// Survives repeated initialization calls, including clone-only queries.
static NUM_SCREENS: AtomicI32 = AtomicI32::new(0);

fn screen_rect(screen: &xcb_xinerama_screen_info_t) -> Rect {
    Rect {
        x: screen.x_org as u32,
        y: screen.y_org as u32,
        width: u32::from(screen.width),
        height: u32::from(screen.height),
    }
}
fn same_origin(left: Rect, right: Rect) -> bool {
    left.x == right.x && left.y == right.y
}
fn origin_is_zero(rect: Rect) -> bool {
    rect.x == 0 && rect.y == 0
}
fn shrink_clone(rect: &mut Rect, screen: &xcb_xinerama_screen_info_t) {
    // Existing unsigned extents passed through signed int in the original min.
    rect.width = (rect.width as i32).min(i32::from(screen.width)) as u32;
    rect.height = (rect.height as i32).min(i32::from(screen.height)) as u32;
}

fn checked_screen_count(number: u32, length: u32) -> Option<usize> {
    let count = usize::try_from(number).ok()?;
    let required = count.checked_mul(std::mem::size_of::<xcb_xinerama_screen_info_t>())?;
    let available = usize::try_from(length).ok()?.checked_mul(4)?;
    (required <= available && required <= isize::MAX as usize).then_some(count)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ScreenSource {
    MissingExtension,
    Inactive,
    Xinerama,
}
fn screen_source(present: bool, active: Option<bool>) -> ScreenSource {
    if !present {
        ScreenSource::MissingExtension
    } else if active == Some(true) {
        ScreenSource::Xinerama
    } else {
        ScreenSource::Inactive
    }
}

// Owns complete malloc-backed XCB replies, including their advertised body.
struct NativeReply<T>(NonNull<T>);
impl<T> NativeReply<T> {
    /// # Safety
    /// A non-null pointer transfers unique ownership of a complete initialized
    /// XCB reply allocation (header plus its length-advertised body). No other
    /// owner may free it, and any borrowed views must end before this guard.
    unsafe fn new(pointer: *mut T) -> Option<Self> {
        NonNull::new(pointer).map(Self)
    }
    fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }
    fn header(&self) -> &T {
        // SAFETY: the constructor requires a complete initialized reply.
        unsafe { self.0.as_ref() }
    }
}
impl<T> Drop for NativeReply<T> {
    fn drop(&mut self) {
        // SAFETY: this guard uniquely owns its native malloc allocation.
        unsafe { libc::free(self.0.as_ptr().cast()) }
    }
}
impl NativeReply<xcb_xinerama_query_screens_reply_t> {
    fn screens(&self) -> Option<&[xcb_xinerama_screen_info_t]> {
        let count = checked_screen_count(self.header().number, self.header().length)?;
        if count == 0 {
            return Some(&[]);
        }
        // SAFETY: the constructor owns the full advertised body. XCB's screen
        // pointer follows the aligned 32-byte header; the count is bounded by
        // that body's byte length and the slice cannot outlive this reply.
        Some(unsafe {
            std::slice::from_raw_parts(xcb_xinerama_query_screens_screen_info(self.as_ptr()), count)
        })
    }
}

/// # Safety
/// Requires exclusive event-thread access to the initialized output queue.
/// Returned pointers borrow the graph, which must stay live until their use ends.
unsafe fn get_screen_at(rect: Rect) -> *mut Output {
    unsafe {
        let mut output = outputs.tqh_first;
        while !output.is_null() {
            if same_origin((*output).rect, rect) {
                return output;
            }
            output = (*output).outputs.tqe_next;
        }
        std::ptr::null_mut()
    }
}

/// # Safety
/// The caller owns the initialized native output/render/workspace graph on its
/// event thread. New queue nodes and names use its existing malloc/free contract.
/// Init callbacks must not invalidate the new output or queue while attaching it.
unsafe fn apply_screen(screen: &xcb_xinerama_screen_info_t) {
    unsafe {
        let mut s: *mut Output = get_screen_at(screen_rect(screen));
        if !s.is_null() {
            debuglog(
                c"%s:%s:%d - Re-used old Xinerama screen %p\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"query_screens".as_ptr(),
                50 as ::core::ffi::c_int,
                s,
            );
            shrink_clone(&mut (*s).rect, screen);
        } else {
            s = scalloc(1 as usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
            let output_name: *mut output_name =
                scalloc(1 as usize, ::core::mem::size_of::<output_name>() as usize)
                    as *mut output_name;
            let name = CString::new(format!("xinerama-{}", NUM_SCREENS.load(Ordering::Relaxed)))
                .expect("generated output name contains no NUL");
            (*output_name).name = sstrdup(name.as_ptr());
            (*s).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
            (*output_name).names.sle_next = (*s).names_head.slh_first;
            (*s).names_head.slh_first = output_name;
            debuglog(
                c"%s:%s:%d - Created new Xinerama screen %s (%p)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"query_screens".as_ptr(),
                61 as ::core::ffi::c_int,
                output_primary_name(s),
                s,
            );
            (*s).active = true;
            (*s).rect = screen_rect(screen);
            if origin_is_zero((*s).rect) {
                (*s).outputs.tqe_next = outputs.tqh_first;
                if !(*s).outputs.tqe_next.is_null() {
                    (*outputs.tqh_first).outputs.tqe_prev = &raw mut (*s).outputs.tqe_next;
                } else {
                    outputs.tqh_last = &raw mut (*s).outputs.tqe_next;
                }
                outputs.tqh_first = s as *mut xoutput;
                (*s).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*s).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*s).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = s as *mut xoutput;
                outputs.tqh_last = &raw mut (*s).outputs.tqe_next;
            }
            output_init_con(s);
            init_ws_for_output(s);
            // Keep the existing debug-checked/release-wrapping counter arithmetic.
            NUM_SCREENS
                .try_update(Ordering::Relaxed, Ordering::Relaxed, |count| {
                    Some(count + 1)
                })
                .unwrap();
        }
    }
}

/// # Safety
/// connection is live on the event thread; that thread exclusively owns the
/// initialized output, rendering and workspace graphs throughout this call.
unsafe fn query_screens(connection: *mut xcb_connection_t) {
    unsafe {
        let reply = NativeReply::new(xcb_xinerama_query_screens_reply(
            connection,
            xcb_xinerama_query_screens_unchecked(connection),
            std::ptr::null_mut(),
        ));
        let Some(reply) = reply else {
            errorlog(
                c"ERROR: Couldn't get Xinerama screens\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return;
        };
        let Some(screens) = reply.screens() else {
            errorlog(
                c"ERROR: Invalid Xinerama screen reply\n"
                    .as_ptr()
                    .cast_mut(),
            );
            return;
        };
        for screen in screens {
            apply_screen(screen);
            debuglog(
                c"%s:%s:%d - found Xinerama screen: %d x %d at %d x %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/xinerama.c".as_ptr(),
                c"query_screens".as_ptr(),
                80i32,
                i32::from(screen.width),
                i32::from(screen.height),
                i32::from(screen.x_org),
                i32::from(screen.y_org),
            );
        }
    }
    if NUM_SCREENS.load(Ordering::Relaxed) == 0 {
        unsafe {
            errorlog(
                c"ERROR: No screens found. Please fix your setup. i3 will exit now.\n"
                    .as_ptr()
                    .cast_mut(),
            );
            libc::exit(libc::EXIT_SUCCESS);
        }
    }
}

/// # Safety
/// connection and output/render/workspace graphs are initialized and exclusively
/// owned by this event thread. create_root_output uses the native allocator.
unsafe fn use_root_output(connection: *mut xcb_connection_t) {
    unsafe {
        let output = create_root_output(connection);
        (*output).active = true;
        (*output).outputs.tqe_next = std::ptr::null_mut();
        (*output).outputs.tqe_prev = outputs.tqh_last;
        *outputs.tqh_last = output;
        outputs.tqh_last = &raw mut (*output).outputs.tqe_next;
        output_init_con(output);
        init_ws_for_output(output);
    }
}

/// Initialize legacy Xinerama outputs, falling back to the root display.
///
/// # Safety
/// The caller owns the initialized event-thread connection, native output queue,
/// render and workspace graphs exclusively for this call and all init callbacks.
pub unsafe fn xinerama_init() {
    unsafe {
        let extension = xcb_get_extension_data(conn, &raw mut xcb_xinerama_id);
        let present = !extension.is_null() && (*extension).present != 0;
        // Missing extension must not issue an IsActive request.
        let active = if present {
            NativeReply::new(xcb_xinerama_is_active_reply(
                conn,
                xcb_xinerama_is_active(conn),
                std::ptr::null_mut(),
            ))
        } else {
            None
        };
        match screen_source(
            present,
            active.as_ref().map(|reply| reply.header().state != 0),
        ) {
            ScreenSource::MissingExtension => {
                debuglog(
                    c"%s:%s:%d - Xinerama extension not found, using root output.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/xinerama.c".as_ptr(),
                    c"xinerama_init".as_ptr(),
                    111i32,
                );
                use_root_output(conn);
            }
            ScreenSource::Inactive => {
                debuglog(
                    c"%s:%s:%d - Xinerama is not active (in your X-Server), using root output.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/xinerama.c".as_ptr(),
                    c"xinerama_init".as_ptr(),
                    117i32,
                );
                use_root_output(conn);
            }
            ScreenSource::Xinerama => query_screens(conn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(x: i16, y: i16, width: u16, height: u16) -> xcb_xinerama_screen_info_t {
        xcb_xinerama_screen_info_t {
            x_org: x,
            y_org: y,
            width,
            height,
        }
    }
    fn rect_values(rect: Rect) -> [u32; 4] {
        [rect.x, rect.y, rect.width, rect.height]
    }

    #[test]
    fn signed_origins_and_clone_sizes_keep_legacy_geometry_semantics() {
        let left = screen(-1920, -720, 1920, 1080);
        let mut rect = screen_rect(&left);
        assert_eq!(
            rect_values(rect),
            [(-1920i32) as u32, (-720i32) as u32, 1920, 1080]
        );
        assert!(!origin_is_zero(rect));
        // Clones match their origin, not their name or size, and narrow to the
        // common signed minimum without changing coordinates.
        let clone = screen(-1920, -720, 1000, 1200);
        assert!(same_origin(rect, screen_rect(&clone)));
        shrink_clone(&mut rect, &clone);
        assert_eq!(
            rect_values(rect),
            [(-1920i32) as u32, (-720i32) as u32, 1000, 1080]
        );
        assert!(!same_origin(
            rect,
            screen_rect(&screen(-1919, -720, 1000, 1080))
        ));
        assert!(origin_is_zero(screen_rect(&screen(0, 0, 1, 1))));
        assert_eq!(
            rect_values(screen_rect(&screen(i16::MIN, i16::MAX, u16::MAX, 0))),
            [(-32768i32) as u32, 32767, 65535, 0]
        );
        shrink_clone(&mut rect, &screen(-1920, -720, 0, 1));
        assert_eq!([rect.width, rect.height], [0, 1]);
        // Existing outputs can contain wider unsigned values from other output
        // backends; the old signed min ordering intentionally retains these.
        rect.width = 0x80000000;
        rect.height = u32::MAX;
        shrink_clone(&mut rect, &screen(0, 0, u16::MAX, u16::MAX));
        assert_eq!([rect.width, rect.height], [0x80000000, u32::MAX]);
    }

    fn wire_reply(
        number: u32,
        records: &[xcb_xinerama_screen_info_t],
    ) -> NativeReply<xcb_xinerama_query_screens_reply_t> {
        let bytes = std::mem::size_of_val(records);
        assert_eq!(
            std::mem::size_of::<xcb_xinerama_query_screens_reply_t>(),
            32
        );
        assert_eq!(std::mem::size_of::<xcb_xinerama_screen_info_t>(), 8);
        // SAFETY: calloc supplies a unique, aligned native allocation covering
        // the 32-byte header and its complete advertised record body. Both POD
        // types permit zero initialization; all live fields are written below.
        unsafe {
            let pointer = libc::calloc(1, 32 + bytes).cast::<xcb_xinerama_query_screens_reply_t>();
            assert!(!pointer.is_null());
            pointer.write(xcb_xinerama_query_screens_reply_t {
                response_type: 1,
                pad0: 0,
                sequence: 0,
                length: (bytes / 4) as u32,
                number,
                pad1: [0; 20],
            });
            std::ptr::copy_nonoverlapping(records.as_ptr(), pointer.add(1).cast(), records.len());
            NativeReply::new(pointer).unwrap()
        }
    }

    #[test]
    fn reply_records_are_bounded_by_body_length_and_keep_native_order() {
        let records = [screen(-1, 2, 3, 4), screen(0, 0, 65535, 1)];
        let reply = wire_reply(2, &records);
        let rects: Vec<_> = reply
            .screens()
            .unwrap()
            .iter()
            .map(|screen| rect_values(screen_rect(screen)))
            .collect();
        assert_eq!(rects, [[u32::MAX, 2, 3, 4], [0, 0, 65535, 1]]);
        // A count beyond the allocated reply is rejected before creating any
        // slice. An empty count and unused trailing body records remain valid.
        assert!(wire_reply(3, &records).screens().is_none());
        assert!(wire_reply(0, &[]).screens().unwrap().is_empty());
        assert_eq!(wire_reply(1, &records).screens().unwrap().len(), 1);
        assert_eq!(checked_screen_count(1, 1), None);
        assert_eq!(checked_screen_count(u32::MAX, u32::MAX), None);
        assert_eq!(checked_screen_count(0, 0), Some(0));
    }

    #[test]
    fn root_fallback_distinguishes_missing_extension_and_inactive_or_failed_query() {
        for active in [None, Some(false), Some(true)] {
            assert_eq!(screen_source(false, active), ScreenSource::MissingExtension);
        }
        assert_eq!(screen_source(true, None), ScreenSource::Inactive);
        assert_eq!(screen_source(true, Some(false)), ScreenSource::Inactive);
        assert_eq!(screen_source(true, Some(true)), ScreenSource::Xinerama);
    }
}
