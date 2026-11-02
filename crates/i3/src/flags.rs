//! Application flag domains. Native storage and protocols use explicit raw bits.
//!
//! All bits are retained, including during complement, to preserve legacy
//! full-width integer operations. Zero has wildcard meaning only where the
//! corresponding policy explicitly says so.
use bitflags::bitflags;

macro_rules! flags {
    ($name:ident { $($flag:ident = $bits:expr;)* }) => {
        bitflags! {
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct $name: u32 {
                $(const $flag = $bits;)*
                const _ = !0;
            }
        }
    };
}
flags!(ResizeEdges { LEFT = 1; RIGHT = 2; TOP = 4; BOTTOM = 8; });
flags!(AdjacentEdges { LEFT = 1; RIGHT = 2; TOP = 4; BOTTOM = 16; });
flags!(GapMask {
    INNER = 1; TOP = 2; RIGHT = 4; BOTTOM = 8; LEFT = 16;
    OUTER = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
    HORIZONTAL = Self::RIGHT.bits() | Self::LEFT.bits();
    VERTICAL = Self::TOP.bits() | Self::BOTTOM.bits();
});
flags!(AssignmentKind {
    COMMAND = 1; TO_WORKSPACE = 2; NO_FOCUS = 4;
    TO_WORKSPACE_NUMBER = 8; TO_OUTPUT = 16;
});
flags!(TilingDrag { MODIFIER = 1; TITLEBAR = 2; MODIFIER_OR_TITLEBAR = 3; });
flags!(KeyboardGroups { GROUP1 = 1; GROUP2 = 2; GROUP3 = 4; GROUP4 = 8; });
impl AssignmentKind {
    pub fn accepts(self, filter: Self) -> bool {
        filter.is_empty() || self.intersects(filter)
    }
}
impl KeyboardGroups {
    pub fn from_state_mask(mask: u32) -> Self {
        Self::from_bits_retain(mask >> 16)
    }
    pub fn state_mask(self) -> u32 {
        self.bits() << 16
    }
    pub fn matches_group(self, group: u32) -> bool {
        self.is_empty() || group < 4 && self.intersects(Self::from_bits_retain(1 << group))
    }
    pub fn translation_group(self) -> u32 {
        if self.contains(Self::GROUP2) {
            1
        } else if self.contains(Self::GROUP3) {
            2
        } else if self.contains(Self::GROUP4) {
            3
        } else {
            0
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn raw_roundtrip_complement_and_pod_layout_retain_all_bits() {
        macro_rules! check { ($($ty:ty),*) => {$(
            assert_eq!(std::mem::size_of::<$ty>(), std::mem::size_of::<u32>());
            assert_eq!(std::mem::align_of::<$ty>(), std::mem::align_of::<u32>());
            for raw in [0,1,3,15,31,0x8000_0000,u32::MAX] {
                let value=<$ty>::from_bits_retain(raw);
                assert_eq!(value.bits(),raw);
                assert_eq!((!value).bits(),!raw);
                assert_eq!(value | !value, <$ty>::all());
            }
        )*}; }
        check!(
            ResizeEdges,
            AdjacentEdges,
            GapMask,
            AssignmentKind,
            TilingDrag,
            KeyboardGroups
        );
        assert_eq!((ResizeEdges::LEFT | ResizeEdges::TOP).bits(), 5);
        assert_eq!(GapMask::OUTER.bits(), 30);
        assert_eq!(AdjacentEdges::BOTTOM.bits(), 16);
    }
    #[test]
    fn zero_wildcards_and_group_priority_preserve_legacy_policy() {
        assert!(AssignmentKind::empty().accepts(AssignmentKind::empty()));
        assert!(!AssignmentKind::empty().accepts(AssignmentKind::all()));
        assert!(AssignmentKind::from_bits_retain(0x8000_0000)
            .accepts(AssignmentKind::from_bits_retain(0x8000_0000)));
        assert!(KeyboardGroups::empty().matches_group(99));
        assert!(!KeyboardGroups::GROUP1.matches_group(4));
        assert!((KeyboardGroups::GROUP1 | KeyboardGroups::GROUP3).matches_group(2));
        assert_eq!(
            (KeyboardGroups::GROUP2 | KeyboardGroups::GROUP3).translation_group(),
            1
        );
        assert_eq!(
            KeyboardGroups::from_state_mask(0x50008).state_mask(),
            0x50000
        );
    }
}
