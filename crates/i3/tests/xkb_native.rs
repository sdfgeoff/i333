use std::ffi::CStr;
use xkbcommon::xkb::ffi::*;

#[test]
fn maintained_bindings_create_context_and_resolve_keysym() {
    // No X server, keyboard grabs, input events, or desktop configuration involved.
    unsafe {
        let context = xkb_context_new(XKB_CONTEXT_NO_ENVIRONMENT_NAMES);
        assert!(!context.is_null());
        let symbol = xkb_keysym_from_name(c"Return".as_ptr(), XKB_KEYSYM_NO_FLAGS);
        assert_eq!(symbol, 0xff0d);
        let mut name = [0; 32];
        assert_eq!(
            xkb_keysym_get_name(symbol, name.as_mut_ptr(), name.len()),
            6
        );
        assert_eq!(CStr::from_ptr(name.as_ptr()).to_bytes(), b"Return");
        xkb_context_unref(context);
    }
}
