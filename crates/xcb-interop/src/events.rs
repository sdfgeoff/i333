//! Preserve complete native input state, including XKB group and unknown bits.
//!
//! xcb1.7.1's generated state getters truncate values to known KeyButMask bits.
//! These concrete maintained event types each own a complete 32-byte input body.
use xcb::{x, Raw};

/// # Safety
/// raw points to the complete live native input body owned by a concrete event.
unsafe fn native_input_state(raw: *const u8) -> u16 {
    // This shared fixed-width field is initialized in every input event body.
    unsafe { raw.add(28).cast::<u16>().read_unaligned() }
}

macro_rules! state_accessor {
    ($name:ident, $event:ty) => {
        /// Return all sixteen native state bits without discarding XKB groups.
        pub fn $name(event: &$event) -> u16 {
            // SAFETY: this concrete maintained type owns a complete fixed input
            // event, unlike an arbitrary safely implemented Raw trait object.
            unsafe { native_input_state(event.as_raw().cast()) }
        }
    };
}
state_accessor!(key_press_state, x::KeyPressEvent);
state_accessor!(key_release_state, x::KeyReleaseEvent);
state_accessor!(button_press_state, x::ButtonPressEvent);
state_accessor!(button_release_state, x::ButtonReleaseEvent);
state_accessor!(motion_state, x::MotionNotifyEvent);
state_accessor!(enter_notify_state, x::EnterNotifyEvent);
state_accessor!(leave_notify_state, x::LeaveNotifyEvent);

/// The XKB state fields consumed by the window manager and bar.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XkbState {
    pub modifiers: u8,
    pub group: u8,
    pub time: u32,
}

/// Native change masks remain open so new/unknown bits are preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct XkbKeyboardChange {
    pub sequence: u16,
    pub time: u32,
    pub changed: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XkbEvent {
    NewKeyboard(XkbKeyboardChange),
    Map(XkbKeyboardChange),
    State(XkbState),
}

/// Decode only the consumed XKB subtypes from an owned maintained event.
///
/// XKB shares one response code across subtypes. Validate both tags before
/// borrowing a canonical view; returned values cannot outlive the event body.
pub fn xkb_event(event: &xcb::Event, event_base: u8) -> Option<XkbEvent> {
    use std::mem::ManuallyDrop;
    use xcb::{xkb, BaseEvent};
    const NEW: u8 = xkb::NewKeyboardNotifyEvent::NUMBER as u8;
    const MAP: u8 = xkb::MapNotifyEvent::NUMBER as u8;
    const STATE: u8 = xkb::StateNotifyEvent::NUMBER as u8;
    let raw = event.as_raw();
    let bytes = raw.cast::<u8>();
    // Every concrete/unknown maintained event owns at least its fixed 32-byte
    // body. SendEvent's high bit does not change the native response code.
    if unsafe { bytes.read() } & 0x7f != event_base {
        return None;
    }
    let subtype = unsafe { bytes.add(1).read() };
    match subtype {
        NEW => {
            // SAFETY: checked base/subtype select a complete fixed XKB body.
            // The original event remains its sole allocation owner; suppress
            // destruction of this local canonical borrowed view.
            let view = ManuallyDrop::new(unsafe { xkb::NewKeyboardNotifyEvent::from_raw(raw) });
            Some(XkbEvent::NewKeyboard(XkbKeyboardChange {
                sequence: view.sequence(),
                time: view.time(),
                changed: unsafe { bytes.add(16).cast::<u16>().read_unaligned() },
            }))
        }
        MAP => {
            let view = ManuallyDrop::new(unsafe { xkb::MapNotifyEvent::from_raw(raw) });
            Some(XkbEvent::Map(XkbKeyboardChange {
                sequence: view.sequence(),
                time: view.time(),
                changed: unsafe { bytes.add(10).cast::<u16>().read_unaligned() },
            }))
        }
        STATE => {
            let view = ManuallyDrop::new(unsafe { xkb::StateNotifyEvent::from_raw(raw) });
            Some(XkbEvent::State(XkbState {
                modifiers: view.mods().bits() as u8,
                // The generated group getter transmutes into a closed enum.
                group: unsafe { bytes.add(13).read() },
                time: view.time(),
            }))
        }
        _ => None,
    }
}

pub fn xkb_state(event: &xcb::Event, event_base: u8) -> Option<XkbState> {
    match xkb_event(event, event_base) {
        Some(XkbEvent::State(state)) => Some(state),
        _ => None,
    }
}

/// Retain the native client-message union's five words for every format byte.
/// Legacy consumers read this union view even for byte/short/unknown formats;
/// the maintained data() getter instead rejects unknown format discriminants.
pub fn client_message_words(event: &x::ClientMessageEvent) -> [u32; 5] {
    let raw = event.as_raw().cast::<u8>();
    // SAFETY: the concrete event owns the complete fixed 20-byte union payload.
    std::array::from_fn(|index| unsafe { raw.add(12 + index * 4).cast::<u32>().read_unaligned() })
}

/// Preserve unknown visibility selectors instead of forming an invalid enum.
pub fn visibility_state(event: &x::VisibilityNotifyEvent) -> u8 {
    // SAFETY: the concrete event owns the fixed body containing this byte.
    unsafe { event.as_raw().cast::<u8>().add(8).read() }
}

/// Preserve every native property selector, including unknown future values.
pub fn property_notify_state(event: &x::PropertyNotifyEvent) -> u8 {
    // SAFETY: the concrete event owns the fixed body containing this byte.
    unsafe { event.as_raw().cast::<u8>().add(16).read() }
}

/// Build the canonical DestroyNotify and initialize its complete sendable body.
pub fn destroy_notify(event: x::Window, window: x::Window) -> x::DestroyNotifyEvent {
    let event = x::DestroyNotifyEvent::new(event, window);
    // xcb1.7.1's constructor leaves these padding bytes unwritten. SendEvent
    // copies all 32 bytes, so initialize padding before the packet can be sent.
    unsafe {
        let raw = event.as_raw().cast::<u8>();
        raw.add(1).write(0);
        raw.add(12).write_bytes(0, 20);
    }
    event
}

#[cfg(test)]
mod tests {
    use super::*;
    use xcb::Xid;

    #[test]
    fn every_concrete_input_event_retains_every_native_u16_state() {
        for bits in 0..=u16::MAX {
            let mask = x::KeyButMask::from_bits_retain(u32::from(bits));
            let none = x::Window::none();
            let key = x::KeyPressEvent::new(8, 0, none, none, none, 0, 0, 0, 0, mask, true);
            assert_eq!(key_press_state(&key), bits);
            let release = x::KeyReleaseEvent::new(8, 0, none, none, none, 0, 0, 0, 0, mask, true);
            assert_eq!(key_release_state(&release), bits);
            let button = x::ButtonPressEvent::new(6, 0, none, none, none, 0, 0, 0, 0, mask, true);
            assert_eq!(button_press_state(&button), bits);
            let release =
                x::ButtonReleaseEvent::new(6, 0, none, none, none, 0, 0, 0, 0, mask, true);
            assert_eq!(button_release_state(&release), bits);
            let motion = x::MotionNotifyEvent::new(
                x::Motion::Normal,
                0,
                none,
                none,
                none,
                0,
                0,
                0,
                0,
                mask,
                true,
            );
            assert_eq!(motion_state(&motion), bits);
            let enter = x::EnterNotifyEvent::new(
                x::NotifyDetail::Ancestor,
                0,
                none,
                none,
                none,
                0,
                0,
                0,
                0,
                mask,
                x::NotifyMode::Normal,
                0,
            );
            assert_eq!(enter_notify_state(&enter), bits);
            let leave = x::LeaveNotifyEvent::new(
                x::NotifyDetail::Ancestor,
                0,
                none,
                none,
                none,
                0,
                0,
                0,
                0,
                mask,
                x::NotifyMode::Normal,
                0,
            );
            assert_eq!(leave_notify_state(&leave), bits);
        }
    }

    fn xkb_packet(response: u8, subtype: u8, modifiers: u8, group: u8, changed: u16) -> xcb::Event {
        // Independent incoming wire fixtures, including native receive metadata.
        // No legacy C packet type or potentially invalid Rust enum is formed.
        unsafe {
            let raw = libc::calloc(1, 36).cast::<u8>();
            assert!(!raw.is_null());
            raw.write(response);
            raw.add(1).write(subtype);
            raw.add(2).cast::<u16>().write_unaligned(0xabcd);
            raw.add(4).cast::<u32>().write_unaligned(0x76543210);
            raw.add(9).write(modifiers);
            raw.add(13).write(group);
            match subtype {
                0 => raw.add(16).cast::<u16>().write_unaligned(changed),
                1 => raw.add(10).cast::<u16>().write_unaligned(changed),
                _ => {}
            }
            xcb::Event::Unknown(xcb::UnknownEvent::from_raw(raw.cast()))
        }
    }

    #[test]
    fn xkb_snapshots_check_base_subtype_and_send_event_mask_before_decoding() {
        for response in [85, 85 | 0x80] {
            for subtype in 0..=u8::MAX {
                let packet = xkb_packet(response, subtype, 0xff, 0xff, 0xffff);
                assert_eq!(xkb_event(&packet, 85).is_some(), subtype <= 2);
                assert_eq!(xkb_state(&packet, 85).is_some(), subtype == 2);
                assert!(xkb_event(&packet, 84).is_none());
            }
        }
    }

    #[test]
    fn xkb_state_preserves_all_modifier_and_unknown_group_bytes_after_owner_drop() {
        for modifiers in 0..=u8::MAX {
            for group in 0..=u8::MAX {
                let state = {
                    let packet = xkb_packet(85, 2, modifiers, group, 0);
                    xkb_state(&packet, 85).unwrap()
                };
                assert_eq!(
                    state,
                    XkbState {
                        modifiers,
                        group,
                        time: 0x76543210
                    }
                );
            }
        }
    }

    #[test]
    fn xkb_new_keyboard_and_map_snapshots_preserve_all_changed_bits() {
        for changed in 0..=u16::MAX {
            for subtype in [0, 1] {
                let packet = xkb_packet(85 | 0x80, subtype, 0, 0, changed);
                let snapshot = xkb_event(&packet, 85).unwrap();
                let expected = XkbKeyboardChange {
                    sequence: 0xabcd,
                    time: 0x76543210,
                    changed,
                };
                assert_eq!(
                    snapshot,
                    if subtype == 0 {
                        XkbEvent::NewKeyboard(expected)
                    } else {
                        XkbEvent::Map(expected)
                    }
                );
            }
        }
    }

    #[test]
    fn client_message_union_words_preserve_all_format_bytes() {
        for format in 0..=u8::MAX {
            unsafe {
                let raw = libc::calloc(1, 32).cast::<u8>();
                assert!(!raw.is_null());
                raw.write(33);
                raw.add(1).write(format);
                let expected = [0, 0x01020304, 0x80000000, 0xaabbccdd, u32::MAX];
                for (index, word) in expected.iter().enumerate() {
                    raw.add(12 + index * 4).cast::<u32>().write_unaligned(*word);
                }
                let event = x::ClientMessageEvent::from_raw(raw.cast());
                assert_eq!(client_message_words(&event), expected);
            }
        }
    }

    #[test]
    fn visibility_preserves_every_native_selector() {
        for state in 0..=u8::MAX {
            unsafe {
                let raw = libc::calloc(1, 32).cast::<u8>();
                assert!(!raw.is_null());
                raw.write(15);
                raw.add(8).write(state);
                let event = x::VisibilityNotifyEvent::from_raw(raw.cast());
                assert_eq!(visibility_state(&event), state);
            }
        }
    }

    #[test]
    fn destroy_notification_has_the_exact_initialized_native_wire_body() {
        use xcb::XidNew;
        let parent = unsafe { x::Window::new(0x01020304) };
        let window = unsafe { x::Window::new(0xaabbccdd) };
        let event = destroy_notify(parent, window);
        let mut expected = [0; 32];
        expected[0] = 17;
        expected[4..8].copy_from_slice(&0x01020304_u32.to_ne_bytes());
        expected[8..12].copy_from_slice(&0xaabbccdd_u32.to_ne_bytes());
        let bytes = unsafe { std::slice::from_raw_parts(event.as_raw().cast::<u8>(), 32) };
        assert_eq!(bytes, expected);
    }

    #[test]
    fn property_notification_preserves_every_native_selector() {
        for state in 0..=u8::MAX {
            unsafe {
                let raw = libc::calloc(1, 32).cast::<u8>();
                assert!(!raw.is_null());
                raw.write(28);
                raw.add(16).write(state);
                let event = x::PropertyNotifyEvent::from_raw(raw.cast());
                assert_eq!(property_notify_state(&event), state);
            }
        }
    }
}
