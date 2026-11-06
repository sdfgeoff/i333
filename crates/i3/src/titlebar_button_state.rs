//! Pointer interaction state for ordinary window-titlebar buttons.
//! Targets use arena generations and X window identities, never retained graph borrows.
use std::cell::Cell;

use crate::con::arena::{identity, resolve, ConId};
use crate::titlebar_buttons::{layout_for, ButtonState, TitlebarButton};
use crate::types::Con;
use xcb::{x, Xid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Target {
    con: ConId,
    frame: u32,
    window: u32,
    button: TitlebarButton,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Press<T = Target> {
    target: T,
    armed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Interaction<T = Target> {
    hovered: Option<T>,
    pressed: Option<Press<T>>,
}

impl<T> Default for Interaction<T> {
    fn default() -> Self {
        Self {
            hovered: None,
            pressed: None,
        }
    }
}

impl<T: Copy + Eq> Interaction<T> {
    fn press(&mut self, target: Option<T>, unmodified: bool) -> bool {
        self.hovered = target;
        self.pressed = if unmodified {
            target.map(|target| Press {
                target,
                armed: true,
            })
        } else {
            None
        };
        self.pressed.is_some()
    }

    fn motion(&mut self, target: Option<T>) {
        self.hovered = target;
        if let Some(press) = self.pressed.as_mut() {
            if target != Some(press.target) {
                // Crossing out permanently cancels this press, even on reentry.
                press.armed = false;
            }
        }
    }

    fn release(&mut self, target: Option<T>, unmodified: bool) -> (bool, Option<T>) {
        let press = self.pressed.take();
        self.hovered = target;
        match press {
            Some(press) => (
                true,
                (press.armed && unmodified && target == Some(press.target)).then_some(press.target),
            ),
            None => (false, None),
        }
    }
}

thread_local! {
    static POINTER: Cell<Interaction> = Cell::new(Interaction::default());
}

unsafe fn valid(target: Target) -> Option<*mut Con> {
    let con = resolve(target.con)?;
    if (*con).frame.id != target.frame
        || (*con).window.is_null()
        || (*(*con).window).id != target.window
        || !layout_for(con)
            .buttons
            .iter()
            .any(|item| item.button == target.button)
    {
        return None;
    }
    Some(con)
}

unsafe fn hit(frame: u32, x: i32, y: i32) -> Option<Target> {
    let con = crate::con::con_by_frame_id(frame);
    let id = identity(con)?;
    let button = layout_for(con).hit_test(x, y)?;
    if (*con).window.is_null() {
        return None;
    }
    Some(Target {
        con: id,
        frame,
        window: (*(*con).window).id,
        button,
    })
}

/// Event-thread only; the supplied container must be live for this lookup.
pub(crate) unsafe fn state_for(con: *mut Con) -> ButtonState {
    let Some(id) = identity(con) else {
        return ButtonState::default();
    };
    let state = POINTER.with(Cell::get);
    let hovered = state
        .hovered
        .filter(|target| target.con == id && valid(*target).is_some());
    let pressed = state
        .pressed
        .filter(|press| press.armed && Some(press.target) == hovered);
    ButtonState {
        hovered: hovered.map(|target| target.button),
        pressed: pressed.map(|press| press.target.button),
    }
}

unsafe fn repaint_if_changed(before: Interaction, after: Interaction) {
    if before != after && !crate::tree::croot.is_null() {
        crate::x::x_push_changes(crate::tree::croot);
    }
}

pub(crate) unsafe fn motion(frame: u32, x: i32, y: i32) {
    let target = hit(frame, x, y);
    let before = POINTER.with(Cell::get);
    let mut after = before;
    after.motion(target);
    POINTER.with(|state| state.set(after));
    repaint_if_changed(before, after);
}

pub(crate) unsafe fn leave(frame: u32) {
    let before = POINTER.with(Cell::get);
    let mut after = before;
    if after.hovered.is_some_and(|target| target.frame == frame) {
        after.hovered = None;
    }
    if let Some(press) = after.pressed.as_mut() {
        if press.target.frame == frame {
            press.armed = false;
        }
    }
    POINTER.with(|state| state.set(after));
    repaint_if_changed(before, after);
}

fn unmodified(state: u32, numlock: u32) -> bool {
    state & 0xff & !(x::KeyButMask::LOCK.bits() | numlock) == 0
}

/// Claim only plain left clicks on our controls. Claimed releases are consumed
/// even after cancellation, reload, or destruction of the original target.
pub(crate) unsafe fn button(event: &x::ButtonPressEvent) -> bool {
    if event.detail() != 1 {
        return false;
    }
    let kind = event.response_type() & 0x7f;
    if kind != 4 && kind != 5 {
        return false;
    }
    let target = hit(
        event.event().resource_id(),
        i32::from(event.event_x()),
        i32::from(event.event_y()),
    );
    let mask = u32::from(xcb_interop::events::button_press_state(event));
    let plain = unmodified(mask, crate::xcb::xcb_numlock_mask)
        && mask & (0x1f00 & !x::KeyButMask::BUTTON1.bits()) == 0;
    let before = POINTER.with(Cell::get);
    let mut after = before;
    let action;
    let claimed;
    if kind == 4 {
        // A new physical press also retires an abandoned claim whose frame was
        // destroyed before the server could deliver its release.
        claimed = after.press(target, plain);
        action = None;
    } else {
        (claimed, action) = after.release(target, plain);
    }
    POINTER.with(|state| state.set(after));
    if !claimed {
        repaint_if_changed(before, after);
        return false;
    }
    crate::application::last_timestamp = event.time();
    let connection = i3_common::display::connection().expect("active X11 display");
    connection.send_request(&x::AllowEvents {
        mode: x::Allow::AsyncPointer,
        time: event.time(),
    });
    // All interaction state is committed before an action can destroy a node.
    if let Some(target) = action {
        if let Some(con) = valid(target) {
            match target.button {
                TitlebarButton::Fullscreen => {
                    crate::con::con_toggle_fullscreen(con, crate::domains::FullscreenMode::Output)
                }
                TitlebarButton::Close => {
                    crate::con::con_close(con, crate::types::KillWindow::Client)
                }
            }
            crate::tree::tree_render();
            return true;
        }
    }
    repaint_if_changed(before, after);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_activates_original_target_once_and_only_without_modifiers() {
        let mut state = Interaction::<u8>::default();
        assert!(state.press(Some(1), true));
        assert_eq!(state.release(Some(1), true), (true, Some(1)));
        assert_eq!(state.release(Some(1), true), (false, None));
        assert!(state.press(Some(1), true));
        assert_eq!(state.release(Some(1), false), (true, None));
        assert!(!state.press(Some(1), false));
        assert_eq!(state.release(Some(1), true), (false, None));
    }

    #[test]
    fn leaving_cancels_permanently_but_still_consumes_release() {
        let mut state = Interaction::<u8>::default();
        assert!(state.press(Some(1), true));
        state.motion(None);
        state.motion(Some(1));
        assert_eq!(state.release(Some(1), true), (true, None));
        assert!(state.press(Some(1), true));
        state.motion(Some(2));
        assert_eq!(state.release(Some(2), true), (true, None));
        assert!(state.press(Some(1), true));
        assert!(state.press(Some(2), true));
        assert_eq!(state.release(Some(2), true), (true, Some(2)));
    }

    #[test]
    fn modifiers_preserve_bindings_but_ignore_lock_keys() {
        assert!(unmodified(0, 0));
        assert!(unmodified(
            x::KeyButMask::LOCK.bits() | x::KeyButMask::MOD2.bits(),
            x::KeyButMask::MOD2.bits()
        ));
        assert!(!unmodified(x::KeyButMask::SHIFT.bits(), 0));
        assert!(!unmodified(x::KeyButMask::MOD4.bits(), 0));
        assert!(unmodified(x::KeyButMask::BUTTON1.bits(), 0));
    }
}
