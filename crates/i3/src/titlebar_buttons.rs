//! Shared titlebar control geometry, expressed in parent-frame coordinates.
use crate::con::arena::parent::parent_ptr;
use crate::domains::{ContainerType, FullscreenMode};
use crate::types::{BorderStyle, Con, Layout, Rect, TitlebarButtons};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TitlebarButton {
    Fullscreen,
    Close,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ButtonState {
    pub hovered: Option<TitlebarButton>,
    pub pressed: Option<TitlebarButton>,
}

#[derive(Clone, Copy)]
pub struct ButtonRect {
    pub button: TitlebarButton,
    pub rect: Rect,
}

#[derive(Default)]
pub struct ButtonLayout {
    pub buttons: Vec<ButtonRect>,
    pub reserved_width: u32,
}
impl ButtonLayout {
    pub fn hit_test(&self, x: i32, y: i32) -> Option<TitlebarButton> {
        self.buttons.iter().find_map(|button| {
            let rect = button.rect;
            let dx = i64::from(x) - i64::from(rect.x as i32);
            let dy = i64::from(y) - i64::from(rect.y as i32);
            (dx >= 0 && dy >= 0 && dx < i64::from(rect.width) && dy < i64::from(rect.height))
                .then_some(button.button)
        })
    }
}

fn geometry(deco: Rect, enabled: TitlebarButtons, padding: u32) -> ButtonLayout {
    let mut layout = ButtonLayout::default();
    let padding = padding.max(1);
    let size = deco.height.saturating_sub(padding.saturating_mul(2));
    // Tiny decorations have no usable icon or target; never grow outside them.
    if size < padding.saturating_mul(3) {
        return layout;
    }
    for (button, enabled) in [
        (TitlebarButton::Close, enabled.close),
        (TitlebarButton::Fullscreen, enabled.fullscreen),
    ] {
        if !enabled {
            continue;
        }
        let reserved = if layout.buttons.is_empty() {
            padding
        } else {
            layout.reserved_width
        }
        .saturating_add(size)
        .saturating_add(padding);
        if reserved > deco.width {
            continue;
        }
        layout.buttons.push(ButtonRect {
            button,
            rect: Rect {
                x: deco.x.wrapping_add(deco.width - reserved + padding),
                y: deco.y.wrapping_add(padding),
                width: size,
                height: size,
            },
        });
        layout.reserved_width = reserved;
    }
    layout
}

fn eligible(
    window: bool,
    leaf: bool,
    kind: ContainerType,
    border: BorderStyle,
    fullscreen: FullscreenMode,
    parent: Option<Layout>,
) -> bool {
    window
        && leaf
        && kind == ContainerType::Con
        && border == BorderStyle::Normal
        && fullscreen == FullscreenMode::None
        && parent.is_some_and(|layout| layout != Layout::Tabbed && layout != Layout::Stacked)
}

/// # Safety
/// `con` is a live event-thread graph handle. Its parent/child links and global
/// configuration must remain valid while this owned geometry is computed.
pub unsafe fn layout_for(con: *mut Con) -> ButtonLayout {
    if con.is_null() {
        return ButtonLayout::default();
    }
    let parent = parent_ptr(con);
    let enabled = unsafe { crate::config::config.titlebar_buttons };
    if !enabled.fullscreen && !enabled.close {
        return ButtonLayout::default();
    }
    let allowed = unsafe {
        eligible(
            !(*con).window.is_null(),
            crate::con::con_is_leaf(con),
            (*con).type_0,
            (*con).border_style,
            (*con).fullscreen_mode,
            (!parent.is_null()).then(|| (*parent).layout),
        )
    };
    if !allowed {
        return ButtonLayout::default();
    }
    geometry(
        unsafe { (*con).deco_rect },
        enabled,
        i3_common::dpi::logical_px(2).max(1) as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rect(width: u32, height: u32) -> Rect {
        Rect {
            x: 10,
            y: 20,
            width,
            height,
        }
    }
    fn both() -> TitlebarButtons {
        TitlebarButtons {
            fullscreen: true,
            close: true,
        }
    }
    #[test]
    fn layout_orders_controls_and_reserves_padding() {
        let layout = geometry(rect(100, 20), both(), 2);
        assert_eq!(layout.reserved_width, 38);
        assert_eq!(layout.buttons.len(), 2);
        assert_eq!(layout.hit_test(105, 23), Some(TitlebarButton::Close));
        assert_eq!(layout.hit_test(80, 23), Some(TitlebarButton::Fullscreen));
        assert_eq!(layout.hit_test(109, 23), None);
        assert_eq!(layout.hit_test(105, 20), None);
        assert_eq!(layout.hit_test(-1, 23), None);
    }
    #[test]
    fn narrow_and_scaled_decorations_never_extend_outside_title() {
        assert_eq!(geometry(rect(19, 20), both(), 2).buttons.len(), 0);
        let narrow = geometry(rect(20, 20), both(), 2);
        assert_eq!(narrow.buttons.len(), 1);
        assert_eq!(narrow.buttons[0].button, TitlebarButton::Close);
        let scaled = geometry(rect(200, 40), both(), 4);
        assert_eq!(scaled.reserved_width, 76);
        assert!(geometry(rect(200, 4), both(), 2).buttons.is_empty());
        assert!(geometry(rect(200, 20), TitlebarButtons::default(), 2)
            .buttons
            .is_empty());
        let signed = geometry(
            Rect {
                x: (-100i32) as u32,
                ..rect(100, 20)
            },
            both(),
            2,
        );
        assert_eq!(signed.hit_test(-10, 23), Some(TitlebarButton::Close));
    }
    #[test]
    fn only_normal_leaf_client_decorations_are_eligible() {
        let yes = |window, leaf, kind, border, fullscreen, parent| {
            eligible(window, leaf, kind, border, fullscreen, parent)
        };
        assert!(yes(
            true,
            true,
            ContainerType::Con,
            BorderStyle::Normal,
            FullscreenMode::None,
            Some(Layout::SplitH)
        ));
        for layout in [Layout::Tabbed, Layout::Stacked] {
            assert!(!yes(
                true,
                true,
                ContainerType::Con,
                BorderStyle::Normal,
                FullscreenMode::None,
                Some(layout)
            ));
        }
        assert!(!yes(
            false,
            true,
            ContainerType::Con,
            BorderStyle::Normal,
            FullscreenMode::None,
            Some(Layout::SplitH)
        ));
        assert!(!yes(
            true,
            false,
            ContainerType::Con,
            BorderStyle::Normal,
            FullscreenMode::None,
            Some(Layout::SplitH)
        ));
        assert!(!yes(
            true,
            true,
            ContainerType::Con,
            BorderStyle::Pixel,
            FullscreenMode::None,
            Some(Layout::SplitH)
        ));
        assert!(!yes(
            true,
            true,
            ContainerType::Con,
            BorderStyle::Normal,
            FullscreenMode::Output,
            Some(Layout::SplitH)
        ));
        assert!(!yes(
            true,
            true,
            ContainerType::Con,
            BorderStyle::Normal,
            FullscreenMode::None,
            None
        ));
    }
}
