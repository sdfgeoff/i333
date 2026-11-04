use crate::con::arena::parent::{parent_ptr, set_parent_ptr};
use crate::con::arena::{focus, identity, nodes, resolve};
use crate::domains::{ContainerType, DockMatch, FullscreenMode, MatchInsertWhere};
use crate::regex::compile_or_report;
use crate::types::BorderStyle;
use crate::types::FloatingState;
use crate::types::JsonContent;
use crate::types::Layout;
use crate::types::MarkMode;
use crate::types::ScratchpadState;
use i3_common::json_stream::{Event, EventHandler, ParseError, Parser};
use i3_common::safewrappers::sasprintf;
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::rc::Rc;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * load_layout.c: Restore (parts of) the layout, for example after an inplace
 *                restart.
 *
 */
// Mechanical file-by-file Rust translation of src/load_layout.c.
// Originally translated with C2Rust 0.22.1; subsequently maintained in Rust.

pub type i3Window = Window;

const true_0: i32 = 1;
const false_0: i32 = 0;
fn property_string(bytes: &[u8]) -> CString {
    CString::new(bytes.split(|byte| *byte == 0).next().unwrap()).unwrap()
}
// Keep the legacy independent parsing flags: malformed/overlapping keys can set
// more than one, so replacing them with a strict schema changes partial layouts.
fn key_is(key: &CStr, name: &CStr) -> bool {
    unsafe { strcasecmp(key.as_ptr(), name.as_ptr()) == 0 }
}
fn rect_value(mut rect: Rect, key: &CStr, value: i64) -> Option<Rect> {
    if key_is(key, c"x") {
        rect.x = value as u32;
    } else if key_is(key, c"y") {
        rect.y = value as u32;
    } else if key_is(key, c"width") {
        rect.width = value as u32;
    } else if key_is(key, c"height") {
        rect.height = value as u32;
    } else {
        return None;
    }
    Some(rect)
}
struct LayoutState {
    last_key: CString,
    incomplete: i32,
    json_node: *mut Con,
    to_focus: *mut Con,
    parsing_gaps: bool,
    parsing_swallows: bool,
    parsing_rect: bool,
    parsing_actual_deco_rect: bool,
    parsing_deco_rect: bool,
    parsing_window_rect: bool,
    parsing_geometry: bool,
    parsing_focus: bool,
    parsing_marks: bool,
    current_swallow: *mut Match,
    swallow_is_empty: bool,
    pending_marks: Vec<(CString, *mut Con)>,
    focus_mappings: Vec<i32>,
}
impl LayoutState {
    /// con is a live event-thread graph node for all graph event callbacks.
    unsafe fn new(con: *mut Con) -> Self {
        Self {
            last_key: CString::default(),
            incomplete: 0,
            json_node: con,
            to_focus: std::ptr::null_mut(),
            parsing_gaps: false,
            parsing_swallows: false,
            parsing_rect: false,
            parsing_actual_deco_rect: false,
            parsing_deco_rect: false,
            parsing_window_rect: false,
            parsing_geometry: false,
            parsing_focus: false,
            parsing_marks: false,
            current_swallow: std::ptr::null_mut(),
            swallow_is_empty: false,
            pending_marks: Vec::new(),
            focus_mappings: Vec::new(),
        }
    }
    unsafe fn json_start_map(&mut self) -> ::core::ffi::c_int {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = self.last_key.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::info!("start of map, last_key = {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "start of map, last_key = ".as_bytes(),
                        &log_value_0,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if self.parsing_swallows {
            ::log::info!("creating new swallow\n");
            self.current_swallow = Box::into_raw(Box::new(Match::empty()));

            (*self.current_swallow).dock = DockMatch::DontCheck;
            (*self.current_swallow).matches.tqe_next = ::core::ptr::null_mut::<Match>();
            (*self.current_swallow).matches.tqe_prev = (*self.json_node).swallow_head.tqh_last;
            *(*self.json_node).swallow_head.tqh_last = self.current_swallow;
            (*self.json_node).swallow_head.tqh_last =
                &raw mut (*self.current_swallow).matches.tqe_next;
            self.swallow_is_empty = true;
        } else if !self.parsing_rect
            && !self.parsing_actual_deco_rect
            && !self.parsing_deco_rect
            && !self.parsing_window_rect
            && !self.parsing_geometry
            && !self.parsing_gaps
        {
            if key_is(&self.last_key, c"floating_nodes") {
                ::log::debug!("../src/load_layout.c:json_start_map:72 - New floating_node\n");
                let ws: *mut Con = con_get_workspace(self.json_node);
                self.json_node = con_new_skeleton(
                    ::core::ptr::null_mut::<Con>(),
                    ::core::ptr::null_mut::<i3Window>(),
                );
                (*self.json_node).name = None;
                set_parent_ptr(self.json_node, ws).expect("live acyclic ancestry");
                ::log::debug!(
                    "../src/load_layout.c:json_start_map:77 - Parent is workspace = {}\n",
                    i3_common::logging::pointer(ws)
                );
            } else {
                let parent: *mut Con = self.json_node;
                self.json_node = con_new_skeleton(
                    ::core::ptr::null_mut::<Con>(),
                    ::core::ptr::null_mut::<i3Window>(),
                );
                (*self.json_node).name = None;
                set_parent_ptr(self.json_node, parent).expect("live acyclic ancestry");
            }
            self.incomplete += 1;
            ::log::debug!(
                "../src/load_layout.c:json_start_map:86 - incomplete = {}\n",
                { (self.incomplete) }
            );
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_end_map(&mut self) -> ::core::ffi::c_int {
        ::log::info!("end of map\n");
        if !self.parsing_swallows
            && !self.parsing_rect
            && !self.parsing_actual_deco_rect
            && !self.parsing_deco_rect
            && !self.parsing_window_rect
            && !self.parsing_geometry
            && !self.parsing_gaps
        {
            if (*self.json_node).layout == Layout::Default {
                ::log::debug!(
                    "../src/load_layout.c:json_end_map:103 - Setting layout = L_SPLITH\n"
                );
                (*self.json_node).layout = Layout::SplitH;
            }
            if con_is_split(self.json_node) as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && !(*self.json_node).swallow_head.tqh_first.is_null()
            {
                ::log::debug!("../src/load_layout.c:json_end_map:110 - sanity check: removing swallows specification from split container\n");
                while !(*self.json_node).swallow_head.tqh_first.is_null() {
                    let match_0: *mut Match = (*self.json_node).swallow_head.tqh_first;
                    if !(*match_0).matches.tqe_next.is_null() {
                        (*(*match_0).matches.tqe_next).matches.tqe_prev =
                            (*match_0).matches.tqe_prev;
                    } else {
                        (*self.json_node).swallow_head.tqh_last = (*match_0).matches.tqe_prev;
                    }
                    *(*match_0).matches.tqe_prev = (*match_0).matches.tqe_next;
                    drop(Box::from_raw(match_0));
                }
            }
            if ((*self.json_node).type_0) == ContainerType::Workspace {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_owner_1 = (*self.json_node).name.clone();
                    let log_value_0 =
                        i3_common::logging::string_bytes(log_owner_1.as_deref(), None);
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                        ::log::debug!("../src/load_layout.c:json_end_map:121 - Attaching workspace. name = {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ "../src/load_layout.c:json_end_map:121 - Attaching workspace. name = ".as_bytes(), log_value_0, "\n".as_bytes() ]);
                    }
                };
                if (*self.json_node).name.is_none()
                    || strcmp((*self.json_node).name_ptr(), c"".as_ptr()) == 0 as ::core::ffi::c_int
                {
                    (*self.json_node).name = Some(c"unnamed".to_owned());
                }
                let base = (*self.json_node)
                    .name
                    .clone()
                    .expect("layout workspace name is initialized");
                let mut cnt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while !get_existing_workspace_by_name((*self.json_node).name_ptr()).is_null() {
                    let fresh1 = cnt;
                    cnt += 1;
                    let mut name = base.to_bytes().to_vec();
                    name.extend_from_slice(format!("_{fresh1}").as_bytes());
                    (*self.json_node).name = Some(CString::new(name).unwrap());
                }
                (*self.json_node).num =
                    ws_name_to_number(std::ffi::CStr::from_ptr((*self.json_node).name_ptr()))
                        .unwrap_or(-1);
            }
            if ((*self.json_node).type_0) == ContainerType::Floating {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = i3_common::logging::pointer(parent_ptr(self.json_node));
                    let log_owner_2 = (*parent_ptr(self.json_node)).name.clone();
                    let log_value_1 =
                        i3_common::logging::string_bytes(log_owner_2.as_deref(), None);
                    if let (Ok(log_value_1_text),) = (std::str::from_utf8(log_value_1),) {
                        ::log::debug!("../src/load_layout.c:json_end_map:148 - fixing parent which currently is {} / {}\n", log_value_0, log_value_1_text);
                    } else {
                        i3_common::logging::emit_parts(i3_common::logging::Level::Debug, &[ format!("../src/load_layout.c:json_end_map:148 - fixing parent which currently is {} / ", log_value_0).as_bytes(), log_value_1, "\n".as_bytes() ]);
                    }
                };
                set_parent_ptr(
                    self.json_node,
                    con_get_workspace(parent_ptr(self.json_node)),
                )
                .expect("live acyclic ancestry");
                if rect_equals(
                    (*self.json_node).rect as Rect,
                    Rect {
                        x: 0_u32,
                        y: 0_u32,
                        width: 0_u32,
                        height: 0_u32,
                    },
                ) {
                    ::log::debug!("../src/load_layout.c:json_end_map:155 - Geometry not set, combining children\n");
                    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
                    child = nodes::first_ptr(self.json_node);
                    while !child.is_null() {
                        ::log::debug!(
                            "../src/load_layout.c:json_end_map:158 - child geometry: {} x {}\n",
                            ((*child).geometry.width) as i32,
                            ((*child).geometry.height) as i32
                        );
                        (*self.json_node).rect.width = (*self.json_node)
                            .rect
                            .width
                            .wrapping_add((*child).geometry.width);
                        (*self.json_node).rect.height = max(
                            (*self.json_node).rect.height as ::core::ffi::c_int,
                            (*child).geometry.height as ::core::ffi::c_int,
                        ) as u32;
                        child = nodes::next_ptr(child);
                    }
                }
                floating_check_size(self.json_node, false);
            }
            for (mark, con) in std::mem::take(&mut self.pending_marks) {
                con_mark(con, mark.as_ptr(), MarkMode::Add);
            }
            ::log::info!("attaching\n");
            con_attach(self.json_node, parent_ptr(self.json_node), true);
            ::log::info!("Creating window\n");
            x_con_init(self.json_node);
            let old_floating_mode: FloatingState = (*self.json_node).floating;
            if (old_floating_mode).is_floating()
                && ((*parent_ptr(self.json_node)).type_0) != ContainerType::Floating
            {
                ::log::info!("Fixing floating node without CT_FLOATING_CON parent\n");
                (*self.json_node).floating = FloatingState::AutoOff;
                floating_enable(self.json_node, false);
                (*self.json_node).floating = old_floating_mode;
            }
            self.json_node = parent_ptr(self.json_node);
            self.incomplete -= 1;
            ::log::debug!(
                "../src/load_layout.c:json_end_map:198 - incomplete = {}\n",
                { (self.incomplete) }
            );
        }
        if self.parsing_swallows as ::core::ffi::c_int != 0
            && self.swallow_is_empty as ::core::ffi::c_int != 0
        {
            ::log::error!("ERROR: Layout file is invalid: found an empty swallow definition.\n");
            return 0 as ::core::ffi::c_int;
        }
        self.parsing_gaps = false;
        self.parsing_rect = false;
        self.parsing_actual_deco_rect = false;
        self.parsing_deco_rect = false;
        self.parsing_window_rect = false;
        self.parsing_geometry = false;
        1 as ::core::ffi::c_int
    }
    unsafe fn json_end_array(&mut self) -> ::core::ffi::c_int {
        ::log::info!("end of array\n");
        if !self.parsing_swallows && !self.parsing_focus && !self.parsing_marks {
            con_fix_percent(self.json_node);
        }
        if self.parsing_swallows {
            self.parsing_swallows = false;
        }
        if self.parsing_marks {
            self.parsing_marks = false;
        }
        if self.parsing_focus {
            let owner = identity(self.json_node).expect("live parsed focus owner");
            for old_id in std::mem::take(&mut self.focus_mappings).into_iter().rev() {
                for id in focus::children(owner) {
                    let Some(con) = resolve(id) else { continue };
                    if (*con).old_id == old_id {
                        ::log::info!("got it! {}\n", i3_common::logging::pointer(con));
                        focus::move_to_front(owner, id).expect("existing parsed focus member");
                        break;
                    }
                }
            }
            self.parsing_focus = false;
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_key(
        &mut self,
        val: *const ::core::ffi::c_uchar,
        len: usize,
    ) -> ::core::ffi::c_int {
        let logged_key = property_string(if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(val, len)
        });
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_precision_0 = (len as ::core::ffi::c_int);
            let log_value_0 = i3_common::logging::string_bytes(
                Some(logged_key.as_c_str()),
                usize::try_from(log_precision_0).ok(),
            );
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(log_value_0),) {
                ::log::info!("key: {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &["key: ".as_bytes(), log_value_0, "\n".as_bytes()],
                );
            }
        };
        self.last_key = logged_key;
        if strcasecmp(self.last_key.as_ptr(), c"swallows".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_swallows = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"gaps".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_gaps = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"rect".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_rect = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"actual_deco_rect".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            self.parsing_actual_deco_rect = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"deco_rect".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_deco_rect = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"window_rect".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_window_rect = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"geometry".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_geometry = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"focus".as_ptr()) == 0 as ::core::ffi::c_int {
            self.parsing_focus = true;
        }
        if strcasecmp(self.last_key.as_ptr(), c"marks".as_ptr()) == 0 as ::core::ffi::c_int {
            self.pending_marks.clear();
            self.parsing_marks = true;
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_string(
        &mut self,
        val: *const ::core::ffi::c_uchar,
        len: usize,
    ) -> ::core::ffi::c_int {
        let logged_string = property_string(if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(val, len)
        });
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_precision_0 = (len as ::core::ffi::c_int);
            let log_value_0 = i3_common::logging::string_bytes(
                Some(logged_string.as_c_str()),
                usize::try_from(log_precision_0).ok(),
            );
            let log_value_1 = {
                let pointer: *const libc::c_char = self.last_key.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text), Ok(log_value_1_text)) = (
                std::str::from_utf8(log_value_0),
                std::str::from_utf8(&log_value_1),
            ) {
                ::log::info!(
                    "string: {} for key {}\n",
                    log_value_0_text,
                    log_value_1_text
                );
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        "string: ".as_bytes(),
                        log_value_0,
                        " for key ".as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if self.parsing_swallows {
            let owned_sval = property_string(std::slice::from_raw_parts(val, len));
            let sval = owned_sval.as_ptr();
            if strcasecmp(self.last_key.as_ptr(), c"class".as_ptr()) == 0 as ::core::ffi::c_int {
                let new = compile_or_report(owned_sval.as_c_str());

                (*self.current_swallow).class = new;
                self.swallow_is_empty = false;
            } else if strcasecmp(self.last_key.as_ptr(), c"instance".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                let new = compile_or_report(owned_sval.as_c_str());

                (*self.current_swallow).instance = new;
                self.swallow_is_empty = false;
            } else if strcasecmp(self.last_key.as_ptr(), c"window_role".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                let new = compile_or_report(owned_sval.as_c_str());

                (*self.current_swallow).window_role = new;
                self.swallow_is_empty = false;
            } else if strcasecmp(self.last_key.as_ptr(), c"title".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                let new = compile_or_report(owned_sval.as_c_str());

                (*self.current_swallow).title = new;
                self.swallow_is_empty = false;
            } else if strcasecmp(self.last_key.as_ptr(), c"machine".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                let new = compile_or_report(owned_sval.as_c_str());

                (*self.current_swallow).machine = new;
                self.swallow_is_empty = false;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = self.last_key.as_ptr();
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::error!("ERROR: swallow key {} unknown\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Error,
                            &[
                                "ERROR: swallow key ".as_bytes(),
                                &log_value_0,
                                " unknown\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if self.parsing_marks {
            self.pending_marks.push((
                property_string(std::slice::from_raw_parts(val, len)),
                self.json_node,
            ));
        } else if strcasecmp(self.last_key.as_ptr(), c"name".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).name = Some(property_string(std::slice::from_raw_parts(val, len)));
        } else if strcasecmp(self.last_key.as_ptr(), c"title_format".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            (*self.json_node).title_format =
                Some(property_string(std::slice::from_raw_parts(val, len)));
        } else if strcasecmp(self.last_key.as_ptr(), c"sticky_group".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let group = property_string(std::slice::from_raw_parts(val, len));
            (*self.json_node).sticky_group = Some(group.clone());
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = {
                    let pointer: *const libc::c_char = group.as_ptr();
                    i3_common::logging::string_bytes(
                        if pointer.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(pointer))
                        },
                        None,
                    )
                    .to_vec()
                };
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::info!("sticky_group of this container is {}\n", log_value_0_text);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Verbose,
                        &[
                            "sticky_group of this container is ".as_bytes(),
                            &log_value_0,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
        } else if strcasecmp(self.last_key.as_ptr(), c"orientation".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let owned_buf = property_string(std::slice::from_raw_parts(val, len));
            let buf = owned_buf.as_ptr();
            if strcasecmp(buf, c"none".as_ptr()) == 0 as ::core::ffi::c_int
                || strcasecmp(buf, c"horizontal".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*self.json_node).last_split_layout = Layout::SplitH;
            } else if strcasecmp(buf, c"vertical".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).last_split_layout = Layout::SplitV;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled orientation: {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled orientation: ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"border".as_ptr()) == 0 as ::core::ffi::c_int
        {
            let owned_buf_0 = property_string(std::slice::from_raw_parts(val, len));
            let buf_0 = owned_buf_0.as_ptr();
            if strcasecmp(buf_0, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).border_style = BorderStyle::None;
                (*self.json_node).max_user_border_style = (*self.json_node).border_style;
            } else if strcasecmp(buf_0, c"1pixel".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).border_style = BorderStyle::Pixel;
                (*self.json_node).max_user_border_style = (*self.json_node).border_style;
                (*self.json_node).current_border_width = 1 as ::core::ffi::c_int;
            } else if strcasecmp(buf_0, c"pixel".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).border_style = BorderStyle::Pixel;
                (*self.json_node).max_user_border_style = (*self.json_node).border_style;
            } else if strcasecmp(buf_0, c"normal".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).border_style = BorderStyle::Normal;
                (*self.json_node).max_user_border_style = (*self.json_node).border_style;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf_0;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled \"border\": {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled \"border\": ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"type".as_ptr()) == 0 as ::core::ffi::c_int {
            let owned_buf_1 = property_string(std::slice::from_raw_parts(val, len));
            let buf_1 = owned_buf_1.as_ptr();
            if strcasecmp(buf_1, c"root".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::Root;
            } else if strcasecmp(buf_1, c"output".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::Output;
            } else if strcasecmp(buf_1, c"con".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::Con;
            } else if strcasecmp(buf_1, c"floating_con".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::Floating;
            } else if strcasecmp(buf_1, c"workspace".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::Workspace;
            } else if strcasecmp(buf_1, c"dockarea".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).type_0 = ContainerType::DockArea;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf_1;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled \"type\": {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled \"type\": ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"layout".as_ptr()) == 0 as ::core::ffi::c_int
        {
            let owned_buf_2 = property_string(std::slice::from_raw_parts(val, len));
            let buf_2 = owned_buf_2.as_ptr();
            if strcasecmp(buf_2, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = (*self.json_node).last_split_layout;
            } else if strcasecmp(buf_2, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::Stacked;
            } else if strcasecmp(buf_2, c"tabbed".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::Tabbed;
            } else if strcasecmp(buf_2, c"dockarea".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::DockArea;
            } else if strcasecmp(buf_2, c"output".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::Output;
            } else if strcasecmp(buf_2, c"splith".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::SplitH;
            } else if strcasecmp(buf_2, c"splitv".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).layout = Layout::SplitV;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf_2;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled \"layout\": {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled \"layout\": ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"workspace_layout".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let owned_buf_3 = property_string(std::slice::from_raw_parts(val, len));
            let buf_3 = owned_buf_3.as_ptr();
            if strcasecmp(buf_3, c"default".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).workspace_layout = Layout::Default;
            } else if strcasecmp(buf_3, c"stacked".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).workspace_layout = Layout::Stacked;
            } else if strcasecmp(buf_3, c"tabbed".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).workspace_layout = Layout::Tabbed;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf_3;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled \"workspace_layout\": {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled \"workspace_layout\": ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"last_split_layout".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let owned_buf_4 = property_string(std::slice::from_raw_parts(val, len));
            let buf_4 = owned_buf_4.as_ptr();
            if strcasecmp(buf_4, c"splith".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).last_split_layout = Layout::SplitH;
            } else if strcasecmp(buf_4, c"splitv".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).last_split_layout = Layout::SplitV;
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = buf_4;
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::info!("Unhandled \"last_splitlayout\": {}\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Verbose,
                            &[
                                "Unhandled \"last_splitlayout\": ".as_bytes(),
                                &log_value_0,
                                "\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"mark".as_ptr()) == 0 as ::core::ffi::c_int {
            ::log::debug!(
                "../src/load_layout.c:json_string:443 - Found deprecated key \"mark\".\n"
            );
            let owned_buf_5 = property_string(std::slice::from_raw_parts(val, len));
            let buf_5 = owned_buf_5.as_ptr();
            con_mark(self.json_node, buf_5, MarkMode::Replace);
        } else if strcasecmp(self.last_key.as_ptr(), c"floating".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let owned_buf_6 = property_string(std::slice::from_raw_parts(val, len));
            let buf_6 = owned_buf_6.as_ptr();
            if strcasecmp(buf_6, c"auto_off".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).floating = FloatingState::AutoOff;
            } else if strcasecmp(buf_6, c"auto_on".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).floating = FloatingState::AutoOn;
            } else if strcasecmp(buf_6, c"user_off".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).floating = FloatingState::UserOff;
            } else if strcasecmp(buf_6, c"user_on".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).floating = FloatingState::UserOn;
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"scratchpad_state".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            let owned_buf_7 = property_string(std::slice::from_raw_parts(val, len));
            let buf_7 = owned_buf_7.as_ptr();
            if strcasecmp(buf_7, c"none".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).scratchpad_state = ScratchpadState::None;
            } else if strcasecmp(buf_7, c"fresh".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).scratchpad_state = ScratchpadState::Fresh;
            } else if strcasecmp(buf_7, c"changed".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).scratchpad_state = ScratchpadState::Changed;
            }
        } else if strcasecmp(self.last_key.as_ptr(), c"previous_workspace_name".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            free(previous_workspace_name as *mut ::core::ffi::c_void);
            previous_workspace_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
            previous_workspace_name = sstrndup(val as *const ::core::ffi::c_char, len);
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_int(&mut self, val: ::core::ffi::c_longlong) -> ::core::ffi::c_int {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = (val) as libc::c_longlong;
            let log_value_1 = {
                let pointer: *const libc::c_char = self.last_key.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::info!("int {} for key {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        format!("int {} for key ", log_value_0).as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if strcasecmp(self.last_key.as_ptr(), c"type".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).type_0 = ContainerType::from_raw(val as u32);
        }
        if strcasecmp(self.last_key.as_ptr(), c"fullscreen_mode".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            (*self.json_node).fullscreen_mode = FullscreenMode::from_raw(val as u32);
        }
        if strcasecmp(self.last_key.as_ptr(), c"num".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).num = val as ::core::ffi::c_int;
        }
        if strcasecmp(self.last_key.as_ptr(), c"current_border_width".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            (*self.json_node).current_border_width = val as ::core::ffi::c_int;
        }
        if strcasecmp(self.last_key.as_ptr(), c"window_icon_padding".as_ptr())
            == 0 as ::core::ffi::c_int
        {
            (*self.json_node).window_icon_padding = val as ::core::ffi::c_int;
        }
        if strcasecmp(self.last_key.as_ptr(), c"depth".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).depth = val as u16;
        }
        if !self.parsing_swallows
            && strcasecmp(self.last_key.as_ptr(), c"id".as_ptr()) == 0 as ::core::ffi::c_int
        {
            (*self.json_node).old_id = val as ::core::ffi::c_int;
        }
        if self.parsing_focus {
            self.focus_mappings.push(val as i32);
        }
        if self.parsing_rect as ::core::ffi::c_int != 0
            || self.parsing_window_rect as ::core::ffi::c_int != 0
            || self.parsing_geometry as ::core::ffi::c_int != 0
        {
            let mut r: *mut Rect = ::core::ptr::null_mut::<Rect>();
            if self.parsing_rect {
                r = &raw mut (*self.json_node).rect;
            } else if self.parsing_window_rect {
                r = &raw mut (*self.json_node).window_rect;
            } else {
                r = &raw mut (*self.json_node).geometry;
            }
            if let Some(rect) = rect_value(*r, &self.last_key, val) {
                r.write(rect);
            } else {
                {
                    // Snapshot native bytes before the logger can dispatch a callback.
                    let log_value_0 = {
                        let pointer: *const libc::c_char = self.last_key.as_ptr();
                        i3_common::logging::string_bytes(
                            if pointer.is_null() {
                                None
                            } else {
                                Some(std::ffi::CStr::from_ptr(pointer))
                            },
                            None,
                        )
                        .to_vec()
                    };
                    if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                        ::log::error!("ERROR: WARNING: unknown key {} in rect\n", log_value_0_text);
                    } else {
                        i3_common::logging::emit_parts(
                            i3_common::logging::Level::Error,
                            &[
                                "ERROR: WARNING: unknown key ".as_bytes(),
                                &log_value_0,
                                " in rect\n".as_bytes(),
                            ],
                        );
                    }
                };
            }
            ::log::debug!(
                "../src/load_layout.c:json_int:539 - rect now: ({}, {}, {}, {})\n",
                ((*r).x) as i32,
                ((*r).y) as i32,
                ((*r).width) as i32,
                ((*r).height) as i32
            );
        }
        if self.parsing_swallows {
            if strcasecmp(self.last_key.as_ptr(), c"id".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.current_swallow).id = val as u32;
                self.swallow_is_empty = false;
            }
            if strcasecmp(self.last_key.as_ptr(), c"dock".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.current_swallow).dock = DockMatch::from_raw(val as i32);
                self.swallow_is_empty = false;
            }
            if strcasecmp(self.last_key.as_ptr(), c"insert_where".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                (*self.current_swallow).insert_where = MatchInsertWhere::from_raw(val as u32);
                self.swallow_is_empty = false;
            }
        }
        if self.parsing_gaps {
            if strcasecmp(self.last_key.as_ptr(), c"inner".as_ptr()) == 0 as ::core::ffi::c_int {
                (*self.json_node).gaps.inner = val as ::core::ffi::c_int;
            } else if strcasecmp(self.last_key.as_ptr(), c"top".as_ptr()) == 0 as ::core::ffi::c_int
            {
                (*self.json_node).gaps.top = val as ::core::ffi::c_int;
            } else if strcasecmp(self.last_key.as_ptr(), c"right".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                (*self.json_node).gaps.right = val as ::core::ffi::c_int;
            } else if strcasecmp(self.last_key.as_ptr(), c"bottom".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                (*self.json_node).gaps.bottom = val as ::core::ffi::c_int;
            } else if strcasecmp(self.last_key.as_ptr(), c"left".as_ptr())
                == 0 as ::core::ffi::c_int
            {
                (*self.json_node).gaps.left = val as ::core::ffi::c_int;
            }
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_bool(&mut self, val: ::core::ffi::c_int) -> ::core::ffi::c_int {
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = (val);
            let log_value_1 = {
                let pointer: *const libc::c_char = self.last_key.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_1_text),) = (std::str::from_utf8(&log_value_1),) {
                ::log::info!("bool {} for key {}\n", log_value_0, log_value_1_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Verbose,
                    &[
                        format!("bool {} for key ", log_value_0).as_bytes(),
                        &log_value_1,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if strcasecmp(self.last_key.as_ptr(), c"focused".as_ptr()) == 0 as ::core::ffi::c_int
            && val != 0
        {
            self.to_focus = self.json_node;
        }
        if strcasecmp(self.last_key.as_ptr(), c"sticky".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).sticky = val != 0;
        }
        if self.parsing_swallows
            && strcasecmp(self.last_key.as_ptr(), c"restart_mode".as_ptr())
                == 0 as ::core::ffi::c_int
        {
            (*self.current_swallow).restart_mode = val != 0;
            self.swallow_is_empty = false;
        }
        1 as ::core::ffi::c_int
    }
    unsafe fn json_double(&mut self, val: ::core::ffi::c_double) -> ::core::ffi::c_int {
        let value = i3_common::logging::fixed_decimal(val, i3_common::logging::FixedPrecision::Six);
        let key = self.last_key.clone();
        if let (Ok(value), Ok(key)) = (std::str::from_utf8(&value), key.to_str()) {
            ::log::info!("double {} for key {}\n", value, key);
        } else {
            i3_common::logging::emit_parts(
                i3_common::logging::Level::Verbose,
                &[b"double ", &value, b" for key ", key.to_bytes(), b"\n"],
            );
        }
        if strcasecmp(self.last_key.as_ptr(), c"percent".as_ptr()) == 0 as ::core::ffi::c_int {
            (*self.json_node).percent = val;
        }
        1 as ::core::ffi::c_int
    }
}
struct LayoutEvents {
    state: Rc<RefCell<LayoutState>>,
    error: Option<String>,
}
impl EventHandler for LayoutEvents {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut state = self.state.borrow_mut();
        // Private handler construction is restricted to tree_append_json's
        // explicit graph contract. Only this parse owns this state; reentrant
        // parses get different owners, never a shared global parser context.
        unsafe {
            match event {
                Event::StartMap => state.json_start_map() != 0,
                Event::EndMap => state.json_end_map() != 0,
                Event::EndArray => state.json_end_array() != 0,
                Event::String(_) | Event::Number(_) | Event::Boolean(_)
                    if state.parsing_swallows && state.current_swallow.is_null() =>
                {
                    self.error = Some("parse error: swallow value outside a map".into());
                    false
                }
                Event::MapKey(bytes) => state.json_key(bytes.as_ptr(), bytes.len()) != 0,
                Event::String(bytes) => state.json_string(bytes.as_ptr(), bytes.len()) != 0,
                Event::Boolean(value) => state.json_bool(i32::from(value)) != 0,
                Event::Number(number) => {
                    if number.contains(['.', 'e', 'E']) {
                        if let Ok(value) = number.parse::<f64>() {
                            if value.is_finite() {
                                return state.json_double(value) != 0;
                            }
                        }
                        self.error = Some("parse error: numeric (floating point) overflow".into());
                        false
                    } else {
                        if let Ok(value) = number.parse::<i64>() {
                            if value != i64::MIN {
                                return state.json_int(value) != 0;
                            }
                        }
                        self.error = Some("parse error: integer overflow".into());
                        false
                    }
                }
                _ => true,
            }
        }
    }
    fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
fn layout_parser(handler: impl EventHandler + 'static, invalid_utf8: bool) -> Parser {
    let mut parser = Parser::new(handler);
    let options = parser.options_mut();
    options.allow_comments = true;
    options.allow_multiple_values = true;
    options.allow_legacy_whitespace = true;
    options.allow_invalid_utf8 = invalid_utf8;
    parser
}
fn validate_bytes(bytes: &[u8]) -> Result<(), ParseError> {
    let mut parser = layout_parser(|_: Event<'_>| true, false);
    let result = parser.parse(bytes);
    let _ = parser.finish();
    result
}
#[derive(Debug)]
struct ContentState {
    key: CString,
    level: i32,
    result: JsonContent,
}
fn content_result(bytes: &[u8]) -> (JsonContent, Option<ParseError>) {
    let state = Rc::new(RefCell::new(ContentState {
        key: CString::default(),
        level: 0,
        result: JsonContent::Container,
    }));
    let output = state.clone();
    let mut parser = layout_parser(
        move |event: Event<'_>| {
            let mut state = output.borrow_mut();
            match event {
                Event::StartMap | Event::StartArray => state.level += 1,
                Event::EndMap | Event::EndArray => state.level -= 1,
                Event::MapKey(bytes) => state.key = property_string(bytes),
                Event::String(value)
                    if state.level <= 1
                        && unsafe { strcasecmp(state.key.as_ptr(), c"type".as_ptr()) } == 0 =>
                {
                    if unsafe {
                        strncasecmp(value.as_ptr().cast(), c"workspace".as_ptr(), value.len())
                    } == 0
                    {
                        state.result = JsonContent::Workspace;
                    }
                    return false;
                }
                _ => {}
            }
            true
        },
        false,
    );
    let parsed = parser.parse(bytes);
    let _ = parser.finish();
    drop(parser);
    let result = state.borrow().result;
    (
        result,
        parsed
            .err()
            .filter(|error| error.kind != i3_common::json_stream::ParseErrorKind::Canceled),
    )
}
fn determine_content(bytes: &[u8]) -> JsonContent {
    content_result(bytes).0
}
/// buf is a readable initialized byte span. Legacy validation intentionally
/// ignores finish errors, accepting syntactically incomplete layout fragments.
pub unsafe fn json_validate(buf: *const libc::c_char, len: usize) -> bool {
    let bytes = if len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(buf.cast(), len)
    };
    match validate_bytes(bytes) {
        Ok(()) => true,
        Err(error) => {
            let text = CString::new(format!("{error}\n")).unwrap();
            {
                // Snapshot native bytes before the logger can dispatch a callback.
                let log_value_0 = {
                    let pointer: *const libc::c_char = text.as_ptr();
                    i3_common::logging::string_bytes(
                        if pointer.is_null() {
                            None
                        } else {
                            Some(std::ffi::CStr::from_ptr(pointer))
                        },
                        None,
                    )
                    .to_vec()
                };
                if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                    ::log::error!("ERROR: JSON parsing error: {}\n", log_value_0_text);
                } else {
                    i3_common::logging::emit_parts(
                        i3_common::logging::Level::Error,
                        &[
                            "ERROR: JSON parsing error: ".as_bytes(),
                            &log_value_0,
                            "\n".as_bytes(),
                        ],
                    );
                }
            };
            false
        }
    }
}
pub unsafe fn json_determine_content(buf: *const libc::c_char, len: usize) -> JsonContent {
    let (content, error) = content_result(if len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(buf.cast(), len)
    });
    if let Some(error) = error {
        let text = CString::new(format!("{error}\n")).unwrap();
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = text.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: JSON parsing error: {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: JSON parsing error: ".as_bytes(),
                        &log_value_0,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
    }
    content
}
struct NumericLocale(CString);
impl NumericLocale {
    /// Process-global locale users must be serialized for this native scope.
    unsafe fn c() -> Self {
        let old = CStr::from_ptr(setlocale(libc::LC_NUMERIC, std::ptr::null())).to_owned();
        setlocale(libc::LC_NUMERIC, c"C".as_ptr());
        Self(old)
    }
}
impl Drop for NumericLocale {
    fn drop(&mut self) {
        unsafe {
            setlocale(libc::LC_NUMERIC, self.0.as_ptr());
        }
    }
}
/// Native graph/locale entry point. Completed containers stay attached on parse
/// error; only incomplete containers are freed, exactly as the legacy loader.
pub unsafe fn tree_append_json(
    con: *mut Con,
    buf: *const libc::c_char,
    len: usize,
    errormsg: *mut *mut libc::c_char,
) {
    let bytes = if len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(buf.cast(), len)
    };
    let state = Rc::new(RefCell::new(LayoutState::new(con)));
    let mut parser = layout_parser(
        LayoutEvents {
            state: state.clone(),
            error: None,
        },
        true,
    );
    let locale = NumericLocale::c();
    if let Err(error) = parser.parse(bytes) {
        let message = error.to_string();
        let message = CString::new(format!("{message}\n")).unwrap();
        {
            // Snapshot native bytes before the logger can dispatch a callback.
            let log_value_0 = {
                let pointer: *const libc::c_char = message.as_ptr();
                i3_common::logging::string_bytes(
                    if pointer.is_null() {
                        None
                    } else {
                        Some(std::ffi::CStr::from_ptr(pointer))
                    },
                    None,
                )
                .to_vec()
            };
            if let (Ok(log_value_0_text),) = (std::str::from_utf8(&log_value_0),) {
                ::log::error!("ERROR: JSON parsing error: {}\n", log_value_0_text);
            } else {
                i3_common::logging::emit_parts(
                    i3_common::logging::Level::Error,
                    &[
                        "ERROR: JSON parsing error: ".as_bytes(),
                        &log_value_0,
                        "\n".as_bytes(),
                    ],
                );
            }
        };
        if !errormsg.is_null() {
            *errormsg = sstrdup(message.as_ptr());
        }
        loop {
            let node = {
                let mut state = state.borrow_mut();
                if state.incomplete <= 0 {
                    break;
                }
                state.incomplete -= 1;
                let node = state.json_node;
                state.json_node = parent_ptr(node);
                if node == state.to_focus {
                    state.to_focus = std::ptr::null_mut();
                }
                node
            };
            con_free(node);
        }
    }
    con_fix_percent(con);
    drop(locale);
    // finish may emit a pending final scalar; ignore its status but keep those
    // callback side effects, matching the old complete_parse ordering.
    let _ = parser.finish();
    drop(parser);
    let to_focus = state.borrow().to_focus;
    if !to_focus.is_null() {
        con_activate(to_focus);
    }
}
// Internal helpers use Rust linkage rather than foreign symbol lookup.
use crate::util::max;
use crate::util::ws_name_to_number;
use i3_common::safewrappers::scalloc;
use i3_common::safewrappers::smalloc;
use i3_common::safewrappers::srealloc;
use i3_common::safewrappers::sstrdup;
use i3_common::safewrappers::sstrndup;

// Canonical Cairo backend ABI; renderer logic remains local.

pub use libc::{free, memcpy, setlocale, strcasecmp, strcmp, strncasecmp};

// Shared model types retain field layout while using one Rust identity.
pub use crate::types::Con;
pub use crate::types::Match;
pub use crate::types::Rect;
pub use crate::types::Window;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_activate;
use crate::con::con_attach;
use crate::con::con_fix_percent;
use crate::con::con_free;
use crate::con::con_get_workspace;
use crate::con::con_is_split;
use crate::con::con_mark;
use crate::con::con_new_skeleton;
use crate::floating::floating_check_size;
use crate::floating::floating_enable;
use crate::util::rect_equals;
use crate::workspace::get_existing_workspace_by_name;
use crate::workspace::previous_workspace_name;
use crate::x::x_con_init;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn callback_logging_matches_native_bounded_bytes_and_fixed_decimals() {
        const CHILD: &str = "I333_LAYOUT_LOG_NATIVE_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let child = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "load_layout::tests::callback_logging_matches_native_bounded_bytes_and_fixed_decimals", "--nocapture"])
                .env(CHILD, "1")
                .output().unwrap();
            assert!(
                child.status.success(),
                "{}",
                String::from_utf8_lossy(&child.stderr)
            );
            return;
        }
        // A child isolates the process-wide log sink from other unit tests. The
        // callbacks below exercise their real formatting and parser state, with
        // libc printf as the independent byte oracle (no X server required).
        static MESSAGES: std::sync::Mutex<Vec<Vec<u8>>> = std::sync::Mutex::new(Vec::new());
        fn capture(_: i3_common::logging::Level, message: &[u8]) {
            MESSAGES.lock().unwrap().push(message.to_vec());
        }
        fn take() -> Vec<u8> {
            let mut messages = MESSAGES.lock().unwrap();
            assert_eq!(messages.len(), 1);
            messages.pop().unwrap()
        }
        i3_common::logging::set_sink(capture);
        let mut node = Con::empty();
        let mut state = unsafe { LayoutState::new(&raw mut node) };
        for bytes in [
            b"".as_slice(),
            b"unterminated",
            b"nul\0ignored",
            b"bad-\xff",
        ] {
            let pointer = if bytes.is_empty() {
                std::ptr::null()
            } else {
                bytes.as_ptr()
            };
            let mut native = [0u8; 1024];
            let count = unsafe {
                libc::snprintf(
                    native.as_mut_ptr().cast(),
                    native.len(),
                    c"key: %.*s\n".as_ptr(),
                    bytes.len() as i32,
                    pointer,
                )
            };
            assert!(count >= 0 && (count as usize) < native.len());
            assert_eq!(unsafe { state.json_key(pointer, bytes.len()) }, 1);
            assert_eq!(take(), &native[..count as usize]);
            state.last_key = c"unknown-\xff".to_owned();
            let count = unsafe {
                libc::snprintf(
                    native.as_mut_ptr().cast(),
                    native.len(),
                    c"string: %.*s for key %s\n".as_ptr(),
                    bytes.len() as i32,
                    pointer,
                    state.last_key.as_ptr(),
                )
            };
            assert!(count >= 0 && (count as usize) < native.len());
            assert_eq!(unsafe { state.json_string(pointer, bytes.len()) }, 1);
            assert_eq!(take(), &native[..count as usize]);
        }
        for value in [
            0.0,
            -0.0,
            1.2345675,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
        ] {
            let mut native = [0u8; 1024];
            let count = unsafe {
                libc::snprintf(
                    native.as_mut_ptr().cast(),
                    native.len(),
                    c"double %f for key %s\n".as_ptr(),
                    value,
                    state.last_key.as_ptr(),
                )
            };
            assert!(count >= 0 && (count as usize) < native.len());
            assert_eq!(unsafe { state.json_double(value) }, 1);
            assert_eq!(take(), &native[..count as usize]);
        }
    }

    #[test]
    fn fragment_validation_keeps_parse_only_success_and_errors() {
        for bytes in [
            b"{".as_slice(),
            b"[1,",
            b"/* comment */{} {}",
            b"{\"x\":1",
            b"[\x0b\x0c]",
        ] {
            assert!(validate_bytes(bytes).is_ok(), "{bytes:?}");
        }
        for bytes in [b"[1,]".as_slice(), b"{\"x\" 1}", b"\"\xff\""] {
            assert!(validate_bytes(bytes).is_err(), "{bytes:?}");
        }
    }
    #[test]
    fn content_detection_preserves_shallow_cancellation_and_prefix_comparison() {
        assert_eq!(
            determine_content(br#"{"type":"work"}"#),
            JsonContent::Workspace
        );
        assert_eq!(determine_content(br#"{"type":""}"#), JsonContent::Workspace);
        assert_eq!(
            determine_content(br#"{"type":"workspace\u0000suffix"}"#),
            JsonContent::Workspace
        );
        assert_eq!(
            determine_content(br#"{"type":"con","type":"workspace"}"#),
            JsonContent::Container
        );
        assert_eq!(
            determine_content(br#"{"nodes":[{"type":"workspace"}]}"#),
            JsonContent::Container
        );
        assert_eq!(
            determine_content(br#"{"TYPE\u0000ignored":"WORKSPACE"}"#),
            JsonContent::Workspace
        );
    }
    #[test]
    fn property_and_rect_values_are_bounded_owned_and_keep_narrowing() {
        assert_eq!(property_string(b"\xff\0suffix").to_bytes(), b"\xff");
        let rect = Rect {
            x: 1,
            y: 2,
            width: 3,
            height: 4,
        };
        let updated = rect_value(rect, c"WIDTH", -1).unwrap();
        assert_eq!(updated.width, u32::MAX);
        assert_eq!(updated.x, 1);
        assert!(rect_value(rect, c"unknown", 1).is_none());
        assert_eq!(rect_value(rect, c"x", 0x1_0000_0001).unwrap().x, 1);
    }
}
