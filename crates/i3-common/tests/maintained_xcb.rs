//! Real maintained-XCB ownership and protocol checks on an owned private display.
use i3_common::display::DisplayContext;
use maintained_xcb::native_utilities as native;
use std::ffi::{CStr, CString};
use xcb::{x, Xid};

mod maintained_xcb {
    pub mod native_utilities;
    pub mod utility_probe;
}

thread_local! {
    static DISPLAY_CONTEXT: std::cell::Cell<DisplayContext> = std::cell::Cell::new(DisplayContext::default());
}
fn display_context() -> DisplayContext {
    DISPLAY_CONTEXT.with(std::cell::Cell::get)
}

// C atexit runs after the main thread's Rust TLS destructors. The registration
// guard must be the owner, and its nonowning lookup must remain available then.
static TLS_DESTRUCTED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
struct ExitMarker;
impl Drop for ExitMarker {
    fn drop(&mut self) {
        TLS_DESTRUCTED.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
thread_local! { static EXIT_MARKER: ExitMarker = const { ExitMarker }; }
static mut EXIT_REGISTRATION: Option<i3_common::display::DisplayConnection> = None;
extern "C" fn release_display_at_exit() {
    assert!(TLS_DESTRUCTED.load(std::sync::atomic::Ordering::SeqCst));
    let connection = i3_common::display::connection().expect("registry survives TLS destruction");
    sync(&connection);
    let root_visual = connection.get_setup().roots().next().unwrap().root_visual();
    assert!(i3_common::display::visual(root_visual).is_some());
    drop(connection);
    // SAFETY: only this single-threaded child owns/accesses its atexit guard.
    let registration = unsafe { std::ptr::replace(&raw mut EXIT_REGISTRATION, None) };
    drop(registration);
    assert!(i3_common::display::connection().is_none());
}
fn atexit_probe() -> ! {
    let (connection, _) = xcb::Connection::connect(None).unwrap();
    let connection = std::rc::Rc::new(connection);
    let registration = i3_common::display::install_connection(connection.clone());
    unsafe { EXIT_REGISTRATION = Some(registration) };
    drop(connection); // The guard is the only strong owner before process::exit.
    EXIT_MARKER.with(|_| {});
    assert_eq!(unsafe { libc::atexit(release_display_at_exit) }, 0);
    std::process::exit(0);
}

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
fn verify_core_font(connection: &xcb::Connection, root: x::Window, gc: u32) {
    use i3_common::font;
    // All globals are event-thread borrows of this private probe's owned display.
    unsafe {
        let mut selected = font::load_font(c"fixed", true);
        let id = selected.xcb_details().unwrap().0;
        let color = |pixel| i3_common::draw_util::color_t {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
            colorpixel: pixel,
        };
        for sample in [String::new(), "abcdef".to_owned(), "x".repeat(260)] {
            let text = i3_common::string::I3String::new(&sample);
            let glyphs: Vec<x::Char2b> = text
                .rendering_glyphs()
                .iter()
                .map(|glyph| x::Char2b {
                    byte1: glyph.byte1,
                    byte2: glyph.byte2,
                })
                .collect();
            let expected = connection
                .wait_for_reply(connection.send_request(&x::QueryTextExtents {
                    font: x::Fontable::Font(id),
                    string: &glyphs,
                }))
                .unwrap()
                .overall_width();
            assert_eq!(font::predict_text_width(&selected, &text), expected);
            font::draw_text(
                &selected,
                &text,
                root.resource_id(),
                gc,
                std::ptr::null_mut(),
                color(0),
                color(1),
                0,
                10,
                10000,
            );
        }
        sync(connection);
        // A legal constant-width/no-table reply must use the query slow path.
        // Retain the actual server font resource and original owned reply.
        let info = selected.xcb_details().unwrap().1;
        let raw = libc::calloc(1, 60).cast::<u8>();
        assert!(!raw.is_null());
        std::ptr::copy_nonoverlapping(xcb::Reply::as_raw(info), raw, 60);
        raw.add(4).cast::<u32>().write_unaligned(7);
        raw.add(46).cast::<u16>().write_unaligned(0);
        raw.add(56).cast::<u32>().write_unaligned(0);
        let slow: x::QueryFontReply = xcb::Reply::from_raw(raw);
        let original = selected.replace_xcb_metrics(slow).unwrap();
        let text = i3_common::string::I3String::new("slow path");
        let expected = connection
            .wait_for_reply(
                connection.send_request(&x::QueryTextExtents {
                    font: x::Fontable::Font(id),
                    string: &text
                        .rendering_glyphs()
                        .iter()
                        .map(|glyph| x::Char2b {
                            byte1: glyph.byte1,
                            byte2: glyph.byte2,
                        })
                        .collect::<Vec<_>>(),
                }),
            )
            .unwrap()
            .overall_width();
        assert_eq!(font::predict_text_width(&selected, &text), expected);
        drop(selected.replace_xcb_metrics(original).unwrap());
        // Moving the owning font keeps the server resource alive; dropping its
        // final owner closes the XID exactly once even while the GC references it.
        let mut moved = vec![selected];
        let selected = moved.pop().unwrap();
        assert_eq!(font::predict_text_width(&selected, &text), expected);
        drop(selected);
        assert!(connection
            .wait_for_reply(connection.send_request(&x::QueryFont {
                font: x::Fontable::Font(id),
            }))
            .is_err());
        let fallback = font::load_font(c"i333-deliberately-nonexistent-font", true);
        assert_eq!(fallback.pattern(), Some(c"fixed"));
        assert_eq!(font::predict_text_width(&fallback, &text), expected);
        drop(fallback);
        sync(connection);
    }
}

#[test]
fn maintained_xcb_requests_and_borrowed_lifetime() {
    if std::env::var_os("I333_DISPLAY_ATEXIT_PROBE").is_some() {
        atexit_probe();
    }
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
    let exit_status = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "maintained_xcb_requests_and_borrowed_lifetime",
            "--nocapture",
        ])
        .env("I333_DISPLAY_ATEXIT_PROBE", "1")
        .status()
        .unwrap();
    assert!(
        exit_status.success(),
        "C atexit/TLS ownership probe failed: {exit_status}"
    );
    let (connection, screen_number) = xcb::Connection::connect_with_extensions(
        None,
        &[],
        &[
            xcb::Extension::RandR,
            xcb::Extension::Xinerama,
            xcb::Extension::Xkb,
            xcb::Extension::Shape,
        ],
    )
    .unwrap();
    let connection = std::rc::Rc::new(connection);
    assert!(i3_common::display::connection().is_none());
    let registration = i3_common::display::install_connection(connection.clone());
    assert!(std::rc::Rc::ptr_eq(
        &connection,
        &i3_common::display::connection().unwrap()
    ));
    assert!(
        std::thread::spawn(|| i3_common::display::connection().is_none())
            .join()
            .unwrap()
    );
    let screen = connection
        .get_setup()
        .roots()
        .nth(screen_number as usize)
        .unwrap();
    let root = screen.root();
    let raw = connection.get_raw_conn();
    {
        // SAFETY: this view is dropped before its sole maintained owner, and no
        // callback can disconnect the connection during this synchronous probe.
        let view = unsafe { i3_common::xcb_connection::borrowed(raw.cast()) }.unwrap();
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
            i3_common::root_atom_contents::root_atom_contents(
                name.as_ptr(),
                raw.cast(),
                screen_number,
            )
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
    let raw_screen = unsafe { native::xcb_aux_get_screen(raw, screen_number) };
    assert!(!raw_screen.is_null());
    let owned_screen =
        i3_common::display::DisplayScreen::new(connection.clone(), screen_number).unwrap();
    assert!(i3_common::display::DisplayScreen::new(connection.clone(), -1).is_none());
    assert!(i3_common::display::DisplayScreen::new(connection.clone(), i32::MAX).is_none());
    assert_eq!(owned_screen.screen().root(), root);
    let owned_visual = owned_screen.visual(screen.root_visual()).unwrap();
    let cached_visual = i3_common::display::visual(screen.root_visual()).unwrap();
    assert!(std::rc::Rc::ptr_eq(
        &cached_visual,
        &i3_common::display::visual(screen.root_visual()).unwrap()
    ));
    let cached_visual_weak = std::rc::Rc::downgrade(&cached_visual);
    unsafe {
        let native = native::xcb_aux_find_visual_by_id(raw_screen, screen.root_visual());
        assert!(!native.is_null());
        let native_bytes = std::slice::from_raw_parts(native.cast::<u8>(), 24);
        assert_eq!(
            std::slice::from_raw_parts(owned_visual.as_ptr().cast::<u8>(), 24),
            native_bytes
        );
        assert_eq!(
            std::slice::from_raw_parts(cached_visual.as_ptr().cast::<u8>(), 24),
            native_bytes
        );
        // Exercise Cairo's real native visual consumer, retaining each visual
        // until its surface is destroyed, including unsupported 8-bit visuals.
        let original = cairo_sys::cairo_xcb_surface_create(
            raw.cast(),
            root.resource_id(),
            native.cast(),
            2,
            2,
        );
        let copied = cairo_sys::cairo_xcb_surface_create(
            raw.cast(),
            root.resource_id(),
            owned_visual.as_ptr(),
            2,
            2,
        );
        assert_eq!(
            cairo_sys::cairo_surface_status(original),
            cairo_sys::cairo_surface_status(copied)
        );
        cairo_sys::cairo_surface_destroy(original);
        cairo_sys::cairo_surface_destroy(copied);
    }
    drop(owned_visual);
    drop(cached_visual);
    assert!(
        cached_visual_weak.upgrade().is_some(),
        "guard retains the Cairo visual cache"
    );
    DISPLAY_CONTEXT.with(|context| context.set(DisplayContext::from_screen(&owned_screen, None)));
    i3_common::display::set_provider(display_context);
    for depth in screen.allowed_depths() {
        for visual in depth.visuals() {
            assert_eq!(
                unsafe { i3_common::draw_util::get_visual_depth(visual.visual_id()) },
                u16::from(depth.depth())
            );
        }
    }
    // The native utility oracle supplies Cairo's own opaque native visual;
    // protocol traversal and header schemas are not duplicated in the fixture.
    let native_visual =
        unsafe { native::xcb_aux_find_visual_by_id(raw_screen, screen.root_visual()) };
    assert!(!native_visual.is_null());
    let mut surface: i3_common::draw_util::surface_t = unsafe { std::mem::zeroed() };
    unsafe {
        i3_common::draw_util::draw_util_surface_init(
            raw.cast(),
            &mut surface,
            root.resource_id(),
            Some(&i3_common::display::visual(screen.root_visual()).unwrap()),
            160,
            120,
        );
        assert_ne!(surface.gc, 0);
        // Some 8-bit Xvfb visuals are unsupported by Cairo. Compare against
        // its direct native constructor rather than assuming TrueColor support.
        let native_surface = cairo_sys::cairo_xcb_surface_create(
            raw.cast(),
            root.resource_id(),
            native_visual.cast(),
            160,
            120,
        );
        let native_context = cairo_sys::cairo_create(native_surface);
        assert_eq!(
            cairo_sys::cairo_status(surface.cr),
            cairo_sys::cairo_status(native_context)
        );
        cairo_sys::cairo_destroy(native_context);
        cairo_sys::cairo_surface_destroy(native_surface);
        connection
            .send_and_check_request(&x::ChangeGc {
                gc: xcb::XidNew::new(surface.gc),
                value_list: &[x::Gc::Foreground(0)],
            })
            .unwrap();
        i3_common::draw_util::draw_util_surface_free(raw.cast(), &mut surface);
    }
    verify_core_font(&connection, root, surface.gc);
    maintained_xcb::utility_probe::verify(connection.clone(), screen_number, root);
    assert_eq!(
        unsafe { i3_common::draw_util::get_visual_depth(u32::MAX) },
        0
    );
    if screen.root_depth() == 8 {
        let cookie = connection.send_request(&x::AllocColor {
            cmap: screen.default_colormap(),
            red: 0x1212,
            green: 0x3434,
            blue: 0x5656,
        });
        let expected = connection.wait_for_reply(cookie).unwrap().pixel();
        let actual = unsafe { i3_common::get_colorpixel::get_colorpixel(c"#123456".as_ptr()) };
        assert_eq!(actual, expected);
        assert_eq!(
            unsafe { i3_common::get_colorpixel::get_colorpixel(c"#123456".as_ptr()) },
            expected
        );
    }
    let mapping = connection
        .wait_for_reply(connection.send_request(&x::GetModifierMapping {}))
        .unwrap();
    let symbols = unsafe { native::xcb_key_symbols_alloc(raw) };
    assert!(!symbols.is_null());
    for keysym in [0xffe1, 0xffe3, 0xffe9, 0xff7f, 0xffeb, u32::MAX] {
        // Compute the expected modifier with the native utility's keycode list
        // and the real server mapping, independently of production traversal.
        let codes = unsafe { native::xcb_key_symbols_get_keycode(symbols, keysym) };
        let expected = if codes.is_null() {
            0
        } else {
            let codes_slice = unsafe { CStr::from_ptr(codes.cast()) }.to_bytes();
            let per = usize::from(mapping.keycodes_per_modifier());
            let result = if per == 0 {
                0
            } else {
                mapping
                    .keycodes()
                    .chunks_exact(per)
                    .enumerate()
                    .find_map(|(index, values)| {
                        values
                            .iter()
                            .any(|code| *code != 0 && codes_slice.contains(code))
                            .then_some(1 << index)
                    })
                    .unwrap_or(0)
            };
            unsafe { libc::free(codes.cast()) };
            result
        };
        assert_eq!(
            unsafe { i3_common::get_mod_mask::get_mod_mask_for(keysym, symbols, &mapping) },
            expected
        );
        assert_eq!(
            unsafe { i3_common::get_mod_mask::aio_get_mod_mask_for(keysym, symbols) },
            expected
        );
    }
    unsafe { native::xcb_key_symbols_free(symbols) };

    assert!(i3_common::is_background_set::is_background_set(
        &connection,
        owned_screen.info()
    ));
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
    i3_common::screenshot_wallpaper::set_screenshot_as_wallpaper(&connection, owned_screen.info());
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
    assert!(i3_common::is_background_set::is_background_set(
        &connection,
        owned_screen.info()
    ));
    sync(&connection);
    connection.has_error().unwrap();
    assert_ne!(root.resource_id(), 0);
    let retained = i3_common::display::connection().unwrap();
    drop(registration);
    assert!(
        cached_visual_weak.upgrade().is_none(),
        "cache is released after graphics teardown"
    );
    assert!(i3_common::display::connection().is_none());
    sync(&retained);
    drop(retained);
    sync(&connection);
    i3_common::display::set_provider(DisplayContext::default);
    DISPLAY_CONTEXT.with(|context| context.set(DisplayContext::default()));
}
