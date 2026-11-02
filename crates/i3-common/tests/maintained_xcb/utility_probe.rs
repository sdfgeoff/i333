//! Independent native utility comparisons and real additional-button delivery.
use std::{
    ffi::CStr,
    rc::Rc,
    time::{Duration, Instant},
};
use xcb::{x, Xid};
use xcb_interop::{cursor::CursorContext, keysyms::KeySymbols, transport, xrm::Database};

pub fn verify(connection: Rc<xcb::Connection>, screen: i32, root: x::Window) {
    let mut owned = KeySymbols::new(connection.clone()).unwrap();
    let reference = unsafe { xcb_sys::xcb_key_symbols_alloc(connection.get_raw_conn().cast()) };
    assert!(!reference.is_null());
    for keycode in 0..=u8::MAX {
        for column in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, 7, 255, 256, i32::MAX] {
            assert_eq!(owned.keysym(keycode, column), unsafe {
                xcb_sys::xcb_key_symbols_get_keysym(reference, keycode, column)
            });
        }
    }
    let reply = connection
        .wait_for_reply(connection.send_request(&x::GetModifierMapping {}))
        .unwrap();
    for keysym in [0, 0xffe1, 0xffe3, 0xff7f, 0xffeb, u32::MAX] {
        let native = unsafe { xcb_sys::xcb_key_symbols_get_keycode(reference, keysym) };
        let expected = if native.is_null() {
            Vec::new()
        } else {
            let bytes = unsafe { CStr::from_ptr(native.cast()).to_bytes().to_vec() };
            unsafe { libc::free(native.cast()) };
            bytes
        };
        assert_eq!(owned.keycodes(keysym), expected);
        let mask = unsafe { i3_common::get_mod_mask::get_mod_mask_for(keysym, reference, &reply) };
        assert_eq!(
            i3_common::get_mod_mask::modifier_mask_for(keysym, &owned, &reply),
            mask
        );
        assert_eq!(
            i3_common::get_mod_mask::query_modifier_mask(keysym, &owned),
            mask
        );
    }
    let mapping = x::MappingNotifyEvent::new(
        x::Mapping::Keyboard,
        connection.get_setup().min_keycode(),
        1,
    );
    owned.refresh(&mapping);
    assert_eq!(
        owned.keysym(connection.get_setup().min_keycode(), 0),
        unsafe {
            xcb_sys::xcb_key_symbols_get_keysym(reference, connection.get_setup().min_keycode(), 0)
        }
    );
    unsafe { xcb_sys::xcb_key_symbols_free(reference) };
    drop(owned);

    assert!(CursorContext::new(connection.clone(), -1).is_err());
    assert!(CursorContext::new(connection.clone(), i32::MAX).is_err());
    let cursors = CursorContext::new(connection.clone(), screen).unwrap();
    let cursor = cursors.load(c"left_ptr");
    assert_ne!(cursor.resource_id(), 0);
    connection
        .send_and_check_request(&x::FreeCursor { cursor })
        .unwrap();
    drop(cursors);

    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: root,
        property: x::ATOM_RESOURCE_MANAGER,
        r#type: x::ATOM_STRING,
        data: b"Xft.dpi: 144\nI333.label: \xffowned\n",
    });
    crate::sync(&connection);
    let resources = Database::from_connection(connection.clone()).unwrap();
    let dpi = resources.resource(c"Xft.dpi", None).unwrap();
    let label = resources.resource(c"I333.label", None).unwrap();
    drop(resources);
    assert_eq!(dpi, c"144");
    assert_eq!(label.to_bytes(), b"\xffowned");

    let pending = connection.send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(root),
    });
    transport::discard_reply(&connection, pending);
    crate::sync(&connection);
    verify_additional_buttons(&connection, root);
    verify_open_stack_mode(&connection, root);
}

fn verify_additional_buttons(connection: &xcb::Connection, root: x::Window) {
    use x11rb::protocol::{xproto, xtest::ConnectionExt};
    let (injector, _) = x11rb::connect(None).unwrap();
    for button in [1, 6, 7] {
        while connection.poll_for_event().unwrap().is_some() {}
        transport::grab_button(
            connection,
            false,
            root,
            x::EventMask::BUTTON_PRESS,
            x::GrabMode::Sync,
            x::GrabMode::Async,
            root,
            xcb::Xid::none(),
            button,
            x::ModMask::ANY,
        );
        crate::sync(connection);
        injector
            .xtest_fake_input(
                xproto::BUTTON_PRESS_EVENT,
                button,
                0,
                root.resource_id(),
                0,
                0,
                0,
            )
            .unwrap()
            .check()
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            if let Some(xcb::Event::X(x::Event::ButtonPress(event))) =
                connection.poll_for_event().unwrap()
            {
                assert_eq!(event.detail(), button);
                assert_eq!(event.event(), root);
                break;
            }
            assert!(
                Instant::now() < deadline,
                "button {button} was not delivered to its passive grab"
            );
            std::thread::sleep(Duration::from_millis(2));
        }
        connection.send_request(&x::AllowEvents {
            mode: x::Allow::AsyncPointer,
            time: x::CURRENT_TIME,
        });
        connection.flush().unwrap();
        injector
            .xtest_fake_input(
                xproto::BUTTON_RELEASE_EVENT,
                button,
                0,
                root.resource_id(),
                0,
                0,
                0,
            )
            .unwrap()
            .check()
            .unwrap();
        connection.send_request(&x::UngrabButton {
            button: x::ButtonIndex::Any,
            grab_window: root,
            modifiers: x::ModMask::ANY,
        });
        crate::sync(connection);
    }
}

fn verify_open_stack_mode(connection: &xcb::Connection, root: x::Window) {
    let window: x::Window = connection.generate_id();
    connection.send_request(&x::CreateWindow {
        depth: 0,
        wid: window,
        parent: root,
        x: 0,
        y: 0,
        width: 1,
        height: 1,
        border_width: 0,
        class: x::WindowClass::InputOutput,
        visual: 0,
        value_list: &[],
    });
    let cookie = transport::configure_window_with_open_stack_mode(connection, window, 0x40, &[99]);
    crate::sync(connection);
    let mut found = false;
    loop {
        match connection.poll_for_event() {
            Err(xcb::Error::Protocol(error)) => {
                let header = unsafe { &*error.as_raw() };
                assert_eq!(header.error_code, 2); // native BadValue
                assert_eq!(header.major_code, 12); // native ConfigureWindow
                assert_eq!(header.sequence, cookie as u16);
                assert_eq!(header.full_sequence, cookie);
                found = true;
            }
            Ok(Some(_)) => {}
            Ok(None) => break,
            Err(error) => panic!("unexpected connection error: {error}"),
        }
    }
    assert!(
        found,
        "unknown stacking mode must retain its native asynchronous error"
    );
    connection.send_request(&x::DestroyWindow { window });
    crate::sync(connection);
}
