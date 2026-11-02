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
}
