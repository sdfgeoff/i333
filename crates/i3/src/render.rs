use crate::con::arena::focus;
use crate::con::arena::{floating, identity, resolve};
use crate::types::BorderStyle;
use crate::types::Direction;
use crate::types::Layout;
use crate::types::PopupDuringFullscreen;
/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * render.c: Renders (determines position/sizes) the layout tree, updating the
 *           various rects. Needs to be pushed to X11 (see x.c) to be visible.
 *
 */
// Local policy and temporary values use owned Rust storage. Native entrypoints
// require live event-thread graph handles and a live display; callbacks may mutate
// the graph, so node references must never span those calls.
const CF_GLOBAL: u32 = 2;
const CF_OUTPUT: u32 = 1;
const CF_NONE: u32 = 0;
const CT_WORKSPACE: u32 = 4;
const CT_OUTPUT: u32 = 1;
const CT_ROOT: u32 = 0;
const CT_CON: u32 = 2;
const CT_DOCKAREA: u32 = 5;
type fullscreen_mode_t = u32;
extern "C" {
    fn lround(value: f64) -> libc::c_long;
}
// The graph is owned by the event thread. Local layout values and sizing buffers
// are ordinary Rust values; no node reference is held across recursive rendering.
struct RenderParams {
    x: i32,
    y: i32,
    deco_height: i32,
    rect: Rect,
    children: i32,
    sizes: Vec<i32>,
}

fn decoration_height(font_height: i32) -> i32 {
    font_height.wrapping_add(4).wrapping_add(font_height & 1)
}

fn split_sizes(total: i32, percentages: &[f64]) -> Vec<i32> {
    let children = percentages.len() as i32;
    if children == 0 {
        return Vec::new();
    }
    let mut sizes: Vec<i32> = percentages
        .iter()
        .map(|&percent| {
            let percentage = if percent > 0.0 {
                percent
            } else {
                1.0 / children as f64
            };
            // libm defines the platform's floating conversion (including exceptional
            // values); it has no pointer/lifetime requirements. Keep its native result.
            unsafe { lround(percentage * total as f64) as i32 }
        })
        .collect();
    let mut assigned = sizes.iter().copied().fold(0i32, i32::wrapping_add);
    assert!(
        assigned == total
            || assigned.wrapping_sub(total).unsigned_abs() <= children.wrapping_mul(2) as u32,
        "split percentages do not sum to the container size"
    );
    let increment = if assigned < total { 1 } else { -1 };
    while assigned != total {
        for size in &mut sizes {
            if assigned == total {
                break;
            }
            *size = size.wrapping_add(increment);
            assigned = assigned.wrapping_add(increment);
        }
    }
    sizes
}

fn workspace_inset(gaps: gaps_t) -> Rect {
    Rect {
        x: gaps.left as u32,
        y: gaps.top as u32,
        width: gaps.left.wrapping_add(gaps.right).wrapping_neg() as u32,
        height: gaps.top.wrapping_add(gaps.bottom).wrapping_neg() as u32,
    }
}
fn inner_inset(inner: i32, adjacent: [bool; 4]) -> Rect {
    let amounts = adjacent.map(|adjacent| if adjacent { inner / 2 } else { inner });
    Rect {
        x: amounts[0] as u32,
        y: amounts[1] as u32,
        width: amounts[2].wrapping_neg().wrapping_sub(amounts[0]) as u32,
        height: amounts[3].wrapping_neg().wrapping_sub(amounts[1]) as u32,
    }
}
fn window_geometry(rect: Rect, border: Option<Rect>, border_width: i32) -> Rect {
    let mut result = Rect {
        x: 0,
        y: 0,
        width: rect.width,
        height: rect.height,
    };
    if let Some(border) = border {
        result = rect_add(result, border);
    }
    let native_border = 2i32.wrapping_mul(border_width) as u32;
    result.width = result.width.wrapping_sub(native_border);
    result.height = result.height.wrapping_sub(native_border);
    rect_sanitize_dimensions(result)
}
fn popup_blocked(
    policy: PopupDuringFullscreen,
    has_window: bool,
    transient: bool,
    ancestor: bool,
) -> bool {
    match policy {
        PopupDuringFullscreen::LeaveFullscreen | PopupDuringFullscreen::Ignore => true,
        PopupDuringFullscreen::Smart => !has_window || !transient,
        PopupDuringFullscreen::All => ancestor,
    }
}

pub unsafe fn render_deco_height() -> i32 {
    decoration_height(config.font.height)
}

pub unsafe fn render_con(con: *mut Con) {
    let mut params: RenderParams = RenderParams {
        x: (*con).rect.x as i32,
        y: (*con).rect.y as i32,
        deco_height: 0,
        rect: (*con).rect as Rect,
        children: con_num_children(con),
        sizes: Vec::new(),
    };
    debuglog(
        c"%s:%s:%d - Rendering node %p / %s / layout %d / children %d\n"
            .as_ptr()
            .cast_mut(),
        c"../src/render.c".as_ptr(),
        c"render_con".as_ptr(),
        51,
        con,
        (*con).name,
        (*con).layout as u32,
        params.children,
    );
    if (*con).type_0 as u32 == CT_WORKSPACE {
        let gaps: gaps_t = calculate_effective_gaps(con);
        let inset = workspace_inset(gaps);
        (*con).rect = rect_add((*con).rect as Rect, inset) as Rect;
        params.rect = rect_add(params.rect, inset);
        params.x = params.x.wrapping_add(gaps.left);
        params.y = params.y.wrapping_add(gaps.top);
    }
    if gaps_should_inset_con(con, params.children) {
        let gaps_0: gaps_t = calculate_effective_gaps(con);
        let inset_0 = inner_inset(
            gaps_0.inner,
            [
                gaps_has_adjacent_container(con, Direction::Left),
                gaps_has_adjacent_container(con, Direction::Up),
                gaps_has_adjacent_container(con, Direction::Right),
                gaps_has_adjacent_container(con, Direction::Down),
            ],
        );
        if (*con).fullscreen_mode as u32 == CF_NONE {
            params.rect = rect_add(params.rect, inset_0);
            (*con).rect = rect_add((*con).rect as Rect, inset_0) as Rect;
        }
        params.x = (*con).rect.x as i32;
        params.y = (*con).rect.y as i32;
    }
    (*con).mapped = true;
    if !(*con).window.is_null() {
        let border = if (*con).fullscreen_mode as u32 == CF_NONE {
            Some(con_border_style_rect(con))
        } else {
            None
        };
        let inset_1 = window_geometry((*con).rect, border, (*con).border_width);
        (*con).window_rect = inset_1;
        debuglog(
            c"%s:%s:%d - child will be at %dx%d with size %dx%d\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_con".as_ptr(),
            123,
            inset_1.x,
            inset_1.y,
            inset_1.width,
            inset_1.height,
        );
    }
    let mut fullscreen: *mut Con = ::core::ptr::null_mut::<Con>();
    if (*con).type_0 as u32 != CT_OUTPUT {
        fullscreen = con_get_fullscreen_con(
            con,
            (if (*con).type_0 as u32 == CT_ROOT {
                CF_GLOBAL as i32
            } else {
                CF_OUTPUT as i32
            }) as fullscreen_mode_t,
        );
    }
    if !fullscreen.is_null() {
        (*fullscreen).rect = params.rect as Rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        if (*con).type_0 as u32 != CT_ROOT {
            return;
        }
    }
    params.deco_height = render_deco_height();
    params.sizes = precalculate_sizes(con, params.children);
    if (*con).layout == Layout::Output {
        if !con_is_internal(con) {
            render_output(con);
        }
    } else if (*con).type_0 as u32 == CT_ROOT {
        render_root(con, fullscreen);
    } else {
        let mut i: i32 = 0;
        let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
        child = (*con).nodes_head.tqh_first as *mut Con;
        while !child.is_null() {
            if params.children > 0 {
            } else {
                __assert_fail(
                    c"params.children > 0".as_ptr(),
                    c"../src/render.c".as_ptr(),
                    164 as u32,
                    c"void render_con(Con *)".as_ptr(),
                );
            };
            if matches!(
                (*con).layout,
                Layout::SplitH
                    | Layout::SplitV
                    | Layout::Stacked
                    | Layout::Tabbed
                    | Layout::DockArea
            ) {
                render_child_geometry(con, child, &mut params, i);
            }
            (*child).rect = rect_sanitize_dimensions((*child).rect as Rect) as Rect;
            debuglog(
                c"%s:%s:%d - child at (%d, %d) with (%d x %d)\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_con".as_ptr(),
                179,
                (*child).rect.x,
                (*child).rect.y,
                (*child).rect.width,
                (*child).rect.height,
            );
            x_raise_con(child);
            render_con(child);
            if (*con).layout == Layout::SplitH || (*con).layout == Layout::SplitV {
                if con_is_leaf(child) {
                    if (*child).border_style == BorderStyle::Normal {
                        (*child).deco_rect.width = (*child).rect.width;
                    }
                }
            }
            i += 1;
            child = (*child).nodes.tqe_next as *mut Con;
        }
        if (*con).layout == Layout::Stacked || (*con).layout == Layout::Tabbed {
            for id in focus::children_rev(identity(con).expect("live rendered focus owner")) {
                if let Some(child) = resolve(id) {
                    x_raise_con(child);
                }
            }
            child = focus::first_ptr(con);
            if !child.is_null() {
                render_con(child);
            }
            if params.children != 1 {
                x_raise_con(con);
            }
        }
    }
}
unsafe fn precalculate_sizes(con: *mut Con, children: i32) -> Vec<i32> {
    if !matches!((*con).layout, Layout::SplitH | Layout::SplitV) || children <= 0 {
        return Vec::new();
    }
    let mut percentages = Vec::with_capacity(children as usize);
    let mut child = (*con).nodes_head.tqh_first;
    while !child.is_null() {
        percentages.push((*child).percent);
        child = (*child).nodes.tqe_next;
    }
    assert_eq!(percentages.len(), children as usize);
    split_sizes(con_rect_size_in_orientation(con) as i32, &percentages)
}
unsafe fn fullscreen_blocks_floating_render(fullscreen: *mut Con, floating: *mut Con) -> bool {
    if fullscreen.is_null() {
        return false;
    }
    let policy = config.popup_during_fullscreen;
    let has_window = !(*fullscreen).window.is_null();
    let transient = policy == PopupDuringFullscreen::Smart
        && has_window
        && con_find_transient_for_window(con_descend_focused(floating), (*(*fullscreen).window).id);
    let ancestor = policy == PopupDuringFullscreen::All && con_has_parent(fullscreen, floating);
    popup_blocked(policy, has_window, transient, ancestor)
}
unsafe fn render_root(con: *mut Con, fullscreen: *mut Con) {
    let mut output: *mut Con = ::core::ptr::null_mut::<Con>();
    if fullscreen.is_null() {
        output = (*con).nodes_head.tqh_first as *mut Con;
        while !output.is_null() {
            render_con(output);
            output = (*output).nodes.tqe_next as *mut Con;
        }
    }
    debuglog(
        c"%s:%s:%d - Rendering floating windows:\n"
            .as_ptr()
            .cast_mut(),
        c"../src/render.c".as_ptr(),
        c"render_root".as_ptr(),
        285,
    );
    output = (*con).nodes_head.tqh_first as *mut Con;
    while !output.is_null() {
        if !con_is_internal(output) {
            let content: *mut Con = output_get_content(output);
            if content.is_null() || focus::first_ptr(content).is_null() {
                debuglog(
                    c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/render.c".as_ptr(),
                    c"render_root".as_ptr(),
                    293,
                );
            } else {
                let workspace: *mut Con = focus::first_ptr(content);
                let fullscreen_0: *mut Con = con_get_fullscreen_covering_ws(workspace);
                for floating_id in
                    floating::children(identity(workspace).expect("live floating parent"))
                {
                    let Some(child) = resolve(floating_id) else {
                        continue;
                    };
                    if !fullscreen_blocks_floating_render(fullscreen_0, child) {
                        debuglog(
                            c"%s:%s:%d - floating child at (%d,%d) with %d x %d\n"
                                .as_ptr()
                                .cast_mut(),
                            c"../src/render.c".as_ptr(),
                            c"render_root".as_ptr(),
                            304,
                            (*child).rect.x,
                            (*child).rect.y,
                            (*child).rect.width,
                            (*child).rect.height,
                        );
                        x_raise_con(child);
                        render_con(child);
                    }
                }
            }
        }
        output = (*output).nodes.tqe_next as *mut Con;
    }
}
unsafe fn render_output(con: *mut Con) {
    let mut child: *mut Con = ::core::ptr::null_mut::<Con>();
    let mut dockchild: *mut Con = ::core::ptr::null_mut::<Con>();
    let x: i32 = (*con).rect.x as i32;
    let mut y: i32 = (*con).rect.y as i32;
    let mut height: i32 = (*con).rect.height as i32;
    let mut content: *mut Con = ::core::ptr::null_mut::<Con>();
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as u32 == CT_CON {
            if !content.is_null() {
                debuglog(
                    c"%s:%s:%d - More than one CT_CON on output container\n"
                        .as_ptr()
                        .cast_mut(),
                    c"../src/render.c".as_ptr(),
                    c"render_output".as_ptr(),
                    329,
                );
                if 0 != 0 {
                } else {
                    __assert_fail(
                        c"false".as_ptr(),
                        c"../src/render.c".as_ptr(),
                        330 as u32,
                        c"void render_output(Con *)".as_ptr(),
                    );
                };
            }
            content = child;
        } else if (*child).type_0 as u32 != CT_DOCKAREA {
            debuglog(
                c"%s:%s:%d - Child %p of type %d is inside the OUTPUT con\n"
                    .as_ptr()
                    .cast_mut(),
                c"../src/render.c".as_ptr(),
                c"render_output".as_ptr(),
                334,
                child,
                (*child).type_0 as u32,
            );
            if 0 != 0 {
            } else {
                __assert_fail(
                    c"false".as_ptr(),
                    c"../src/render.c".as_ptr(),
                    335 as u32,
                    c"void render_output(Con *)".as_ptr(),
                );
            };
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    if content.is_null() {
        debuglog(
            c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            340,
        );
        return;
    }
    let ws: *mut Con = con_get_fullscreen_con(content, CF_OUTPUT);
    if ws.is_null() {
        debuglog(
            c"%s:%s:%d - Skipping this output because it is currently being destroyed.\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            349,
        );
        return;
    }
    let fullscreen: *mut Con = con_get_fullscreen_con(ws, CF_OUTPUT);
    if !fullscreen.is_null() {
        (*fullscreen).rect = (*con).rect;
        x_raise_con(fullscreen);
        render_con(fullscreen);
        return;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if !((*child).type_0 as u32 != CT_DOCKAREA) {
            (*child).rect.height = 0 as u32;
            dockchild = (*child).nodes_head.tqh_first as *mut Con;
            while !dockchild.is_null() {
                (*child).rect.height = (*child)
                    .rect
                    .height
                    .wrapping_add((*dockchild).geometry.height);
                dockchild = (*dockchild).nodes.tqe_next as *mut Con;
            }
            height = (height as u32).wrapping_sub((*child).rect.height) as i32 as i32;
        }
        child = (*child).nodes.tqe_next as *mut Con;
    }
    child = (*con).nodes_head.tqh_first as *mut Con;
    while !child.is_null() {
        if (*child).type_0 as u32 == CT_CON {
            (*child).rect.x = x as u32;
            (*child).rect.y = y as u32;
            (*child).rect.width = (*con).rect.width;
            (*child).rect.height = height as u32;
        }
        (*child).rect.x = x as u32;
        (*child).rect.y = y as u32;
        (*child).rect.width = (*con).rect.width;
        (*child).deco_rect.x = 0 as u32;
        (*child).deco_rect.y = 0 as u32;
        (*child).deco_rect.width = 0 as u32;
        (*child).deco_rect.height = 0 as u32;
        y = (y as u32).wrapping_add((*child).rect.height) as i32 as i32;
        debuglog(
            c"%s:%s:%d - child at (%d, %d) with (%d x %d)\n"
                .as_ptr()
                .cast_mut(),
            c"../src/render.c".as_ptr(),
            c"render_output".as_ptr(),
            396,
            (*child).rect.x,
            (*child).rect.y,
            (*child).rect.width,
            (*child).rect.height,
        );
        x_raise_con(child);
        render_con(child);
        child = (*child).nodes.tqe_next as *mut Con;
    }
}
fn child_geometry(
    layout: Layout,
    border: BorderStyle,
    leaf: bool,
    parent: Rect,
    p: &mut RenderParams,
    index: i32,
    dock_height: u32,
) -> (Rect, Option<Rect>) {
    let mut rect = Rect {
        x: p.x as u32,
        y: p.y as u32,
        width: p.rect.width,
        height: p.rect.height,
    };
    let empty = Rect {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
    };
    let deco = match layout {
        Layout::SplitH | Layout::SplitV => {
            if layout == Layout::SplitH {
                rect.width = p.sizes[index as usize] as u32;
                p.x = (p.x as u32).wrapping_add(rect.width) as i32;
            } else {
                rect.height = p.sizes[index as usize] as u32;
                p.y = (p.y as u32).wrapping_add(rect.height) as i32;
            }
            if leaf {
                Some(if border == BorderStyle::Normal {
                    Rect {
                        width: rect.width,
                        height: p.deco_height as u32,
                        ..empty
                    }
                } else {
                    empty
                })
            } else {
                None
            }
        }
        Layout::Stacked => {
            let deco = Rect {
                x: rect.x.wrapping_sub(parent.x),
                y: rect
                    .y
                    .wrapping_sub(parent.y)
                    .wrapping_add(index.wrapping_mul(p.deco_height) as u32),
                width: rect.width,
                height: p.deco_height as u32,
            };
            if p.children > 1 || !matches!(border, BorderStyle::Pixel | BorderStyle::None) {
                let offset = p.deco_height.wrapping_mul(p.children) as u32;
                rect.y = rect.y.wrapping_add(offset);
                rect.height = rect.height.wrapping_sub(offset);
            }
            Some(deco)
        }
        Layout::Tabbed => {
            // Preserve the native float (not double) division before floor.
            let width = ((rect.width as f32 / p.children as f32) as f64).floor() as u32;
            let x = rect
                .x
                .wrapping_sub(parent.x)
                .wrapping_add((index as u32).wrapping_mul(width));
            let mut deco = Rect {
                x,
                y: rect.y.wrapping_sub(parent.y),
                width: if index == p.children - 1 {
                    rect.width.wrapping_sub(x)
                } else {
                    width
                },
                height: 0,
            };
            if p.children > 1 || !matches!(border, BorderStyle::Pixel | BorderStyle::None) {
                rect.y = rect.y.wrapping_add(p.deco_height as u32);
                rect.height = rect.height.wrapping_sub(p.deco_height as u32);
                deco.height = p.deco_height as u32;
            } else {
                deco.height = u32::from(border == BorderStyle::Pixel);
            }
            Some(deco)
        }
        Layout::DockArea => {
            rect.height = dock_height;
            p.y = (p.y as u32).wrapping_add(rect.height) as i32;
            Some(empty)
        }
        _ => unreachable!("unsupported child layout"),
    };
    (rect, deco)
}

unsafe fn render_child_geometry(con: *mut Con, child: *mut Con, p: &mut RenderParams, index: i32) {
    let (rect, deco) = child_geometry(
        (*con).layout,
        (*child).border_style,
        con_is_leaf(child),
        (*con).rect,
        p,
        index,
        (*child).geometry.height,
    );
    (*child).rect = rect;
    if let Some(deco) = deco {
        (*child).deco_rect = deco;
    }
}
// Internal helpers use Rust linkage rather than foreign symbol lookup.

// Canonical Cairo backend ABI; renderer logic remains local.

// Canonical Pango backend ABI; font policy remains local.

use i3_common::native_stdio::__assert_fail;

use crate::log::debuglog;

// Shared model types retain field layout while using one Rust identity.
use crate::types::gaps_t;
use crate::types::Con;
use crate::types::Rect;

// Internal calls and globals use canonical Rust identities.
use crate::con::con_border_style_rect;
use crate::con::con_descend_focused;
use crate::con::con_find_transient_for_window;
use crate::con::con_get_fullscreen_con;
use crate::con::con_get_fullscreen_covering_ws;
use crate::con::con_has_parent;
use crate::con::con_is_internal;
use crate::con::con_is_leaf;
use crate::con::con_num_children;
use crate::con::con_rect_size_in_orientation;
use crate::config::config;
use crate::gaps::calculate_effective_gaps;
use crate::gaps::gaps_has_adjacent_container;
use crate::gaps::gaps_should_inset_con;
use crate::output::output_get_content;
use crate::util::rect_add;
use crate::util::rect_sanitize_dimensions;
use crate::x::x_raise_con;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_rounding_and_remainders_follow_child_order() {
        assert_eq!(split_sizes(5, &[0.5, 0.5]), [2, 3]);
        assert_eq!(split_sizes(5, &[0.0, 0.0, 0.0]), [1, 2, 2]);
        assert_eq!(split_sizes(-5, &[0.5, 0.5]), [-2, -3]);
        assert_eq!(split_sizes(7, &[f64::NAN, 0.0]), [3, 4]);
        assert_eq!(decoration_height(13), 18);
    }
    #[test]
    fn gaps_and_window_insets_preserve_odd_halves_and_wrapping() {
        let gap = gaps_t {
            inner: 5,
            left: 3,
            right: 7,
            top: 11,
            bottom: 13,
        };
        let outer = workspace_inset(gap);
        assert_eq!(
            (outer.x, outer.y, outer.width, outer.height),
            (3, 11, (-10i32) as u32, (-24i32) as u32)
        );
        let inset = inner_inset(5, [true, false, false, true]);
        assert_eq!(
            (inset.x, inset.y, inset.width, inset.height),
            (2, 5, (-7i32) as u32, (-7i32) as u32)
        );
        let window = window_geometry(
            Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 80,
            },
            Some(Rect {
                x: 2,
                y: 18,
                width: (-4i32) as u32,
                height: (-20i32) as u32,
            }),
            1,
        );
        assert_eq!(
            (window.x, window.y, window.width, window.height),
            (2, 18, 94, 58)
        );
        assert!(popup_blocked(
            PopupDuringFullscreen::Smart,
            false,
            true,
            false
        ));
        assert!(!popup_blocked(
            PopupDuringFullscreen::Smart,
            true,
            true,
            false
        ));
        assert!(!popup_blocked(
            PopupDuringFullscreen::All,
            false,
            false,
            false
        ));
    }
    #[test]
    fn layout_geometry_preserves_offsets_and_single_child_borders() {
        let parent = Rect {
            x: (-10i32) as u32,
            y: 20,
            width: 101,
            height: 80,
        };
        let mut p = RenderParams {
            x: -8,
            y: 22,
            rect: Rect {
                width: 99,
                height: 78,
                ..parent
            },
            children: 3,
            deco_height: 18,
            sizes: vec![33, 33, 33],
        };
        let (r, d) = child_geometry(
            Layout::Tabbed,
            BorderStyle::Normal,
            true,
            parent,
            &mut p,
            2,
            0,
        );
        let d = d.unwrap();
        assert_eq!(
            (r.x, r.y, r.height, d.x, d.width),
            ((-8i32) as u32, 40, 60, 68, 31)
        );
        p.children = 1;
        let (r, d) = child_geometry(
            Layout::Stacked,
            BorderStyle::Pixel,
            true,
            parent,
            &mut p,
            0,
            0,
        );
        assert_eq!((r.y, r.height, d.unwrap().height), (22, 78, 18));
        let (_, d) = child_geometry(
            Layout::Tabbed,
            BorderStyle::Pixel,
            true,
            parent,
            &mut p,
            0,
            0,
        );
        assert_eq!(d.unwrap().height, 1);
        let (r, d) = child_geometry(
            Layout::DockArea,
            BorderStyle::None,
            true,
            parent,
            &mut p,
            0,
            17,
        );
        assert_eq!((r.height, p.y, d.unwrap().height), (17, 39, 0));
    }
}
