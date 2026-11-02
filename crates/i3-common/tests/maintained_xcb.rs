//! Real maintained-XCB ownership and protocol checks on an owned private display.
use std::ffi::{CStr, CString};
use xcb::{x, Xid};

fn atom(connection: &xcb::Connection, name: &CStr) -> x::Atom {
    let cookie = connection.send_request(&x::InternAtom {
        only_if_exists: false,
        name: name.to_bytes(),
    });
    connection.wait_for_reply(cookie).unwrap().atom()
}
fn sync(connection: &xcb::Connection) {
    let cookie = connection.send_request(&x::GetInputFocus {});
    connection.wait_for_reply(cookie).unwrap();
}
#[test]
fn maintained_xcb_requests_and_borrowed_lifetime() {
    if std::env::var_os("I333_MAINTAINED_XCB_PROBE").is_none() {
        let _guard = i3_test_support::suite_lock();
        i3_test_support::run_python(
            "test-maintained-xcb.py",
            &[
                "--probe".into(),
                std::env::current_exe().unwrap().into_os_string(),
            ],
        );
        return;
    }
    let (connection, screen_number) = xcb::Connection::connect(None).unwrap();
    let screen = connection
        .get_setup()
        .roots()
        .nth(screen_number as usize)
        .unwrap();
    let root = screen.root();
    let raw = connection
        .get_raw_conn()
        .cast::<xcb_sys::xcb_connection_t>();
    {
        // SAFETY: this view is dropped before its sole maintained owner, and no
        // callback can disconnect the connection during this synchronous probe.
        let view = unsafe { i3_common::xcb_connection::borrowed(raw) }.unwrap();
        assert_eq!(view.get_raw_conn(), connection.get_raw_conn());
        sync(&view);
    }
    sync(&connection); // Dropping the borrowed view must not disconnect its owner.
    let long_property = vec![b'x'; 2057];
    for (name, kind, data) in [
        (c"I333_XCB_BYTES", x::ATOM_STRING, b"\xff\0tail".as_slice()),
        (c"I333_XCB_LONG", x::ATOM_STRING, long_property.as_slice()),
    ] {
        let property = atom(&connection, name);
        connection.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: root,
            property,
            r#type: kind,
            data,
        });
        sync(&connection);
        // SAFETY: the native raw pointer is borrowed from this live owner.
        let result = unsafe {
            i3_common::root_atom_contents::root_atom_contents(name.as_ptr(), raw, screen_number)
        };
        assert!(!result.is_null());
        let expected = data.split(|&byte| byte == 0).next().unwrap();
        assert_eq!(unsafe { CStr::from_ptr(result) }.to_bytes(), expected);
        unsafe {
            libc::free(result.cast());
        }
        // The public API independently owns its temporary connection too.
        assert_eq!(
            i3_common::root_atom_contents::read_root_property(name)
                .unwrap()
                .to_bytes(),
            expected
        );
    }
    let name = c"I333_XCB_CARDINAL";
    connection.send_request(&x::ChangeProperty {
        mode: x::PropMode::Replace,
        window: root,
        property: atom(&connection, name),
        r#type: x::ATOM_CARDINAL,
        data: &[u32::MAX],
    });
    sync(&connection);
    assert_eq!(
        i3_common::root_atom_contents::read_root_property(name),
        Some(CString::new(u32::MAX.to_string()).unwrap())
    );
    connection.send_request(&x::ChangeWindowAttributes {
        window: root,
        value_list: &[x::Cw::BackPixel(0x123456)],
    });
    connection.send_request(&x::ClearArea {
        exposures: false,
        window: root,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    });
    sync(&connection);
    // The remaining raw screen adapter is exclusively for existing native
    // callers; the maintained requests own all replies used by these helpers.
    let raw_screen = unsafe { xcb_sys::xcb_aux_get_screen(raw, screen_number) };
    assert!(!raw_screen.is_null());
    assert!(unsafe { i3_common::is_background_set::is_background_set(raw, raw_screen) });
    let image = || {
        let request = connection.send_request(&x::GetImage {
            format: x::ImageFormat::ZPixmap,
            drawable: x::Drawable::Window(root),
            x: 100,
            y: 100,
            width: 1,
            height: 1,
            plane_mask: u32::MAX,
        });
        connection.wait_for_reply(request).unwrap().data().to_vec()
    };
    let before = image();
    unsafe {
        i3_common::screenshot_wallpaper::set_screenshot_as_wallpaper(raw, raw_screen);
    }
    connection.send_request(&x::ClearArea {
        exposures: false,
        window: root,
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    });
    sync(&connection);
    assert_eq!(image(), before);
    assert!(unsafe { i3_common::is_background_set::is_background_set(raw, raw_screen) });
    sync(&connection);
    connection.has_error().unwrap();
    assert_ne!(root.resource_id(), 0);
}
