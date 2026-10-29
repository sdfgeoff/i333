use xkbcommon::xkb;

#[test]
fn maintained_bindings_create_context_and_resolve_keysym() {
    // Owned maintained bindings release the context even if an assertion fails.
    // No X server, keyboard grabs, input events, or desktop configuration.
    let context = xkb::Context::new(xkb::CONTEXT_NO_ENVIRONMENT_NAMES);
    assert!(!context.get_raw_ptr().is_null());
    let symbol = xkb::keysym_from_name("Return", xkb::KEYSYM_NO_FLAGS);
    assert_eq!(symbol.raw(), 0xff0d);
    assert_eq!(xkb::keysym_get_name(symbol), "Return");
}
