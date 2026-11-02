//! Application domains, including numeric values preserved by the layout format.
//!
//! These are internal Rust values, not foreign enum arguments. Open domains use
//! explicit C/integer tags with a valid zero variant, so Copy records initialized
//! to zero remain valid. Unknown payloads never cross native ABI boundaries.
use crate::flags::AdjacentEdges;

macro_rules! open_domain {
    ($name:ident: $repr:ident, $unknown:literal {
        $($(#[$attr:meta])* $variant:ident = $raw:expr,)*
    }) => {
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        #[repr(C, $repr)]
        pub enum $name {
            $($(#[$attr])* $variant = $raw,)*
            Unknown($repr) = $unknown,
        }

        impl $name {
            pub const fn from_raw(raw: $repr) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Unknown(other),
                }
            }

            pub const fn raw(self) -> $repr {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Unknown(raw) => raw,
                }
            }
        }

        impl From<$repr> for $name {
            fn from(raw: $repr) -> Self { Self::from_raw(raw) }
        }
        impl From<$name> for $repr {
            fn from(value: $name) -> Self { value.raw() }
        }
    };
}

open_domain!(ContainerType: u32, 6 {
    #[default] Root = 0,
    Output = 1,
    Con = 2,
    Floating = 3,
    Workspace = 4,
    DockArea = 5,
});
open_domain!(FullscreenMode: u32, 3 {
    #[default] None = 0,
    Output = 1,
    Global = 2,
});
open_domain!(DockMatch: i32, 4 {
    #[default] NoDock = 0,
    Any = 1,
    Top = 2,
    Bottom = 3,
    DontCheck = -1,
});
open_domain!(MatchInsertWhere: u32, 3 {
    #[default] Here = 0,
    AssignWorkspace = 1,
    Below = 2,
});

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(u32)]
pub enum HideEdgeBordersMode {
    #[default]
    None = 0,
    Vertical = 3,
    Horizontal = 20,
    Both = 23,
    Smart = 32,
    SmartNoGaps = 64,
}
impl HideEdgeBordersMode {
    pub const fn raw(self) -> u32 {
        self as u32
    }
    pub const fn edge_mask(self) -> AdjacentEdges {
        match self {
            Self::Vertical => AdjacentEdges::LEFT.union(AdjacentEdges::RIGHT),
            Self::Horizontal => AdjacentEdges::TOP.union(AdjacentEdges::BOTTOM),
            Self::Both => AdjacentEdges::LEFT
                .union(AdjacentEdges::RIGHT)
                .union(AdjacentEdges::TOP)
                .union(AdjacentEdges::BOTTOM),
            Self::None | Self::Smart | Self::SmartNoGaps => AdjacentEdges::empty(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_domain_numeric_extrema_roundtrip_and_known_values_are_canonical() {
        for raw in [
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            i32::MAX as u32,
            0x8000_0000,
            u32::MAX,
        ] {
            assert_eq!(ContainerType::from_raw(raw).raw(), raw);
            assert_eq!(FullscreenMode::from_raw(raw).raw(), raw);
            assert_eq!(MatchInsertWhere::from_raw(raw).raw(), raw);
        }
        for raw in [i32::MIN, -2, -1, 0, 1, 2, 3, 4, i32::MAX] {
            assert_eq!(DockMatch::from_raw(raw).raw(), raw);
        }
        assert_eq!(ContainerType::from_raw(2), ContainerType::Con);
        assert_eq!(FullscreenMode::from_raw(2), FullscreenMode::Global);
        assert_eq!(DockMatch::from_raw(-1), DockMatch::DontCheck);
        assert_eq!(
            MatchInsertWhere::from_raw(1),
            MatchInsertWhere::AssignWorkspace
        );
        assert_eq!(
            ContainerType::from_raw(u32::MAX),
            ContainerType::Unknown(u32::MAX)
        );
    }
    #[test]
    fn zero_storage_is_valid_and_internal_payload_layout_is_deliberate() {
        // Explicit repr(C,integer) fixes tag location and all zero tags select a
        // known payload-free variant. These types own no resources and are Copy.
        unsafe {
            assert_eq!(std::mem::zeroed::<ContainerType>(), ContainerType::Root);
            assert_eq!(std::mem::zeroed::<FullscreenMode>(), FullscreenMode::None);
            assert_eq!(std::mem::zeroed::<DockMatch>(), DockMatch::NoDock);
            assert_eq!(
                std::mem::zeroed::<MatchInsertWhere>(),
                MatchInsertWhere::Here
            );
            assert_eq!(
                std::mem::zeroed::<HideEdgeBordersMode>(),
                HideEdgeBordersMode::None
            );
            let criteria: crate::types::Match = std::mem::zeroed();
            assert_eq!(criteria.dock, DockMatch::NoDock);
            assert_eq!(criteria.insert_where, MatchInsertWhere::Here);
        }
        assert_eq!(std::mem::size_of::<ContainerType>(), 8);
        assert_eq!(std::mem::size_of::<DockMatch>(), 8);
        assert_eq!(std::mem::size_of::<HideEdgeBordersMode>(), 4);
        assert!(!std::mem::needs_drop::<crate::types::Match>());
    }
    #[test]
    fn hiding_modes_translate_only_actual_edges_and_keep_native_codes() {
        for (mode, raw, edges) in [
            (HideEdgeBordersMode::None, 0, 0),
            (HideEdgeBordersMode::Vertical, 3, 3),
            (HideEdgeBordersMode::Horizontal, 20, 20),
            (HideEdgeBordersMode::Both, 23, 23),
            (HideEdgeBordersMode::Smart, 32, 0),
            (HideEdgeBordersMode::SmartNoGaps, 64, 0),
        ] {
            assert_eq!(mode.raw(), raw);
            assert_eq!(mode.edge_mask().bits(), edges);
        }
    }
}
