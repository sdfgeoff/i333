//! Golden bytes captured from the pre-schema production emitters at 01b7ac4d.
use super::*;
use crate::con::arena::parent::set_parent_ptr;
use crate::domains::ContainerType;
use crate::domains::{DockMatch, MatchInsertWhere};
use crate::types::Window;
use std::collections::BTreeMap;

fn normalize_pointer_ids(bytes: &[u8], pointers: &[*mut Con]) -> String {
    let mut text = String::from_utf8(bytes.to_vec()).expect("JSON output is ASCII/UTF8");
    fn normalized(number: &str, pointers: &[*mut Con]) -> Option<String> {
        pointers
            .iter()
            .position(|pointer| number == (*pointer as usize as i64).to_string())
            .map(|index| format!("\"con{index}\""))
    }
    // Match a complete numeric token after exactly the id key. Prefix-matching
    // an address could otherwise conceal a different, larger numeric value.
    let mut cursor = 0;
    while let Some(offset) = text[cursor..].find("\"id\":") {
        let start = cursor + offset + 5;
        let end = text[start..]
            .find(|ch: char| !ch.is_ascii_digit() && ch != '-')
            .map_or(text.len(), |len| start + len);
        if let Some(replacement) = normalized(&text[start..end], pointers) {
            text.replace_range(start..end, &replacement);
            cursor = start + replacement.len();
        } else {
            cursor = end.max(start + 1).min(text.len());
        }
    }
    // Normalize only numeric pointer positions in a focus array. Geometry,
    // window XIDs, quoted names and all other numeric fields remain untouched.
    let mut cursor = 0;
    while let Some(offset) = text[cursor..].find("\"focus\":[") {
        let start = cursor + offset + 9;
        let Some(length) = text[start..].find(']') else {
            break;
        };
        let end = start + length;
        let normalized = text[start..end]
            .split(',')
            .map(|number| normalized(number, pointers).unwrap_or_else(|| number.to_owned()))
            .collect::<Vec<_>>()
            .join(",");
        text.replace_range(start..end, &normalized);
        cursor = start + normalized.len() + 1;
    }
    text
}

#[test]
fn pointer_normalization_keeps_numeric_boundaries_and_non_id_fields() {
    let pointers = [
        std::ptr::without_provenance_mut::<Con>(123),
        std::ptr::without_provenance_mut::<Con>(321),
    ];
    let input = br#"{"id":1234,"focus":[123,321,1234],"rect":{"x":123},"name":"123","window":123}"#;
    assert_eq!(
        normalize_pointer_ids(input, &pointers),
        r#"{"id":1234,"focus":["con0","con1",1234],"rect":{"x":123},"name":"123","window":123}"#
    );
    assert_eq!(
        normalize_pointer_ids(br#"{"id":123,"id":321}"#, &pointers),
        r#"{"id":"con0","id":"con1"}"#
    );
}

unsafe fn node_bytes(node: *mut Con, restart: bool, depth: usize, pointers: &[*mut Con]) -> String {
    let bytes = if depth == 0 {
        node_payload(node, restart)
    } else {
        let mut output = i3_common::json_rejection::RejectedOutput::new();
        for _ in 0..depth {
            output.start_array().unwrap();
        }
        dump_node_stream(&mut output, node, restart);
        output.bytes().to_vec()
    };
    normalize_pointer_ids(&bytes, pointers)
}
unsafe fn collect() -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    let mut allocated = Vec::new();
    unsafe fn node(all: &mut Vec<*mut Con>, kind: ContainerType, parent: *mut Con) -> *mut Con {
        let raw = crate::con::con_new_skeleton(std::ptr::null_mut(), std::ptr::null_mut());
        (*raw).type_0 = kind;
        set_parent_ptr(raw, parent).expect("registered fixture ancestry");
        (*raw).layout = Layout::SplitH;
        (*raw).workspace_layout = Layout::Default;
        (*raw).border_style = BorderStyle::Pixel;
        (*raw).name = i3_common::safewrappers::sstrdup(c"node".as_ptr());
        all.push(raw);
        raw
    }
    let root = node(&mut allocated, ContainerType::Root, std::ptr::null_mut());
    let output = node(&mut allocated, ContainerType::Output, root);
    let workspace = node(&mut allocated, ContainerType::Workspace, output);
    let leaf = node(&mut allocated, ContainerType::Con, workspace);
    (*workspace).num = 3;
    (*workspace).rect.x = 100;
    (*workspace).rect.y = 50;
    (*workspace).gaps = gaps_t {
        inner: 1,
        top: 2,
        right: 3,
        bottom: 4,
        left: 5,
    };
    (*leaf).rect = Rect {
        x: u32::MAX,
        y: 80,
        width: 640,
        height: 480,
    };
    (*leaf).deco_rect = Rect {
        x: 3,
        y: 4,
        width: 100,
        height: 20,
    };
    (*leaf).window_rect = Rect {
        x: 2,
        y: 20,
        width: 636,
        height: 458,
    };
    (*leaf).geometry = Rect {
        x: 0x8000_0000,
        y: 7,
        width: 40,
        height: 30,
    };
    focused = leaf;
    result.insert("node_plain".into(), node_bytes(leaf, false, 0, &allocated));
    result.insert(
        "workspace".into(),
        node_bytes(workspace, false, 0, &allocated),
    );
    let mut window = Box::new(Window::default());
    window.id = 4242;
    window.name = Some(i3_common::string::I3String::new("quoted \"title\" 雪"));
    window.class_class = c"App".as_ptr().cast_mut();
    window.class_instance = c"".as_ptr().cast_mut();
    window.role = c"dialog".as_ptr().cast_mut();
    window.machine = c"host".as_ptr().cast_mut();
    window.transient_for = 123;
    (*leaf).window = &mut *window;
    (*leaf).border_style = BorderStyle::Normal;
    (*leaf).depth = 32;
    (*leaf).title_format = c"%title [%class]".as_ptr().cast_mut();
    (*leaf).percent = 0.25;
    result.insert("node_window".into(), node_bytes(leaf, false, 0, &allocated));
    result.insert("node_restart".into(), node_bytes(leaf, true, 0, &allocated));
    previous_workspace_name = c"old workspace".as_ptr().cast_mut();
    result.insert("root_restart".into(), node_bytes(root, true, 0, &allocated));
    for (name, percent) in [
        ("negative_zero", -0.0),
        ("nan", f64::NAN),
        ("infinity", f64::INFINITY),
    ] {
        (*leaf).percent = percent;
        result.insert(name.into(), node_bytes(leaf, false, 0, &allocated));
    }
    (*leaf).percent = 0.5;
    let invalid = CStr::from_bytes_with_nul(b"\xff\0").unwrap();
    window.class_class = invalid.as_ptr().cast_mut();
    result.insert(
        "invalid_class".into(),
        node_bytes(leaf, false, 0, &allocated),
    );
    window.class_class = c"App".as_ptr().cast_mut();
    (*leaf).type_0 = ContainerType::from_raw(42);
    result.insert(
        "unknown_type".into(),
        node_bytes(leaf, false, 0, &allocated),
    );
    (*leaf).type_0 = ContainerType::Con;
    let floating_child = node(&mut allocated, ContainerType::Floating, workspace);
    floating::append(
        identity(workspace).unwrap(),
        identity(floating_child).unwrap(),
    )
    .unwrap();
    result.insert(
        "workspace_floating".into(),
        node_bytes(workspace, false, 0, &allocated),
    );
    let focus_owner = identity(workspace).unwrap();
    let leaf_id = identity(leaf).unwrap();
    let floating_id = identity(floating_child).unwrap();
    focus::append(focus_owner, leaf_id).unwrap();
    focus::append(focus_owner, floating_id).unwrap();
    result.insert(
        "workspace_focus_order".into(),
        node_bytes(workspace, false, 0, &allocated),
    );
    focus::remove(focus_owner, leaf_id).unwrap();
    focus::remove(focus_owner, floating_id).unwrap();
    let mut swallow = Match::empty();
    swallow.window_type = 0;
    swallow.urgent = crate::types::UrgencyMatch::Latest;
    swallow.dock = DockMatch::Top;
    swallow.insert_where = MatchInsertWhere::AssignWorkspace;
    swallow.class = Some(crate::regex::Regex::compile(c"^App$").unwrap());
    let mut ignored_swallow = Match::empty();
    ignored_swallow.restart_mode = true;
    swallow.matches.tqe_next = &mut ignored_swallow;
    (*leaf).swallow_head.tqh_first = &mut swallow;
    result.insert(
        "node_swallows".into(),
        node_bytes(leaf, false, 0, &allocated),
    );
    result.insert(
        "node_restart_swallows".into(),
        node_bytes(leaf, true, 0, &allocated),
    );
    // Numeric layout fields remain open domains at the production JSON edge.
    // Check signed/unsigned extremes without changing the captured golden set.
    for (fullscreen, dock, insert) in [
        (u32::MAX, i32::MIN, u32::MAX),
        (0x8000_0000, i32::MAX, 0x8000_0000),
        (3, 4, 3),
    ] {
        (*leaf).fullscreen_mode = FullscreenMode::from_raw(fullscreen);
        swallow.dock = DockMatch::from_raw(dock);
        swallow.insert_where = MatchInsertWhere::from_raw(insert);
        let json: serde_json::Value =
            serde_json::from_str(&node_bytes(leaf, false, 0, &allocated)).unwrap();
        assert_eq!(json["fullscreen_mode"], fullscreen);
        assert_eq!(json["swallows"][0]["dock"], dock);
        assert_eq!(json["swallows"][0]["insert_where"], insert);
    }
    swallow.dock = DockMatch::DontCheck;
    let json: serde_json::Value =
        serde_json::from_str(&node_bytes(leaf, false, 0, &allocated)).unwrap();
    assert!(json["swallows"][0].get("dock").is_none());
    assert!(json["swallows"][0].get("insert_where").is_none());
    (*leaf).fullscreen_mode = FullscreenMode::None;
    (*leaf).swallow_head.tqh_first = std::ptr::null_mut();
    for depth in [124, 125, 126, 127, 128] {
        result.insert(
            format!("depth_{depth}"),
            node_bytes(leaf, false, depth, &allocated),
        );
    }
    // A real nested graph exercises streaming recursion instead of constructing
    // a recursively owned Serde DTO before the depth limit can be checked.
    nodes::append(identity(root).unwrap(), identity(output).unwrap()).unwrap();
    nodes::append(identity(output).unwrap(), identity(workspace).unwrap()).unwrap();
    nodes::append(identity(workspace).unwrap(), identity(leaf).unwrap()).unwrap();
    let mut parent = leaf;
    for _ in 0..70 {
        let child = node(&mut allocated, ContainerType::Con, parent);
        nodes::append(identity(parent).unwrap(), identity(child).unwrap()).unwrap();
        parent = child;
    }
    result.insert("deep_graph".into(), node_bytes(root, false, 0, &allocated));
    for raw in &allocated {
        let owner = identity(*raw).unwrap();
        while let Some(child) = nodes::first(owner) {
            nodes::remove(owner, child).unwrap();
        }
    }
    (*leaf).window = std::ptr::null_mut();
    for raw in allocated.into_iter().rev() {
        crate::con::con_free(raw);
    }
    focused = std::ptr::null_mut();
    previous_workspace_name = std::ptr::null_mut();
    // Barconfig remains a native-compatible POD; no owned Rust fields reside
    // in its calloc-initialized representation.
    let mut bar: Barconfig = std::mem::zeroed();
    bar.id = c"bar0".as_ptr().cast_mut();
    let bytes = snapshot_bar_config(&mut bar).to_bytes();
    result.insert("bar_sparse".into(), String::from_utf8(bytes).unwrap());
    bar.socket_path = c"socket".as_ptr().cast_mut();
    bar.status_command = c"status --flag".as_ptr().cast_mut();
    bar.workspace_command = c"workspaces".as_ptr().cast_mut();
    bar.font = c"pango:monospace 8".as_ptr().cast_mut();
    bar.separator_symbol = c"|".as_ptr().cast_mut();
    bar.bar_height = 30;
    bar.padding = Rect {
        x: u32::MAX,
        y: 2,
        width: 3,
        height: 4,
    };
    bar.colors.background = c"#112233".as_ptr().cast_mut();
    bar.colors.statusline = c"".as_ptr().cast_mut();
    bar.colors.binding_mode_text = c"#abcdef".as_ptr().cast_mut();
    let mut bar_outputs = [
        c"PrImArY".as_ptr().cast_mut(),
        c"nonprimary".as_ptr().cast_mut(),
    ];
    bar.outputs = bar_outputs.as_mut_ptr();
    bar.num_outputs = 2;
    let mut binding: Barbinding = std::mem::zeroed();
    binding.input_code = 1;
    binding.command = c"nop \"click\"".as_ptr().cast_mut();
    binding.release = true;
    bar.bar_bindings.tqh_first = &mut binding;
    let mut tray: tray_output_t = std::mem::zeroed();
    tray.output = c"PRIMARY".as_ptr().cast_mut();
    bar.tray_outputs.tqh_first = &mut tray;
    let bytes = snapshot_bar_config(&mut bar).to_bytes();
    result.insert("bar_full".into(), String::from_utf8(bytes).unwrap());
    bar.socket_path = invalid.as_ptr().cast_mut();
    let bytes = snapshot_bar_config(&mut bar).to_bytes();
    result.insert(
        "bar_invalid_socket".into(),
        String::from_utf8(bytes).unwrap(),
    );
    result
}
#[test]
fn production_node_restart_and_bar_outputs_match_baseline() {
    if std::env::var_os("I333_PRIVATE_SCHEMA_PROBE").is_none() {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "ipc::snapshot_tests::production_node_restart_and_bar_outputs_match_baseline",
                "--nocapture",
            ])
            .env("I333_PRIVATE_SCHEMA_PROBE", "1");
        for key in [
            "DISPLAY",
            "WAYLAND_DISPLAY",
            "I3SOCK",
            "XAUTHORITY",
            "DBUS_SESSION_BUS_ADDRESS",
            "DESKTOP_STARTUP_ID",
            "NOTIFY_SOCKET",
            "LISTEN_PID",
            "LISTEN_FDS",
            "LISTEN_FDNAMES",
            "LIBEV_FLAGS",
        ] {
            command.env_remove(key);
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let actual = unsafe { collect() };
    let expected: BTreeMap<String, String> =
        serde_json::from_str(include_str!("../tests/fixtures/ipc-output-baseline.json")).unwrap();
    assert_eq!(actual, expected);
}

use crate::con::arena::nodes;
