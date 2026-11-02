//! Owned setup views and the actual Cairo native visual boundary.
use std::rc::Rc;
use xcb::{x, Xid};

/// Semantic screen properties copied from the maintained setup view.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScreenInfo {
    pub root: u32,
    pub default_colormap: u32,
    pub black_pixel: u32,
    pub white_pixel: u32,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_millimeters: u16,
    pub height_in_millimeters: u16,
    pub root_visual: u32,
    pub root_depth: u8,
}
impl From<&x::Screen> for ScreenInfo {
    fn from(screen: &x::Screen) -> Self {
        Self {
            root: screen.root().resource_id(),
            default_colormap: screen.default_colormap().resource_id(),
            black_pixel: screen.black_pixel(),
            white_pixel: screen.white_pixel(),
            width_in_pixels: screen.width_in_pixels(),
            height_in_pixels: screen.height_in_pixels(),
            width_in_millimeters: screen.width_in_millimeters(),
            height_in_millimeters: screen.height_in_millimeters(),
            root_visual: screen.root_visual(),
            root_depth: screen.root_depth(),
        }
    }
}

/// A selected setup screen whose connection owner outlives every borrowed view.
#[derive(Clone)]
pub struct DisplayScreen {
    connection: Rc<xcb::Connection>,
    index: usize,
}

impl DisplayScreen {
    pub fn new(connection: Rc<xcb::Connection>, index: i32) -> Option<Self> {
        let index = usize::try_from(index).ok()?;
        connection.get_setup().roots().nth(index)?;
        Some(Self { connection, index })
    }

    pub fn screen(&self) -> &x::Screen {
        // Setup data is immutable for the entire retained connection lifetime.
        self.connection.get_setup().roots().nth(self.index).unwrap()
    }

    pub fn info(&self) -> ScreenInfo {
        ScreenInfo::from(self.screen())
    }

    pub fn connection(&self) -> &xcb::Connection {
        &self.connection
    }

    pub fn visual(&self, id: u32) -> Option<CairoVisual> {
        self.screen().allowed_depths().find_map(|depth| {
            depth
                .visuals()
                .iter()
                .find(|visual| visual.visual_id() == id)
                .map(CairoVisual::new)
        })
    }
}

/// An aligned, independently owned native visual for Cairo's XCB backend.
///
/// Cairo retains this pointer through surface destruction. Keep this owner alive
/// until every associated Cairo surface has been destroyed. The maintained
/// Visualtype is an alignment-one wire view and must never itself be cast to a
/// native visual pointer. This owner copies its complete initialized payload into
/// an aligned allocation without reading its closed class enum.
pub struct CairoVisual {
    words: Box<[u32; 6]>,
}

impl CairoVisual {
    pub fn new(visual: &x::Visualtype) -> Self {
        // xcb1.7.1's concrete Visualtype consists solely of its initialized
        // [u8;24] payload. Pin that layout assumption at this binding boundary.
        const { assert!(size_of::<x::Visualtype>() == 24) };
        let mut words = Box::new([0u32; 6]);
        // SAFETY: all 24 bytes of the concrete maintained visual are initialized;
        // the destination is a separate 24-byte allocation aligned for native
        // u32 fields. Byte copying neither reads nor constructs a closed enum.
        unsafe {
            std::ptr::copy_nonoverlapping(
                std::ptr::from_ref(visual).cast::<u8>(),
                words.as_mut_ptr().cast::<u8>(),
                24,
            );
        }
        Self { words }
    }

    pub fn visual_id(&self) -> u32 {
        self.words[0]
    }

    /// Borrow Cairo's canonical opaque native visual pointer. The allocation
    /// must remain owned until all surfaces using this pointer are destroyed.
    pub fn as_ptr(&self) -> *mut cairo_sys::xcb_visualtype_t {
        self.words.as_ptr().cast_mut().cast()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cairo_visual_is_aligned_owned_and_preserves_every_class_byte() {
        for class in 0..=u8::MAX {
            let mut visual = x::Visualtype::new(
                0x12345678,
                x::VisualClass::TrueColor,
                8,
                256,
                0xff0000,
                0xff00,
                0xff,
            );
            // Independent foreign-input fixture; never call the closed getter.
            unsafe {
                std::ptr::from_mut(&mut visual)
                    .cast::<u8>()
                    .add(4)
                    .write(class)
            };
            let owned = CairoVisual::new(&visual);
            assert_eq!(owned.as_ptr() as usize % align_of::<u32>(), 0);
            // Alter the source after copying to verify independent ownership.
            unsafe { std::ptr::from_mut(&mut visual).cast::<u8>().write(0) };
            let bytes = unsafe { std::slice::from_raw_parts(owned.as_ptr().cast::<u8>(), 24) };
            let mut expected = [0u8; 24];
            expected[..4].copy_from_slice(&0x12345678u32.to_ne_bytes());
            expected[4] = class;
            expected[5] = 8;
            expected[6..8].copy_from_slice(&256u16.to_ne_bytes());
            expected[8..12].copy_from_slice(&0xff0000u32.to_ne_bytes());
            expected[12..16].copy_from_slice(&0xff00u32.to_ne_bytes());
            expected[16..20].copy_from_slice(&0xffu32.to_ne_bytes());
            assert_eq!(bytes, expected);
        }
    }
}
