//! Offscreen ABI probes for the maintained gtk-rs sys crates.
//! No X display, i3 process, or input device is used.

#[cfg(test)]
mod cairo_sys {
    use cairo_sys::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    struct Surface(*mut cairo_surface_t);
    impl Drop for Surface {
        fn drop(&mut self) {
            unsafe {
                cairo_surface_destroy(self.0);
            }
        }
    }
    struct Context(*mut cairo_t);
    impl Drop for Context {
        fn drop(&mut self) {
            unsafe {
                cairo_destroy(self.0);
            }
        }
    }
    fn image() -> Surface {
        // SAFETY: image creation needs no X connection; these dimensions and
        // format describe a small valid native buffer owned by Surface.
        let surface = Surface(unsafe { cairo_image_surface_create(FORMAT_A_RGB32, 2, 2) });
        assert!(!surface.0.is_null());
        surface
    }
    #[test]
    fn image_context_paint_uses_the_native_abi_without_x() {
        let surface = image();
        // SAFETY: all handles remain owned and live through the pixel read.
        // Cairo's ARGB32 buffer is at least 2x2 pixels with the returned stride.
        unsafe {
            assert_eq!(cairo_image_surface_get_width(surface.0), 2);
            assert_eq!(cairo_image_surface_get_height(surface.0), 2);
            assert!(cairo_image_surface_get_stride(surface.0) >= 8);
            let context = Context(cairo_create(surface.0));
            assert!(!context.0.is_null());
            assert_eq!(cairo_status(context.0), STATUS_SUCCESS);
            cairo_set_operator(context.0, OPERATOR_SOURCE);
            cairo_set_source_rgba(context.0, 1.0, 0.0, 0.0, 1.0);
            cairo_paint(context.0);
            cairo_surface_flush(surface.0);
            let data = cairo_image_surface_get_data(surface.0);
            assert!(!data.is_null());
            assert_eq!(std::ptr::read_unaligned(data.cast::<u32>()), 0xffff0000);
            assert_eq!(cairo_status(context.0), STATUS_SUCCESS);
        }
    }
    #[test]
    fn region_rectangle_layout_and_subtraction_match_native_abi() {
        assert_eq!(std::mem::size_of::<cairo_rectangle_int_t>(), 16);
        assert_eq!(std::mem::align_of::<cairo_rectangle_int_t>(), 4);
        let mut rectangle = cairo_rectangle_int_t {
            x: 0,
            y: 0,
            width: 20,
            height: 20,
        };
        let mut remove = cairo_rectangle_int_t {
            x: 10,
            y: 0,
            width: 10,
            height: 20,
        };
        // SAFETY: both stack rectangles have the C layout asserted above and
        // are borrowed only during each synchronous call; region is destroyed.
        unsafe {
            let region = cairo_region_create_rectangle(&mut rectangle);
            assert!(!region.is_null());
            assert_eq!(
                cairo_region_subtract_rectangle(region, &mut remove),
                STATUS_SUCCESS
            );
            assert_eq!(cairo_region_num_rectangles(region), 1);
            let mut result = rectangle;
            cairo_region_get_rectangle(region, 0, &mut result);
            assert_eq!(
                (result.x, result.y, result.width, result.height),
                (0, 0, 10, 20)
            );
            cairo_region_destroy(region);
        }
    }
    unsafe extern "C" fn release_counter(data: *mut std::ffi::c_void) {
        // SAFETY: user_data_ownership_callback receives this exact Box once,
        // when its surface is destroyed by Cairo.
        let counter = unsafe { Box::from_raw(data.cast::<Arc<AtomicUsize>>()) };
        counter.fetch_add(1, Ordering::SeqCst);
    }
    #[test]
    fn user_data_ownership_callback_has_the_native_calling_convention() {
        assert_eq!(
            std::mem::size_of::<cairo_user_data_key_t>(),
            std::mem::size_of::<std::ffi::c_int>()
        );
        let counter = Arc::new(AtomicUsize::new(0));
        let key = cairo_user_data_key_t { unused: 0 };
        let surface = image();
        let data = Box::into_raw(Box::new(counter.clone()));
        // SAFETY: key stays at a stable stack address until the owned surface
        // drops. A successful registration transfers the Box to its callback.
        let status = unsafe {
            cairo_surface_set_user_data(surface.0, &key, data.cast(), Some(release_counter))
        };
        if status != STATUS_SUCCESS {
            unsafe {
                drop(Box::from_raw(data));
            }
        }
        assert_eq!(status, STATUS_SUCCESS);
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        drop(surface);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
mod pango_sys {
    use pango_sys::*;
    #[test]
    fn font_description_round_trip_uses_native_pango_without_x() {
        // SAFETY: Pango copies this terminated description. Its family pointer
        // stays borrowed until description_free releases the unique owner.
        unsafe {
            let description = pango_font_description_from_string(c"Sans Bold 12".as_ptr());
            assert!(!description.is_null());
            let family = pango_font_description_get_family(description);
            assert!(!family.is_null());
            assert_eq!(std::ffi::CStr::from_ptr(family).to_bytes(), b"Sans");
            assert_eq!(
                pango_font_description_get_size(description),
                12 * PANGO_SCALE
            );
            pango_font_description_free(description);
        }
        assert_eq!(std::mem::size_of::<PangoWrapMode>(), 4);
        assert_eq!(std::mem::size_of::<PangoEllipsizeMode>(), 4);
    }
}

#[cfg(test)]
mod gobject_sys {
    use gobject_sys::*;
    use std::ffi::c_char;
    #[test]
    fn native_object_reference_round_trip_needs_no_display() {
        // SAFETY: A plain GObject requires no properties or display connection.
        // Both references acquired here are released exactly once below.
        unsafe {
            let object = g_object_new(g_object_get_type(), core::ptr::null::<c_char>());
            assert!(!object.is_null());
            assert_eq!(g_object_ref(object), object);
            g_object_unref(object);
            g_object_unref(object);
        }
    }
}
