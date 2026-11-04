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

use crate::output::output_primary_name;
use crate::randr::{create_root_output, init_ws_for_output, output_init_con, outputs};
use crate::types::{output_name, xoutput, xoutput as Output, Rect};
use ::xcb::xinerama::{QueryScreensReply, ScreenInfo};
use ::xcb::{x, xinerama, Connection, Reply};
use i3_common::display;
use i3_common::safewrappers::scalloc;
use std::sync::atomic::{AtomicI32, Ordering};

// Survives repeated initialization calls, including clone-only queries.
static NUM_SCREENS: AtomicI32 = AtomicI32::new(0);

fn screen_rect(screen: &ScreenInfo) -> Rect {
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
fn shrink_clone(rect: &mut Rect, screen: &ScreenInfo) {
    // Existing unsigned extents passed through signed int in the original min.
    rect.width = (rect.width as i32).min(i32::from(screen.width)) as u32;
    rect.height = (rect.height as i32).min(i32::from(screen.height)) as u32;
}

fn checked_screen_count(number: u32, length: u32) -> Option<usize> {
    let count = usize::try_from(number).ok()?;
    let required = count.checked_mul(std::mem::size_of::<ScreenInfo>())?;
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

/// Validate the wire count before calling the generated slice accessor. The
/// maintained reply owns the allocation; only this private header field needs
/// raw access because its generated getter is not public.
fn screens(reply: &QueryScreensReply) -> Option<&[ScreenInfo]> {
    // SAFETY: a maintained reply owns its complete initialized 32-byte header.
    let number = unsafe { reply.as_raw().add(8).cast::<u32>().read_unaligned() };
    checked_screen_count(number, reply.length())?;
    Some(reply.screen_info())
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
unsafe fn apply_screen(screen: &ScreenInfo) {
    unsafe {
        let mut s: *mut Output = get_screen_at(screen_rect(screen));
        if !s.is_null() {
            ::log::debug!(
                "../src/xinerama.c:query_screens:50 - Re-used old Xinerama screen {}\n",
                i3_common::logging::pointer(s)
            );
            shrink_clone(&mut (*s).rect, screen);
        } else {
            s = scalloc(1_usize, ::core::mem::size_of::<Output>() as usize) as *mut Output;
            let name = format!("xinerama-{}", NUM_SCREENS.load(Ordering::Relaxed));
            let output_name = Box::into_raw(Box::new(output_name::new(name.as_bytes())));
            (*s).names_head.slh_first = ::core::ptr::null_mut::<output_name>();
            (*output_name).names.sle_next = (*s).names_head.slh_first;
            (*s).names_head.slh_first = output_name;
            ::log::debug!(
                "../src/xinerama.c:query_screens:61 - Created new Xinerama screen {} ({})\n",
                name,
                i3_common::logging::pointer(s)
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
                outputs.tqh_first = s;
                (*s).outputs.tqe_prev = &raw mut outputs.tqh_first;
            } else {
                (*s).outputs.tqe_next = ::core::ptr::null_mut::<xoutput>();
                (*s).outputs.tqe_prev = outputs.tqh_last;
                *outputs.tqh_last = s;
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
unsafe fn query_screens(connection: &Connection) {
    unsafe {
        // Keep protocol errors on the event queue, as the original unchecked
        // query did; the owned reply is released after applying its records.
        let cookie = connection.send_request_unchecked(&xinerama::QueryScreens {});
        let reply = connection.wait_for_reply_unchecked(cookie).ok().flatten();
        let Some(reply) = reply else {
            ::log::error!("ERROR: Couldn't get Xinerama screens\n");
            return;
        };
        let Some(screens) = screens(&reply) else {
            ::log::error!("ERROR: Invalid Xinerama screen reply\n");
            return;
        };
        for screen in screens {
            apply_screen(screen);
            ::log::debug!(
                "../src/xinerama.c:query_screens:80 - found Xinerama screen: {} x {} at {} x {}\n",
                { (i32::from(screen.width)) },
                { (i32::from(screen.height)) },
                { (i32::from(screen.x_org)) },
                { (i32::from(screen.y_org)) }
            );
        }
    }
    if NUM_SCREENS.load(Ordering::Relaxed) == 0 {
        unsafe {
            ::log::error!("ERROR: No screens found. Please fix your setup. i3 will exit now.\n");
            libc::exit(libc::EXIT_SUCCESS);
        }
    }
}

/// # Safety
/// The display and output/render/workspace graphs are initialized and exclusively
/// owned by this event thread. create_root_output uses the native allocator.
unsafe fn use_root_output() {
    unsafe {
        let output = create_root_output();
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
        let connection = display::connection();
        let Some(connection) = connection else {
            use_root_output();
            return;
        };
        // Query through the maintained API rather than its generated cached
        // accessor, which asserts on a missing reply after connection failure.
        // Event decoding deliberately leaves extension metadata unregistered so
        // unknown extension events can reach the existing tolerant dispatcher.
        let cookie = connection.send_request(&x::QueryExtension { name: b"XINERAMA" });
        let present = connection
            .wait_for_reply(cookie)
            .ok()
            .is_some_and(|reply| reply.present());
        // Missing extension must not issue an IsActive request.
        let active = if present {
            let cookie = connection.send_request(&xinerama::IsActive {});
            connection
                .wait_for_reply(cookie)
                .ok()
                .map(|reply| reply.state() != 0)
        } else {
            None
        };
        match screen_source(present, active) {
            ScreenSource::MissingExtension => {
                ::log::debug!("../src/xinerama.c:xinerama_init:111 - Xinerama extension not found, using root output.\n");
                use_root_output();
            }
            ScreenSource::Inactive => {
                ::log::debug!("../src/xinerama.c:xinerama_init:117 - Xinerama is not active (in your X-Server), using root output.\n");
                use_root_output();
            }
            ScreenSource::Xinerama => query_screens(&connection),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(x: i16, y: i16, width: u16, height: u16) -> ScreenInfo {
        ScreenInfo {
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

    fn wire_reply(number: u32, records: &[ScreenInfo]) -> QueryScreensReply {
        let bytes = std::mem::size_of_val(records);
        assert_eq!(std::mem::size_of::<ScreenInfo>(), 8);
        // SAFETY: calloc supplies unique aligned storage for the header and
        // advertised body. Write scalar wire fields and initialized records;
        // ownership transfers once to the maintained reply destructor.
        unsafe {
            let pointer = libc::calloc(1, 32 + bytes).cast::<u8>();
            assert!(!pointer.is_null());
            pointer.write(1);
            pointer.add(4).cast::<u32>().write((bytes / 4) as u32);
            pointer.add(8).cast::<u32>().write(number);
            std::ptr::copy_nonoverlapping(records.as_ptr(), pointer.add(32).cast(), records.len());
            QueryScreensReply::from_raw(pointer)
        }
    }

    #[test]
    fn reply_records_are_bounded_by_body_length_and_keep_native_order() {
        let records = [screen(-1, 2, 3, 4), screen(0, 0, 65535, 1)];
        let reply = wire_reply(2, &records);
        let rects: Vec<_> = screens(&reply)
            .unwrap()
            .iter()
            .map(|screen| rect_values(screen_rect(screen)))
            .collect();
        assert_eq!(rects, [[u32::MAX, 2, 3, 4], [0, 0, 65535, 1]]);
        // A count beyond the allocated reply is rejected before creating any
        // slice. An empty count and unused trailing body records remain valid.
        assert!(screens(&wire_reply(3, &records)).is_none());
        assert!(screens(&wire_reply(0, &[])).unwrap().is_empty());
        assert_eq!(screens(&wire_reply(1, &records)).unwrap().len(), 1);
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
