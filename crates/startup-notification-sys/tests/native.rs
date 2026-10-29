use startup_notification_sys::{sn_free, sn_malloc0, sn_realloc};

#[test]
fn native_allocator_preserves_bytes_when_resized() {
    // Exercise the actual library without opening a display or starting applications.
    unsafe {
        let memory = sn_malloc0(32).cast::<u8>();
        assert!(!memory.is_null());
        assert_eq!(std::slice::from_raw_parts(memory, 32), &[0; 32]);
        std::ptr::copy_nonoverlapping(b"startup".as_ptr(), memory, 7);
        let resized = sn_realloc(memory.cast(), 64).cast::<u8>();
        assert!(!resized.is_null());
        assert_eq!(std::slice::from_raw_parts(resized, 7), b"startup");
        sn_free(resized.cast());
    }
}
