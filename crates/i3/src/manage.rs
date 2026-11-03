use crate::con::arena::focus;
use crate::con::arena::parent::parent_ptr;
use crate::con::arena::{containers, resolve};
use crate::domains::{ContainerType, FullscreenMode, MatchInsertWhere};
use crate::flags::AssignmentKind;
use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::PopupDuringFullscreen;
use crate::types::WindowDock;
use i3_common::safewrappers::sasprintf;
use xcb::{x, Xid, XidNew};
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * manage.c: Initially managing new windows (or existing ones on restart).
 *
 */
// Mechanical file-by-file Rust translation of src/manage.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type Output = xoutput;
pub type i3Window = Window;

pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const NET_WM_DESKTOP_NONE: ::core::ffi::c_uint = 0xfffffff0 as ::core::ffi::c_uint;
pub const NET_WM_DESKTOP_ALL: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
unsafe fn _match_depth(win: *mut i3Window, con: *mut Con) -> u32 {
    let mut old_frame: u32 = 0 as u32;
    if (*con).depth as ::core::ffi::c_int != (*win).depth as ::core::ffi::c_int {
        old_frame = (*con).frame.id as u32;
        (*con).depth = (*win).depth;
        x_con_reframe(con);
    }
    old_frame
}
unsafe fn _remove_matches(con: *mut Con) {
    while !(*con).swallow_head.tqh_first.is_null() {
        let first: *mut Match = (*con).swallow_head.tqh_first as *mut Match;
        if !(*first).matches.tqe_next.is_null() {
            (*(*first).matches.tqe_next).matches.tqe_prev = (*first).matches.tqe_prev;
        } else {
            (*con).swallow_head.tqh_last = (*first).matches.tqe_prev;
        }
        *(*first).matches.tqe_prev = (*first).matches.tqe_next;
        drop(Box::from_raw(first));
    }
}

fn tree_child_count(children: u16, wire_words: u32) -> Option<usize> {
    let count = usize::from(children);
    (count <= wire_words as usize).then_some(count)
}

/// Root and connection are valid native XCB objects on the event thread.
pub unsafe fn manage_existing_windows(root_0: u32) {
    let cookie = connection().send_request(&x::QueryTree {
        window: x::Window::new(root_0),
    });
    let Some(reply) = connection().wait_for_reply(cookie).ok() else {
        return;
    };
    // Validate the wire count before the maintained reply creates its slice.
    let raw = xcb::Reply::as_raw(&reply);
    let count = raw.add(16).cast::<u16>().read_unaligned();
    let Some(_) = tree_child_count(count, reply.length()) else {
        return;
    };
    let children = reply.children().to_vec();
    // Retain batching before managing children can invoke graph callbacks.
    let cookies: Vec<_> = children
        .iter()
        .map(|&window| {
            AttributesCookie::Checked(connection().send_request(&x::GetWindowAttributes { window }))
        })
        .collect();
    drop(reply);
    for (child, cookie) in children.into_iter().zip(cookies) {
        manage_window(child.resource_id(), cookie, true);
    }
}

/// Native complete reply allocation, borrowed only for this copied first word.
/// The original accepted any format/type with at least one byte; reject only
/// undersized payloads that previously triggered an out-of-bounds u32 read.
fn first_property_word(reply: Option<&x::GetPropertyReply>) -> Option<u32> {
    let reply = reply?;
    // The original accepted nonstandard formats too. Only the first four
    // payload bytes are borrowed after checking both advertised lengths.
    let raw = unsafe { xcb::Reply::as_raw(reply) };
    let count = unsafe { raw.add(16).cast::<u32>().read_unaligned() };
    let bytes = usize::try_from(count)
        .ok()?
        .checked_mul(usize::from(reply.format() / 8))?;
    if bytes < 4 || reply.length() == 0 {
        return None;
    }
    Some(unsafe { raw.add(32).cast::<u32>().read_unaligned() })
}

fn connection() -> std::rc::Rc<xcb::Connection> {
    i3_common::display::connection().expect("main X11 connection is registered")
}
fn property_cookie(window: u32, atom: u32, kind: u32, long_length: u32) -> x::GetPropertyCookie {
    connection().send_request(&x::GetProperty {
        delete: false,
        window: x::Window::new(window),
        property: x::Atom::new(atom),
        r#type: x::Atom::new(kind),
        long_offset: 0,
        long_length,
    })
}
fn property_reply(cookie: x::GetPropertyCookie) -> Option<x::GetPropertyReply> {
    connection().wait_for_reply(cookie).ok()
}

pub(crate) enum AttributesCookie {
    Checked(x::GetWindowAttributesCookie),
    Unchecked(x::GetWindowAttributesCookieUnchecked),
}
fn attribute_reply(cookie: AttributesCookie) -> Option<x::GetWindowAttributesReply> {
    match cookie {
        AttributesCookie::Checked(cookie) => connection().wait_for_reply(cookie).ok(),
        AttributesCookie::Unchecked(cookie) => {
            connection().wait_for_reply_unchecked(cookie).ok().flatten()
        }
    }
}
fn attributes_viewable(reply: &x::GetWindowAttributesReply) -> bool {
    // The maintained map_state getter transmutes its native byte. Compare the
    // open wire value directly so unknown states retain the previous policy.
    unsafe { *xcb::Reply::as_raw(reply).add(26) == x::MapState::Viewable as u8 }
}

fn request_failed(cookie: xcb::VoidCookieChecked) -> bool {
    connection().check_request(cookie).is_err()
}

pub unsafe fn restore_geometry() {
    debuglog(
        c"%s:%s:%d - Restoring geometry\n".as_ptr().cast_mut(),
        c"../src/manage.c".as_ptr(),
        c"restore_geometry".as_ptr(),
        79 as ::core::ffi::c_int,
    );
    for id in containers() {
        let Some(con) = resolve(id) else { continue };
        if !(*con).window.is_null() {
            debuglog(
                c"%s:%s:%d - Re-adding X11 border of %d px\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"restore_geometry".as_ptr(),
                84 as ::core::ffi::c_int,
                (*con).border_width,
            );
            (*con).window_rect.width = (*con)
                .window_rect
                .width
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as u32);
            (*con).window_rect.height = (*con)
                .window_rect
                .height
                .wrapping_add((2 as ::core::ffi::c_int * (*con).border_width) as u32);
            xcb_set_window_rect(conn, (*(*con).window).id, (*con).window_rect as Rect);
            debuglog(
                c"%s:%s:%d - placing window %08x at %d %d\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"restore_geometry".as_ptr(),
                88 as ::core::ffi::c_int,
                (*(*con).window).id,
                (*con).rect.x,
                (*con).rect.y,
            );
            connection().send_request(&x::ReparentWindow {
                window: x::Window::new((*(*con).window).id),
                parent: x::Window::new(root),
                x: (*con).rect.x as i16,
                y: (*con).rect.y as i16,
            });
        }
    }
    connection().send_request(&x::ChangeWindowAttributes {
        window: x::Window::new(root),
        value_list: &[x::Cw::EventMask(x::EventMask::SUBSTRUCTURE_REDIRECT)],
    });
    let cookie = connection().send_request(&x::GetInputFocus {});
    let _ = connection().wait_for_reply(cookie);
}

pub unsafe fn manage_window(window: u32, cookie: AttributesCookie, needs_to_be_mapped: bool) {
    let mut values: [u32; 1] = [0; 1];
    let mut cwindow: *mut i3Window = ::core::ptr::null_mut::<i3Window>();
    let mut buttons: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut urgency_hint: bool = false;
    let mut motif_border_style: BorderStyle = BorderStyle::None;
    let mut has_mwm_hints: bool = false;
    let mut startup_ws: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut search_at: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let mut nc: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut match_from_restart_mode: bool = false;
    let mut old_frame: u32 = 0;
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut ws: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut fs: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut set_focus: bool = false;
    let mut want_floating: bool = false;
    debuglog(
        c"%s:%s:%d - window 0x%08x\n".as_ptr().cast_mut(),
        c"../src/manage.c".as_ptr(),
        c"manage_window".as_ptr(),
        108 as ::core::ffi::c_int,
        window,
    );
    let geomc = connection().send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(x::Window::new(window)),
    });
    let attr = attribute_reply(cookie);
    if attr.is_none() {
        debuglog(
            c"%s:%s:%d - Could not get attributes\n".as_ptr().cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"manage_window".as_ptr(),
            129 as ::core::ffi::c_int,
        );
        xcb_interop::transport::discard_reply(&connection(), geomc);
        return;
    }
    let attr = attr.unwrap();
    if needs_to_be_mapped && !attributes_viewable(&attr) {
        xcb_interop::transport::discard_reply(&connection(), geomc);
    } else if attr.override_redirect() {
        xcb_interop::transport::discard_reply(&connection(), geomc);
    } else if !con_by_window_id(window).is_null() {
        debuglog(
            c"%s:%s:%d - already managed (by con %p)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"manage_window".as_ptr(),
            147 as ::core::ffi::c_int,
            con_by_window_id(window),
        );
        xcb_interop::transport::discard_reply(&connection(), geomc);
    } else {
        let geom = connection().wait_for_reply(geomc).ok();
        if geom.is_none() {
            debuglog(
                c"%s:%s:%d - could not get geometry\n".as_ptr().cast_mut(),
                c"../src/manage.c".as_ptr(),
                c"manage_window".as_ptr(),
                154 as ::core::ffi::c_int,
            );
        } else {
            let mut geom = crate::window::WindowGeometry::from(geom.as_ref().unwrap());
            values = [0; 1];
            values[0 as ::core::ffi::c_int as usize] =
                ((::xcb::x::EventMask::PROPERTY_CHANGE.bits()) as ::core::ffi::c_int
                    | (::xcb::x::EventMask::STRUCTURE_NOTIFY.bits()) as ::core::ffi::c_int)
                    as u32;
            let event_mask_cookie = connection().send_request_checked(&x::ChangeWindowAttributes {
                window: x::Window::new(window),
                value_list: &[x::Cw::EventMask(x::EventMask::from_bits_retain(values[0]))],
            });
            if request_failed(event_mask_cookie) {
                verboselog(
                    c"Could not change event mask, the window probably already disappeared.\n"
                        .as_ptr()
                        .cast_mut(),
                );
            } else {
                let wm_type_cookie = property_cookie(
                    window,
                    A__NET_WM_WINDOW_TYPE,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let strut_cookie = property_cookie(
                    window,
                    A__NET_WM_STRUT_PARTIAL,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let state_cookie = property_cookie(
                    window,
                    A__NET_WM_STATE,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let utf8_title_cookie = property_cookie(
                    window,
                    A__NET_WM_NAME,
                    0 as ::core::ffi::c_int as u32,
                    128 as u32,
                );
                let leader_cookie = property_cookie(
                    window,
                    A_WM_CLIENT_LEADER,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let transient_cookie = property_cookie(
                    window,
                    (::xcb::x::ATOM_WM_TRANSIENT_FOR.resource_id()) as ::core::ffi::c_int as u32,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let title_cookie = property_cookie(
                    window,
                    (::xcb::x::ATOM_WM_NAME.resource_id()) as ::core::ffi::c_int as u32,
                    0 as ::core::ffi::c_int as u32,
                    128 as u32,
                );
                let class_cookie = property_cookie(
                    window,
                    (::xcb::x::ATOM_WM_CLASS.resource_id()) as ::core::ffi::c_int as u32,
                    0 as ::core::ffi::c_int as u32,
                    128 as u32,
                );
                let role_cookie = property_cookie(
                    window,
                    A_WM_WINDOW_ROLE,
                    0 as ::core::ffi::c_int as u32,
                    128 as u32,
                );
                let startup_id_cookie = property_cookie(
                    window,
                    A__NET_STARTUP_ID,
                    0 as ::core::ffi::c_int as u32,
                    512 as u32,
                );
                let wm_hints_cookie = property_cookie(
                    window,
                    x::ATOM_WM_HINTS.resource_id(),
                    x::ATOM_WM_HINTS.resource_id(),
                    9,
                );
                let wm_normal_hints_cookie = property_cookie(
                    window,
                    x::ATOM_WM_NORMAL_HINTS.resource_id(),
                    x::ATOM_WM_SIZE_HINTS.resource_id(),
                    18,
                );
                let motif_wm_hints_cookie = property_cookie(
                    window,
                    A__MOTIF_WM_HINTS,
                    0 as ::core::ffi::c_int as u32,
                    5_usize.wrapping_mul(::core::mem::size_of::<u64>() as usize) as u32,
                );
                let wm_user_time_cookie = property_cookie(
                    window,
                    A__NET_WM_USER_TIME,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let wm_desktop_cookie = property_cookie(
                    window,
                    A__NET_WM_DESKTOP,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let wm_machine_cookie = property_cookie(
                    window,
                    (::xcb::x::ATOM_WM_CLIENT_MACHINE.resource_id()) as ::core::ffi::c_int as u32,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                let wm_icon_cookie = property_cookie(
                    window,
                    A__NET_WM_ICON,
                    0 as ::core::ffi::c_int as u32,
                    4294967295 as u32,
                );
                cwindow = crate::con::arena::allocate_window(i3Window::default());
                (*cwindow).id = window;
                (*cwindow).depth = get_visual_depth(attr.visual());
                buttons = bindings_get_buttons_to_grab();
                xcb_grab_buttons(conn, window, buttons);
                free(buttons as *mut ::core::ffi::c_void);
                buttons = ::core::ptr::null_mut::<::core::ffi::c_int>();
                window_update_class(cwindow, property_reply(class_cookie).as_ref());
                window_update_name_legacy(cwindow, property_reply(title_cookie).as_ref());
                window_update_name(cwindow, property_reply(utf8_title_cookie).as_ref());
                window_update_icon(cwindow, property_reply(wm_icon_cookie).as_ref());
                window_update_leader(cwindow, property_reply(leader_cookie).as_ref());
                window_update_transient_for(cwindow, property_reply(transient_cookie).as_ref());
                window_update_strut_partial(cwindow, property_reply(strut_cookie).as_ref());
                window_update_role(cwindow, property_reply(role_cookie).as_ref());
                urgency_hint = false;
                window_update_hints(
                    cwindow,
                    property_reply(wm_hints_cookie).as_ref(),
                    &raw mut urgency_hint,
                );
                motif_border_style = BorderStyle::None;
                has_mwm_hints = window_update_motif_hints(
                    cwindow,
                    property_reply(motif_wm_hints_cookie).as_ref(),
                    &raw mut motif_border_style,
                );
                let normal_hints_reply = property_reply(wm_normal_hints_cookie);
                // window_update_normal_hints borrows its reply; this request owns it.
                window_update_normal_hints(cwindow, normal_hints_reply.as_ref(), Some(&mut geom));
                window_update_machine(cwindow, property_reply(wm_machine_cookie).as_ref());
                let type_reply = property_reply(wm_type_cookie);
                let state_reply = property_reply(state_cookie);
                let startup_id_reply = property_reply(startup_id_cookie);
                startup_ws = startup_workspace_for_window(cwindow, startup_id_reply.as_ref());
                debuglog(
                    c"%s:%s:%d - startup workspace = %s\n".as_ptr().cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    227 as ::core::ffi::c_int,
                    startup_ws,
                );
                let wm_desktop_reply = property_reply(wm_desktop_cookie);
                (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as u32;
                if let Some(desktop) = first_property_word(wm_desktop_reply.as_ref()) {
                    (*cwindow).wm_desktop = desktop;
                }
                (*cwindow).needs_take_focus =
                    window_supports_protocol((*cwindow).id, A_WM_TAKE_FOCUS);
                (*cwindow).window_type = xcb_get_preferred_window_type(type_reply.as_ref());
                search_at = croot;
                if xcb_reply_contains_atom(type_reply.as_ref(), A__NET_WM_WINDOW_TYPE_DOCK) {
                    verboselog(c"This window is of type dock\n".as_ptr().cast_mut());
                    let output: *mut Output = get_output_containing(
                        geom.x as ::core::ffi::c_uint,
                        geom.y as ::core::ffi::c_uint,
                    );
                    if !output.is_null() {
                        debuglog(
                            c"%s:%s:%d - Starting search at output %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            252 as ::core::ffi::c_int,
                            output_primary_name(output),
                        );
                        search_at = (*output).con;
                    }
                    if (*cwindow).reserved.top > 0 as u32 && (*cwindow).reserved.bottom == 0 as u32
                    {
                        debuglog(
                            c"%s:%s:%d - Top dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            258 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = WindowDock::Top;
                    } else if (*cwindow).reserved.top == 0 as u32
                        && (*cwindow).reserved.bottom > 0 as u32
                    {
                        debuglog(
                            c"%s:%s:%d - Bottom dock client\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            261 as ::core::ffi::c_int,
                        );
                        (*cwindow).dock = WindowDock::Bottom;
                    } else {
                        debuglog(
                            c"%s:%s:%d - Ignoring invalid reserved edges (_NET_WM_STRUT_PARTIAL), using position as fallback:\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            264 as ::core::ffi::c_int,
                        );
                        if (geom.y as ::core::ffi::c_int)
                            < (*search_at).rect.height.wrapping_div(2 as u32) as i16
                                as ::core::ffi::c_int
                        {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d < rect.height / 2 = %d, it is a top dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                267 as ::core::ffi::c_int,
                                geom.y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as u32),
                            );
                            (*cwindow).dock = WindowDock::Top;
                        } else {
                            debuglog(
                                c"%s:%s:%d - geom->y = %d >= rect.height / 2 = %d, it is a bottom dock client\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                271 as ::core::ffi::c_int,
                                geom.y as ::core::ffi::c_int,
                                (*search_at).rect.height.wrapping_div(2 as u32),
                            );
                            (*cwindow).dock = WindowDock::Bottom;
                        }
                    }
                }
                debuglog(
                    c"%s:%s:%d - Initial geometry: (%d, %d, %d, %d)\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    277 as ::core::ffi::c_int,
                    geom.x as ::core::ffi::c_int,
                    geom.y as ::core::ffi::c_int,
                    geom.width as ::core::ffi::c_int,
                    geom.height as ::core::ffi::c_int,
                );
                (*cwindow).swallowed = false;
                match_0 = ::core::ptr::null_mut::<Match>();
                nc = con_for_window(search_at, cwindow, &raw mut match_0);
                match_from_restart_mode =
                    !match_0.is_null() && (*match_0).restart_mode as ::core::ffi::c_int != 0;
                if nc.is_null() {
                    let mut wm_desktop_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                    let mut assignment: *mut Assignment = ::core::ptr::null_mut::<Assignment>();
                    assignment = assignment_for(cwindow, AssignmentKind::TO_WORKSPACE);
                    if !assignment.is_null() || {
                        assignment = assignment_for(cwindow, AssignmentKind::TO_WORKSPACE_NUMBER);
                        !assignment.is_null()
                    } {
                        debuglog(
                            c"%s:%s:%d - Assignment matches (%p)\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            291 as ::core::ffi::c_int,
                            match_0,
                        );
                        let mut assigned_ws: *mut Con = ::core::ptr::null_mut::<Con>();
                        if (*assignment).type_0 == AssignmentKind::TO_WORKSPACE_NUMBER {
                            let parsed_num: ::core::ffi::c_long = ws_name_to_number(
                                std::ffi::CStr::from_ptr((*assignment).dest.workspace),
                            )
                            .unwrap_or(-1)
                                as ::core::ffi::c_long;
                            assigned_ws =
                                get_existing_workspace_by_num(parsed_num as ::core::ffi::c_int);
                        }
                        if assigned_ws.is_null() {
                            assigned_ws = workspace_get((*assignment).dest.workspace);
                        }
                        nc = con_descend_tiling_focused(assigned_ws);
                        debuglog(
                            c"%s:%s:%d - focused on ws %s: %p / %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            306 as ::core::ffi::c_int,
                            (*assigned_ws).name,
                            nc,
                            (*nc).name,
                        );
                        if ((*nc).type_0) == ContainerType::Workspace {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con(parent_ptr(nc), cwindow);
                        }
                        if !workspace_is_visible(assigned_ws) {
                            urgency_hint = true;
                        }
                    } else if (*cwindow).wm_desktop != NET_WM_DESKTOP_NONE as u32
                        && (*cwindow).wm_desktop != NET_WM_DESKTOP_ALL as u32
                        && {
                            wm_desktop_ws = ewmh_get_workspace_by_index((*cwindow).wm_desktop);
                            !wm_desktop_ws.is_null()
                        }
                    {
                        debuglog(
                            c"%s:%s:%d - Using workspace %p / %s because _NET_WM_DESKTOP = %d.\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            325 as ::core::ffi::c_int,
                            wm_desktop_ws,
                            (*wm_desktop_ws).name,
                            (*cwindow).wm_desktop,
                        );
                        nc = con_descend_tiling_focused(wm_desktop_ws);
                        if ((*nc).type_0) == ContainerType::Workspace {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con(parent_ptr(nc), cwindow);
                        }
                    } else if !startup_ws.is_null() {
                        debuglog(
                            c"%s:%s:%d - Using workspace on which this application was started (%s)\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            335 as ::core::ffi::c_int,
                            startup_ws,
                        );
                        nc = con_descend_tiling_focused(workspace_get(startup_ws));
                        debuglog(
                            c"%s:%s:%d - focused on ws %s: %p / %s\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            337 as ::core::ffi::c_int,
                            startup_ws,
                            nc,
                            (*nc).name,
                        );
                        if ((*nc).type_0) == ContainerType::Workspace {
                            nc = tree_open_con(nc, cwindow);
                        } else {
                            nc = tree_open_con(parent_ptr(nc), cwindow);
                        }
                    } else if ((*focused).type_0) == ContainerType::Con
                        && con_accepts_window(focused) as ::core::ffi::c_int != 0
                    {
                        verboselog(
                            c"using current container, focused = %p, focused->name = %s\n"
                                .as_ptr()
                                .cast_mut(),
                            focused,
                            (*focused).name,
                        );
                        nc = focused;
                    } else {
                        nc = tree_open_con(::core::ptr::null_mut::<Con>(), cwindow);
                    }
                    assignment = assignment_for(cwindow, AssignmentKind::TO_OUTPUT);
                    if !assignment.is_null() {
                        con_move_to_output_name(nc, (*assignment).dest.output, true);
                    }
                } else {
                    if !match_0.is_null() && ((*match_0).insert_where) == MatchInsertWhere::Below {
                        nc = tree_open_con(nc, cwindow);
                    }
                    if !match_0.is_null() && ((*match_0).insert_where) != MatchInsertWhere::Below {
                        debuglog(
                            c"%s:%s:%d - Removing match %p from container %p\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            369 as ::core::ffi::c_int,
                            match_0,
                            nc,
                        );
                        if !(*match_0).matches.tqe_next.is_null() {
                            (*(*match_0).matches.tqe_next).matches.tqe_prev =
                                (*match_0).matches.tqe_prev;
                        } else {
                            (*nc).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                        }
                        *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                        drop(Box::from_raw(match_0));
                        match_0 = ::core::ptr::null_mut::<Match>();
                    }
                    (*cwindow).swallowed = true;
                }
                debuglog(
                    c"%s:%s:%d - new container = %p\n".as_ptr().cast_mut(),
                    c"../src/manage.c".as_ptr(),
                    c"manage_window".as_ptr(),
                    378 as ::core::ffi::c_int,
                    nc,
                );
                if !(*nc).window.is_null() && (*nc).window != cwindow {
                    if !restore_kill_placeholder((*(*nc).window).id) {
                        debuglog(
                            c"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this to-be-managed window?!\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            381 as ::core::ffi::c_int,
                        );
                    } else {
                        _remove_matches(nc);
                    }
                }
                old_frame = 0 as u32;
                if (*nc).window != cwindow && !(*nc).window.is_null() {
                    window_free((*nc).window as *mut i3Window);
                    old_frame = _match_depth(cwindow, nc);
                }
                (*nc).window = cwindow as *mut Window;
                x_reinit(nc);
                (*nc).border_width = geom.border_width as ::core::ffi::c_int;
                name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                sasprintf(
                    &raw mut name,
                    c"[i3 con] container around %p".as_ptr(),
                    cwindow,
                );
                x_set_name(nc, name);
                free(name as *mut ::core::ffi::c_void);
                ws = con_get_workspace(nc);
                fs = con_get_fullscreen_covering_ws(ws);
                if xcb_reply_contains_atom(state_reply.as_ref(), A__NET_WM_STATE_FULLSCREEN) {
                    if fs != nc {
                        let output_0: *mut Output = get_output_with_dimensions(Rect {
                            x: geom.x as u32,
                            y: geom.y as u32,
                            width: geom.width as u32,
                            height: geom.height as u32,
                        });
                        if !output_0.is_null() {
                            con_move_to_output(nc, output_0, false);
                        }
                        con_toggle_fullscreen(nc, FullscreenMode::Output);
                    }
                    fs = ::core::ptr::null_mut::<Con>();
                }
                set_focus = false;
                if fs.is_null() {
                    debuglog(
                        c"%s:%s:%d - Not in fullscreen mode, focusing\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        426 as ::core::ffi::c_int,
                    );
                    if (*cwindow).dock as u64 == 0 {
                        let current_output: *mut Con = con_get_output(focused);
                        let target_output: *mut Con = con_get_output(ws);
                        if workspace_is_visible(ws) as ::core::ffi::c_int != 0
                            && current_output == target_output
                        {
                            if !match_from_restart_mode {
                                set_focus = true;
                            } else {
                                debuglog(
                                    c"%s:%s:%d - not focusing, matched with restart_mode == true\n"
                                        .as_ptr()
                                        .cast_mut(),
                                    c"../src/manage.c".as_ptr(),
                                    c"manage_window".as_ptr(),
                                    438 as ::core::ffi::c_int,
                                );
                            }
                        } else {
                            debuglog(
                                c"%s:%s:%d - workspace not visible, not focusing\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                441 as ::core::ffi::c_int,
                            );
                        }
                    } else {
                        debuglog(
                            c"%s:%s:%d - dock, not focusing\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            444 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    debuglog(
                        c"%s:%s:%d - fs = %p, ws = %p, not focusing\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        447 as ::core::ffi::c_int,
                        fs,
                        ws,
                    );
                    let owner =
                        crate::con::arena::identity(parent_ptr(nc)).expect("live managed parent");
                    let member = crate::con::arena::identity(nc).expect("live managed child");
                    let first = focus::first(owner).expect("nonempty managed focus order");
                    if first != member {
                        if focus::owner(member) == Some(owner) {
                            focus::move_after(owner, member, first)
                                .expect("existing managed focus member");
                        } else {
                            // Ancestry and focus membership are updated at
                            // separate graph boundaries; preserve unlinking
                            // from the actual owner before inserting here.
                            let former = focus::owner(member).expect("managed focus membership");
                            focus::remove(former, member).expect("live former focus owner");
                            focus::insert_after(owner, member, first)
                                .expect("managed focus insertion");
                        }
                    }
                }
                want_floating = false;
                if xcb_reply_contains_atom(type_reply.as_ref(), A__NET_WM_WINDOW_TYPE_DIALOG)
                    as ::core::ffi::c_int
                    != 0
                    || xcb_reply_contains_atom(type_reply.as_ref(), A__NET_WM_WINDOW_TYPE_UTILITY)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply.as_ref(), A__NET_WM_WINDOW_TYPE_TOOLBAR)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(
                        type_reply.as_ref(),
                        A__NET_WM_WINDOW_TYPE_NOTIFICATION,
                    ) as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(type_reply.as_ref(), A__NET_WM_WINDOW_TYPE_SPLASH)
                        as ::core::ffi::c_int
                        != 0
                    || xcb_reply_contains_atom(state_reply.as_ref(), A__NET_WM_STATE_MODAL)
                        as ::core::ffi::c_int
                        != 0
                    || (*cwindow).max_width > 0 as ::core::ffi::c_int
                        && (*cwindow).max_height > 0 as ::core::ffi::c_int
                        && (*cwindow).min_height == (*cwindow).max_height
                        && (*cwindow).min_width == (*cwindow).max_width
                {
                    verboselog(
                        c"This window is a dialog window, setting floating\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                    want_floating = true;
                }
                if xcb_reply_contains_atom(state_reply.as_ref(), A__NET_WM_STATE_STICKY) {
                    (*nc).sticky = true;
                }
                if (*cwindow).wm_desktop == NET_WM_DESKTOP_ALL as u32
                    && (ws.is_null() || !con_is_internal(ws))
                {
                    debuglog(
                        c"%s:%s:%d - This window has _NET_WM_DESKTOP = 0xFFFFFFFF. Will float it and make it sticky.\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        484 as ::core::ffi::c_int,
                    );
                    (*nc).sticky = true;
                    want_floating = true;
                }
                drop(state_reply);
                drop(type_reply);
                if (*cwindow).transient_for as ::core::ffi::c_long != 0
                    || (*cwindow).leader as ::core::ffi::c_long != 0
                        && (*cwindow).leader != (*cwindow).id
                        && !con_by_window_id((*cwindow).leader).is_null()
                {
                    debuglog(
                        c"%s:%s:%d - This window is transient for another window, setting floating\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        496 as ::core::ffi::c_int,
                    );
                    want_floating = true;
                    if config.popup_during_fullscreen == PopupDuringFullscreen::LeaveFullscreen
                        && !fs.is_null()
                    {
                        debuglog(
                            c"%s:%s:%d - There is a fullscreen window, leaving fullscreen mode\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            501 as ::core::ffi::c_int,
                        );
                        con_disable_fullscreen(fs);
                    } else if config.popup_during_fullscreen == PopupDuringFullscreen::Smart
                        && !fs.is_null()
                        && !(*fs).window.is_null()
                    {
                        set_focus = con_find_transient_for_window(nc, (*(*fs).window).id);
                    }
                }
                if (*cwindow).dock as u64 != 0 {
                    want_floating = false;
                }
                if (*nc).geometry.width == 0 as u32 {
                    (*nc).geometry = Rect {
                        x: geom.x as u32,
                        y: geom.y as u32,
                        width: geom.width as u32,
                        height: geom.height as u32,
                    } as Rect;
                }
                if config.popup_during_fullscreen == PopupDuringFullscreen::All
                    && want_floating as ::core::ffi::c_int != 0
                    && !fs.is_null()
                {
                    set_focus = true;
                }
                if want_floating {
                    debuglog(
                        c"%s:%s:%d - geometry = %d x %d\n".as_ptr().cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        529 as ::core::ffi::c_int,
                        (*nc).geometry.width,
                        (*nc).geometry.height,
                    );
                    if floating_enable(nc, true) {
                        (*nc).floating = FloatingState::AutoOn;
                    }
                }
                if has_mwm_hints {
                    debuglog(
                        c"%s:%s:%d - MOTIF_WM_HINTS specifies decorations (border_style = %d)\n"
                            .as_ptr()
                            .cast_mut(),
                        c"../src/manage.c".as_ptr(),
                        c"manage_window".as_ptr(),
                        536 as ::core::ffi::c_int,
                        motif_border_style as ::core::ffi::c_uint,
                    );
                    if want_floating {
                        con_set_border_style(
                            nc,
                            motif_border_style,
                            config.default_floating_border_width,
                        );
                    } else {
                        con_set_border_style(nc, motif_border_style, config.default_border_width);
                    }
                }
                if (*nc).current_border_width == -(1 as ::core::ffi::c_int) {
                    (*nc).current_border_width = if want_floating as ::core::ffi::c_int != 0 {
                        config.default_floating_border_width
                    } else {
                        config.default_border_width
                    };
                }
                values[0 as ::core::ffi::c_int as usize] = 0 as u32;
                connection().send_request(&x::ChangeWindowAttributes {
                    window: x::Window::new(window),
                    value_list: &[x::Cw::EventMask(x::EventMask::from_bits_retain(values[0]))],
                });
                let rcookie = connection().send_request_checked(&x::ReparentWindow {
                    window: x::Window::new(window),
                    parent: x::Window::new((*nc).frame.id as u32),
                    x: 0 as i16,
                    y: 0 as i16,
                });
                if request_failed(rcookie) {
                    verboselog(
                        c"Could not reparent the window, aborting\n"
                            .as_ptr()
                            .cast_mut(),
                    );
                } else {
                    values[0 as ::core::ffi::c_int as usize] =
                        (((::xcb::x::EventMask::PROPERTY_CHANGE.bits()) as ::core::ffi::c_int
                            | (::xcb::x::EventMask::STRUCTURE_NOTIFY.bits()) as ::core::ffi::c_int
                            | (::xcb::x::EventMask::FOCUS_CHANGE.bits()) as ::core::ffi::c_int)
                            & !((::xcb::x::EventMask::ENTER_WINDOW.bits()) as ::core::ffi::c_int))
                            as u32;
                    connection().send_request(&x::ChangeWindowAttributes {
                        window: x::Window::new(window),
                        value_list: &[x::Cw::EventMask(x::EventMask::from_bits_retain(values[0]))],
                    });
                    let _ = connection().flush();
                    connection().send_request(&x::ChangeSaveSet {
                        mode: x::SetMode::Insert,
                        window: x::Window::new(window),
                    });
                    if shape_supported {
                        connection().send_request(&xcb::shape::SelectInput {
                            destination_window: x::Window::new(window),
                            enable: true,
                        });
                        let cookie = connection().send_request(&xcb::shape::QueryExtents {
                            destination_window: x::Window::new(window),
                        });
                        if connection()
                            .wait_for_reply(cookie)
                            .is_ok_and(|reply| reply.bounding_shaped())
                        {
                            (*cwindow).shaped = true;
                        }
                    }
                    run_assignments(cwindow);
                    ws = con_get_workspace(nc);
                    if !ws.is_null() && !workspace_is_visible(ws) {
                        (*ws).rect = (*parent_ptr(ws)).rect;
                        render_con(ws);
                        set_focus = false;
                    }
                    render_con(croot);
                    (*cwindow).managed_since = time(::core::ptr::null_mut::<libc::time_t>());
                    ipc_send_window_event(c"new".as_ptr(), nc);
                    if set_focus as ::core::ffi::c_int != 0
                        && !assignment_for(cwindow, AssignmentKind::NO_FOCUS).is_null()
                    {
                        if con_num_windows(ws) == 1 as ::core::ffi::c_int {
                            debuglog(
                                c"%s:%s:%d - This is the first window on this workspace, ignoring no_focus.\n".as_ptr().cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                626 as ::core::ffi::c_int,
                            );
                        } else {
                            debuglog(
                                c"%s:%s:%d - no_focus was set for con = %p, not setting focus.\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                628 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false;
                        }
                    }
                    if set_focus {
                        debuglog(
                            c"%s:%s:%d - Checking con = %p for _NET_WM_USER_TIME.\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            634 as ::core::ffi::c_int,
                            nc,
                        );
                        let wm_user_time_reply = property_reply(wm_user_time_cookie);
                        if first_property_word(wm_user_time_reply.as_ref()) == Some(0) {
                            debuglog(
                                c"%s:%s:%d - _NET_WM_USER_TIME set to 0, not focusing con = %p.\n"
                                    .as_ptr()
                                    .cast_mut(),
                                c"../src/manage.c".as_ptr(),
                                c"manage_window".as_ptr(),
                                641 as ::core::ffi::c_int,
                                nc,
                            );
                            set_focus = false;
                        }
                    } else {
                        xcb_interop::transport::discard_reply(&connection(), wm_user_time_cookie);
                    }
                    if (*(*nc).window).doesnt_accept_focus as ::core::ffi::c_int != 0
                        && !(*(*nc).window).needs_take_focus
                    {
                        set_focus = false;
                    }
                    if set_focus as ::core::ffi::c_int != 0
                        && (*nc).mapped as ::core::ffi::c_int != 0
                    {
                        debuglog(
                            c"%s:%s:%d - Now setting focus.\n".as_ptr().cast_mut(),
                            c"../src/manage.c".as_ptr(),
                            c"manage_window".as_ptr(),
                            662 as ::core::ffi::c_int,
                        );
                        con_activate(nc);
                    }
                    tree_render();
                    if old_frame as ::core::ffi::c_long != 0 {
                        connection().send_request(&x::DestroyWindow {
                            window: x::Window::new(old_frame),
                        });
                    }
                    con_set_urgency(nc, urgency_hint);
                    (*cwindow).wm_desktop = NET_WM_DESKTOP_NONE as u32;
                    ewmh_update_wm_desktop();
                    output_push_sticky_windows(focused);
                }
            }
        }
    }
}
unsafe fn placeholder_for_con(con: *mut Con) -> *mut Con {
    if (*(*con).window).swallowed {
        return ::core::ptr::null_mut::<Con>();
    }
    let mut match_0: *mut Match = ::core::ptr::null_mut::<Match>();
    let nc: *mut Con = con_for_window(croot, (*con).window as *mut i3Window, &raw mut match_0);
    if nc.is_null() || (*nc).window.is_null() || (*nc).window == (*con).window {
        return ::core::ptr::null_mut::<Con>();
    }
    if (*(*nc).window).managed_since > (*(*con).window).managed_since {
        return ::core::ptr::null_mut::<Con>();
    }
    nc
}

pub unsafe fn remanage_window(con: *mut Con) -> *mut Con {
    let nc: *mut Con = placeholder_for_con(con);
    if nc.is_null() {
        run_assignments((*con).window as *mut i3Window);
        return con;
    }
    if !restore_kill_placeholder((*(*nc).window).id) {
        debuglog(
            c"%s:%s:%d - Uh?! Container without a placeholder, but with a window, has swallowed this managed window?!\n".as_ptr().cast_mut(),
            c"../src/manage.c".as_ptr(),
            c"remanage_window".as_ptr(),
            726 as ::core::ffi::c_int,
        );
    } else {
        _remove_matches(nc);
    }
    window_free((*nc).window as *mut i3Window);
    let old_frame: u32 = _match_depth((*con).window as *mut i3Window, nc);
    x_reparent_child(nc, con);
    let moved_workpaces: bool = con_get_workspace(nc) != con_get_workspace(con);
    con_merge_into(con, nc);
    if old_frame as ::core::ffi::c_long != 0 {
        connection().send_request(&x::DestroyWindow {
            window: x::Window::new(old_frame),
        });
    }
    run_assignments((*nc).window as *mut i3Window);
    if moved_workpaces {
        startup_sequence_delete_by_window((*nc).window);
        ewmh_update_wm_desktop();
    }
    (*(*nc).window).swallowed = true;
    nc
}

// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::bindings::bindings_get_buttons_to_grab;
use crate::ewmh::ewmh_update_wm_desktop;
use crate::restore_layout::restore_kill_placeholder;
use crate::tree::tree_render;
use crate::util::ws_name_to_number;
use crate::x::window_supports_protocol;
use i3_common::draw_util::get_visual_depth;
use i3_common::safewrappers::scalloc;

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

pub use libc::{free, time};

use crate::log::{debuglog, verboselog};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::xoutput;
pub use crate::types::Assignment;
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::application::conn;
use crate::application::root;
use crate::application::shape_supported;
use crate::application::A_WM_CLIENT_LEADER;
use crate::application::A_WM_TAKE_FOCUS;
use crate::application::A_WM_WINDOW_ROLE;
use crate::application::A__MOTIF_WM_HINTS;
use crate::application::A__NET_STARTUP_ID;
use crate::application::A__NET_WM_DESKTOP;
use crate::application::A__NET_WM_ICON;
use crate::application::A__NET_WM_NAME;
use crate::application::A__NET_WM_STATE;
use crate::application::A__NET_WM_STATE_FULLSCREEN;
use crate::application::A__NET_WM_STATE_MODAL;
use crate::application::A__NET_WM_STATE_STICKY;
use crate::application::A__NET_WM_STRUT_PARTIAL;
use crate::application::A__NET_WM_USER_TIME;
use crate::application::A__NET_WM_WINDOW_TYPE;
use crate::application::A__NET_WM_WINDOW_TYPE_DIALOG;
use crate::application::A__NET_WM_WINDOW_TYPE_DOCK;
use crate::application::A__NET_WM_WINDOW_TYPE_NOTIFICATION;
use crate::application::A__NET_WM_WINDOW_TYPE_SPLASH;
use crate::application::A__NET_WM_WINDOW_TYPE_TOOLBAR;
use crate::application::A__NET_WM_WINDOW_TYPE_UTILITY;
use crate::assignments::assignment_for;
use crate::assignments::run_assignments;
use crate::con::con_accepts_window;
use crate::con::con_activate;
use crate::con::con_by_window_id;
use crate::con::con_descend_tiling_focused;
use crate::con::con_disable_fullscreen;
use crate::con::con_find_transient_for_window;
use crate::con::con_for_window;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_get_output;
use crate::con::con_get_workspace;
use crate::con::con_is_internal;
use crate::con::con_merge_into;
use crate::con::con_move_to_output;
use crate::con::con_move_to_output_name;
use crate::con::con_num_windows;
use crate::con::con_set_border_style;
use crate::con::con_set_urgency;
use crate::con::con_toggle_fullscreen;
use crate::config::config;
use crate::ewmh::ewmh_get_workspace_by_index;
use crate::floating::floating_enable;
use crate::ipc::ipc_send_window_event;
use crate::output::output_primary_name;
use crate::output::output_push_sticky_windows;
use crate::randr::get_output_containing;
use crate::randr::get_output_with_dimensions;
use crate::render::render_con;
use crate::startup::startup_sequence_delete_by_window;
use crate::startup::startup_workspace_for_window;
use crate::tree::croot;
use crate::tree::focused;
use crate::tree::tree_open_con;
use crate::window::window_free;
use crate::window::window_update_class;
use crate::window::window_update_hints;
use crate::window::window_update_icon;
use crate::window::window_update_leader;
use crate::window::window_update_machine;
use crate::window::window_update_motif_hints;
use crate::window::window_update_name;
use crate::window::window_update_name_legacy;
use crate::window::window_update_normal_hints;
use crate::window::window_update_role;
use crate::window::window_update_strut_partial;
use crate::window::window_update_transient_for;
use crate::workspace::get_existing_workspace_by_num;
use crate::workspace::workspace_get;
use crate::workspace::workspace_is_visible;
use crate::x::x_con_reframe;
use crate::x::x_reinit;
use crate::x::x_reparent_child;
use crate::x::x_set_name;
use crate::xcb::xcb_get_preferred_window_type;
use crate::xcb::xcb_grab_buttons;
use crate::xcb::xcb_reply_contains_atom;
use crate::xcb::xcb_set_window_rect;

#[cfg(test)]
mod owned_reply_tests {
    use super::*;
    #[test]
    fn tree_child_count_is_limited_by_native_wire_allocation() {
        assert_eq!(tree_child_count(0, 0), Some(0));
        assert_eq!(tree_child_count(2, 2), Some(2));
        assert_eq!(tree_child_count(2, 1), None);
        assert_eq!(
            tree_child_count(u16::MAX, u32::MAX),
            Some(u16::MAX as usize)
        );
    }
    #[test]
    fn attribute_ingress_compares_unknown_map_states_without_enum_conversion() {
        unsafe {
            let raw = libc::calloc(1, 44).cast::<u8>();
            assert!(!raw.is_null());
            raw.write(1);
            raw.add(4).cast::<u32>().write_unaligned(3);
            raw.add(26).write(u8::MAX);
            let reply: x::GetWindowAttributesReply = xcb::Reply::from_raw(raw);
            assert!(!attributes_viewable(&reply));
            raw.add(26).write(x::MapState::Viewable as u8);
            assert!(attributes_viewable(&reply));
            raw.add(26).write(x::MapState::Unmapped as u8);
            assert!(!attributes_viewable(&reply));
        }
    }
    #[test]
    fn first_property_word_checks_complete_bytes_and_retains_native_formats() {
        unsafe {
            let raw = libc::calloc(1, 36).cast::<u8>();
            assert!(!raw.is_null());
            raw.write(1);
            raw.add(1).write(8);
            raw.add(4).cast::<u32>().write_unaligned(1);
            raw.add(16).cast::<u32>().write_unaligned(3);
            std::ptr::copy_nonoverlapping([0x12u8, 0x34, 0x56, 0x78].as_ptr(), raw.add(32), 4);
            let reply: x::GetPropertyReply = xcb::Reply::from_raw(raw);
            assert_eq!(first_property_word(Some(&reply)), None);
            raw.add(16).cast::<u32>().write_unaligned(4);
            assert_eq!(
                first_property_word(Some(&reply)),
                Some(u32::from_ne_bytes([0x12, 0x34, 0x56, 0x78]))
            );
            raw.add(1).write(32);
            raw.add(16).cast::<u32>().write_unaligned(1);
            assert!(first_property_word(Some(&reply)).is_some());
            // A nonstandard format still exposes its first four native bytes.
            raw.add(1).write(64);
            assert!(first_property_word(Some(&reply)).is_some());
            raw.add(4).cast::<u32>().write_unaligned(0);
            assert_eq!(first_property_word(Some(&reply)), None);
            assert_eq!(first_property_word(None), None);
        }
    }
}
