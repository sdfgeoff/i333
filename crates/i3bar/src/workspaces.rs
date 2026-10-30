/*
 * i3bar - an xcb-based status- and ws-bar for i3
 * © 2010 Axel Wagner and contributors (see LICENSE).
 *
 * workspaces.c: Maintaining the workspace lists
 */
//! Workspace JSON is staged in owned records before touching the event-thread
//! output graph. Its intrusive lists own Box<Workspace> nodes, whose text drops
//! automatically when the lists are detached.

use crate::child::{i3_output, C2RustUnnamed_4};
use crate::config::config;
use crate::outputs::{get_output_by_name, outputs};
use i3_common::font::predict_text_width;
use i3_common::string::I3String;
use i3_json::{Event, EventHandler, ParseError, Parser};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::io::Write;
use std::ptr;
use std::rc::Rc;

/// A node owned by exactly one output list. Only append_workspace transfers a
/// Box into that list; free_workspaces takes each Box back after detaching it.
#[repr(C)]
pub struct Workspace {
    pub id: usize,
    pub num: i32,
    pub canonical_name: Option<CString>,
    pub name: I3String,
    pub name_width: i32,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: *mut i3_output,
    pub tailq: C2RustUnnamed_4,
}

impl Workspace {
    pub fn error(name: I3String, name_width: i32) -> Self {
        Self {
            id: 0,
            num: -1,
            canonical_name: None,
            name,
            name_width,
            visible: true,
            focused: false,
            urgent: true,
            output: ptr::null_mut(),
            tailq: C2RustUnnamed_4 {
                tqe_next: ptr::null_mut(),
                tqe_prev: ptr::null_mut(),
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
struct NameOptions {
    strip_numbers: bool,
    strip_name: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NameSnapshot {
    canonical: CString,
    display: Vec<u8>,
    explicit_length: bool,
}

fn c_prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}

fn name_snapshot(bytes: &[u8], num: i32, options: NameOptions) -> NameSnapshot {
    let canonical = CString::new(c_prefix(bytes)).expect("prefix contains no NUL");
    let (display, explicit_length) = if (options.strip_numbers || options.strip_name) && num >= 0 {
        let number = num.to_string();
        // Upstream uses strspn, not a numeric-prefix comparison: every leading
        // byte belonging to the workspace number's digit set is stripped.
        let mut offset = bytes
            .iter()
            .take_while(|byte| **byte != 0 && number.as_bytes().contains(byte))
            .count();
        if offset != 0 && bytes.get(offset) == Some(&b':') {
            offset += 1;
        }
        if options.strip_numbers && offset < bytes.len() {
            (bytes[offset..].to_vec(), true)
        } else {
            (number.into_bytes(), false)
        }
    } else {
        (bytes.to_vec(), true)
    };
    NameSnapshot {
        canonical,
        display,
        explicit_length,
    }
}

#[derive(Debug)]
struct WorkspaceRecord {
    id: usize,
    num: i32,
    names: Vec<NameSnapshot>,
    visible: bool,
    focused: bool,
    urgent: bool,
    output_names: Vec<CString>,
    completed: bool,
}

impl Default for WorkspaceRecord {
    fn default() -> Self {
        Self {
            id: 0,
            num: -1,
            names: Vec::new(),
            visible: false,
            focused: false,
            urgent: false,
            output_names: Vec::new(),
            completed: false,
        }
    }
}

struct WorkspaceReader {
    options: NameOptions,
    key: Option<Vec<u8>>,
    current: Option<WorkspaceRecord>,
    records: Vec<WorkspaceRecord>,
    parsing_rect: bool,
    error: Option<&'static str>,
}
struct SharedReader(Rc<RefCell<WorkspaceReader>>);

impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => reader.key = Some(c_prefix(key).to_vec()),
            Event::StartMap if reader.key.is_none() => {
                if let Some(previous) = reader.current.take() {
                    reader.records.push(previous);
                }
                reader.current = Some(WorkspaceRecord::default());
                reader.parsing_rect = false;
            }
            Event::StartMap => reader.parsing_rect = true,
            Event::EndMap => {
                if reader.parsing_rect {
                    reader.parsing_rect = false;
                } else if let Some(workspace) = reader.current.as_mut() {
                    workspace.completed = true;
                }
            }
            Event::Boolean(value) => {
                let key = reader.key.take();
                let Some(workspace) = reader.current.as_mut() else {
                    return false;
                };
                match key.as_deref() {
                    Some(b"visible") => workspace.visible = value,
                    Some(b"focused") => workspace.focused = value,
                    Some(b"urgent") => workspace.urgent = value,
                    _ => return false,
                }
            }
            Event::Number(number) if !number.contains(['.', 'e', 'E']) => {
                let value = match number.parse::<i64>() {
                    Ok(value) if value != i64::MIN => value,
                    _ => {
                        reader.error = Some("parse error: integer overflow");
                        return false;
                    }
                };
                let key = reader.key.take();
                let Some(workspace) = reader.current.as_mut() else {
                    return false;
                };
                match key.as_deref() {
                    // Narrowing deliberately retains the upstream numeric
                    // projections after its signed-64 callback range check.
                    Some(b"id") => workspace.id = value as usize,
                    Some(b"num") => workspace.num = value as i32,
                    Some(b"x" | b"y" | b"width" | b"height") => {}
                    _ => return false,
                }
            }
            Event::String(value) => {
                let key = reader.key.take();
                let options = reader.options;
                let Some(workspace) = reader.current.as_mut() else {
                    return false;
                };
                match key.as_deref() {
                    Some(b"name") => {
                        workspace
                            .names
                            .push(name_snapshot(value, workspace.num, options))
                    }
                    Some(b"output") => workspace
                        .output_names
                        .push(CString::new(c_prefix(value)).expect("prefix contains no NUL")),
                    _ => return false,
                }
            }
            _ => {}
        }
        true
    }

    fn error_message(&self) -> Option<&str> {
        self.0.borrow().error
    }
}

fn read_workspaces(json: &[u8], options: NameOptions) -> Result<Vec<WorkspaceRecord>, ParseError> {
    let reader = Rc::new(RefCell::new(WorkspaceReader {
        options,
        key: None,
        current: None,
        records: Vec::new(),
        parsing_rect: false,
        error: None,
    }));
    let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
    parser.options_mut().allow_legacy_whitespace = true;
    // Do not call finish: workspace_command and upstream's callback parser
    // accept a completed workspace before the outer array has closed.
    parser.parse(json)?;
    drop(parser);
    let mut reader = Rc::try_unwrap(reader)
        .ok()
        .expect("parser released its reader")
        .into_inner();
    if let Some(current) = reader.current.take() {
        reader.records.push(current);
    }
    Ok(reader.records)
}

fn output_targets<T: Copy + PartialEq>(
    workspace: &WorkspaceRecord,
    mut lookup: impl FnMut(&CStr) -> Option<T>,
    fallback: Option<T>,
) -> Vec<T> {
    if workspace.output_names.is_empty() {
        return if workspace.completed {
            fallback.into_iter().collect()
        } else {
            Vec::new()
        };
    }
    let mut targets = Vec::new();
    for name in &workspace.output_names {
        if let Some(output) = lookup(name) {
            if !targets.contains(&output) {
                targets.push(output);
            }
        }
    }
    // A later unknown output does not erase an earlier known one. Repeated
    // known outputs used to insert the same node twice, forming cycles or
    // double frees. Independent nodes per distinct output keep the visible
    // result without reproducing that ownership corruption.
    targets
}

/// # Safety
/// output and its workspace head are live event-thread allocations, and no
/// iterator or draw operation is borrowing the output list during this call.
pub unsafe fn append_workspace(output: *mut i3_output, mut workspace: Box<Workspace>) {
    // SAFETY: The caller supplies the live, uniquely modified output list.
    unsafe {
        let head = (*output).workspaces;
        assert!(!head.is_null(), "output workspace head is initialized");
        workspace.output = output;
        workspace.tailq.tqe_next = ptr::null_mut();
        workspace.tailq.tqe_prev = (*head).tqh_last;
        let workspace = Box::into_raw(workspace);
        *(*head).tqh_last = workspace;
        (*head).tqh_last = &raw mut (*workspace).tailq.tqe_next;
    }
}

/// # Safety
/// Called on the bar event thread with live output graph and configured font.
unsafe fn apply_workspaces(records: Vec<WorkspaceRecord>) {
    unsafe {
        let fallback = if outputs.is_null() || (*outputs).slh_first.is_null() {
            None
        } else {
            let primary = get_output_by_name(c"primary".as_ptr().cast_mut());
            Some(if primary.is_null() {
                (*outputs).slh_first
            } else {
                primary
            })
        };
        for record in records {
            let Some(snapshot) = record.names.last() else {
                continue;
            };
            let mut last_name = None;
            for snapshot in &record.names {
                let name = if snapshot.explicit_length {
                    I3String::from_bytes_with_legacy_length(&snapshot.display)
                } else {
                    I3String::from_bytes(&snapshot.display)
                };
                name.set_markup(true);
                // Native font helpers only read the owned text and its interior
                // caches; ownership stays with this record until list transfer.
                let text = &name;
                let width = predict_text_width(text);
                if config.verbose {
                    let _ = writeln!(std::io::stdout().lock(), "[../i3bar/src/workspaces.c:146] Got workspace canonical: {}, name: '{}', name_width: {}, glyphs: {}", snapshot.canonical.to_string_lossy(), name.as_str().expect("workspace text is UTF-8"), width, text.rendering_glyphs().len());
                }
                last_name = Some((name, width));
            }
            let (name, width) = last_name.expect("workspace has a name snapshot");
            let targets = output_targets(
                &record,
                |name| {
                    if fallback.is_none() {
                        return None;
                    }
                    let output = get_output_by_name(name.as_ptr().cast_mut());
                    (!output.is_null()).then_some(output)
                },
                fallback,
            );
            for output in targets {
                append_workspace(
                    output,
                    Box::new(Workspace {
                        id: record.id,
                        num: record.num,
                        canonical_name: Some(snapshot.canonical.clone()),
                        name: name.clone(),
                        name_width: width,
                        visible: record.visible,
                        focused: record.focused,
                        urgent: record.urgent,
                        output: ptr::null_mut(),
                        tailq: C2RustUnnamed_4 {
                            tqe_next: ptr::null_mut(),
                            tqe_prev: ptr::null_mut(),
                        },
                    }),
                );
            }
        }
    }
}

/// # Safety
/// json is readable for size bytes, and the caller owns the bar event thread's
/// output graph. All nodes on existing workspace lists were Box-allocated here.
pub unsafe fn parse_workspaces_json(json: *const u8, size: usize) {
    unsafe {
        let input = if size == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(json, size)
        };
        let options = NameOptions {
            strip_numbers: config.strip_ws_numbers,
            strip_name: config.strip_ws_name,
        };
        free_workspaces();
        match read_workspaces(input, options) {
            Ok(records) => apply_workspaces(records),
            Err(error) => {
                let mut stderr = std::io::stderr().lock();
                let _ = write!(stderr, "[../i3bar/src/workspaces.c:275] ERROR: Could not parse workspaces reply, error:\n{error}\n\njson:---");
                let _ = stderr.write_all(c_prefix(input));
                let _ = stderr.write_all(b"---\n");
                drop(stderr);
                if !config.workspace_command.is_null() {
                    crate::child::kill_ws_child();
                    crate::child::set_workspace_button_error(
                        c"Could not parse workspace_command's JSON".as_ptr(),
                    );
                } else {
                    std::process::exit(libc::EXIT_FAILURE);
                }
            }
        }
    }
}

/// # Safety
/// The caller exclusively owns the live event-thread output graph. Every node
/// belongs to exactly one list and was transferred by append_workspace.
pub unsafe fn free_workspaces() {
    unsafe {
        if outputs.is_null() {
            return;
        }
        let mut output = (*outputs).slh_first;
        while !output.is_null() {
            let head = (*output).workspaces;
            if !head.is_null() {
                clear_workspace_head(&mut *head);
            }
            output = (*output).slist.sle_next;
        }
    }
}

/// # Safety
/// The head exclusively owns an acyclic list of nodes transferred by
/// append_workspace. No borrowed node pointer is used after this call.
unsafe fn clear_workspace_head(head: &mut crate::child::ws_head) {
    let mut workspace = head.tqh_first;
    head.tqh_first = ptr::null_mut();
    head.tqh_last = &raw mut head.tqh_first;
    while !workspace.is_null() {
        // SAFETY: Each detached node is a distinct live Box; next is copied
        // before its predecessor is released, and text drops with the node.
        unsafe {
            let next = (*workspace).tailq.tqe_next;
            drop(Box::from_raw(workspace));
            workspace = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn records(json: &[u8], options: NameOptions) -> Vec<WorkspaceRecord> {
        read_workspaces(json, options).unwrap()
    }

    #[test]
    fn names_keep_strspn_digit_set_and_field_order_behavior() {
        let options = NameOptions {
            strip_numbers: true,
            strip_name: false,
        };
        let parsed = records(br#"[{"num":12,"name":"2112:editor"},{"name":"12:early","num":12},{"num":12,"name":"12:"}]"#, options);
        assert_eq!(parsed[0].names[0].display, b"editor");
        assert_eq!(parsed[1].names[0].display, b"12:early");
        assert_eq!(parsed[2].names[0].display, b"12");
        let parsed = records(
            br#"[{"num":12,"name":"letters"}]"#,
            NameOptions {
                strip_numbers: false,
                strip_name: true,
            },
        );
        assert_eq!(parsed[0].names[0].display, b"12");
    }

    #[test]
    fn owned_names_and_keys_preserve_nul_prefix_rules() {
        let json = br#"[{"visible\u0000ignored":true,"num":1,"name":"1:one\u0000rest","output":"primary\u0000ignored"}]"#.to_vec();
        let parsed = records(
            &json,
            NameOptions {
                strip_numbers: true,
                strip_name: false,
            },
        );
        drop(json);
        assert!(parsed[0].visible);
        assert_eq!(parsed[0].names[0].canonical.to_bytes(), b"1:one");
        assert_eq!(parsed[0].names[0].display, b"one\0rest");
        assert!(parsed[0].names[0].explicit_length);
        assert_eq!(parsed[0].output_names[0].to_bytes(), b"primary");
    }

    #[test]
    fn rect_fields_unknown_types_and_legacy_integer_ranges_are_distinct() {
        let parsed = records(br#"[{"rect":{"x":1,"y":2,"width":3,"height":4},"id":-1,"num":4294967297,"name":"one","unknown":null}]"#, NameOptions::default());
        assert_eq!(parsed[0].id, usize::MAX);
        assert_eq!(parsed[0].num, 1);
        assert!(parsed[0].completed);
        for value in ["true", "1", "\"text\""] {
            let json = format!(r#"[{{"name":"one","unknown":{value}}}]"#);
            assert!(read_workspaces(json.as_bytes(), NameOptions::default()).is_err());
        }
        for value in ["9223372036854775808", "-9223372036854775808"] {
            let json = format!(r#"[{{"name":"one","id":{value}}}]"#);
            let error = read_workspaces(json.as_bytes(), NameOptions::default()).unwrap_err();
            assert_eq!(error.message, "parse error: integer overflow");
        }
        let parsed = records(
            br#"[{"num":1.5,"name":"one","unknown":1e999}]"#,
            NameOptions::default(),
        );
        assert_eq!(parsed[0].num, -1);
    }

    #[test]
    fn output_mentions_keep_prior_matches_without_reusing_owned_nodes() {
        let parsed = records(br#"[{"name":"one","output":"a","output":"missing","output":"a","output":"b"},{"name":"two","output":"missing"},{"name":"fallback"}]"#, NameOptions::default());
        let lookup = |name: &CStr| match name.to_bytes() {
            b"a" => Some(1),
            b"b" => Some(2),
            _ => None,
        };
        assert_eq!(output_targets(&parsed[0], lookup, Some(9)), vec![1, 2]);
        assert!(output_targets(&parsed[1], lookup, Some(9)).is_empty());
        assert_eq!(output_targets(&parsed[2], lookup, Some(9)), vec![9]);
    }

    #[test]
    fn partial_records_only_publish_when_upstream_had_an_output_attachment() {
        let parsed = records(
            br#"[{"name":"one","output":"known","#,
            NameOptions::default(),
        );
        assert!(!parsed[0].completed);
        assert_eq!(output_targets(&parsed[0], |_| Some(1), Some(9)), vec![1]);
        let parsed = records(br#"[{"name":"one","#, NameOptions::default());
        assert!(output_targets(&parsed[0], |_| Some(1), Some(9)).is_empty());
    }

    #[test]
    fn duplicate_names_keep_each_formatting_snapshot_but_last_text_wins() {
        let parsed = records(
            br#"[{"num":1,"name":"1:first","num":2,"name":"2:last"}]"#,
            NameOptions {
                strip_numbers: true,
                strip_name: false,
            },
        );
        assert_eq!(
            parsed[0]
                .names
                .iter()
                .map(|name| name.display.as_slice())
                .collect::<Vec<_>>(),
            vec![b"first".as_slice(), b"last"]
        );
        assert_eq!(parsed[0].num, 2);
    }

    #[test]
    fn intrusive_lists_detach_then_drop_owned_nodes_and_can_be_cleared_twice() {
        let mut output = Box::new(i3_output::new(c"test".to_owned()));
        let head = output.workspaces;
        unsafe {
            append_workspace(
                &raw mut *output,
                Box::new(Workspace::error(I3String::new("one"), 3)),
            );
            append_workspace(
                &raw mut *output,
                Box::new(Workspace::error(I3String::new("two"), 3)),
            );
            let first = (*head).tqh_first;
            let second = (*first).tailq.tqe_next;
            assert_eq!((*first).name.as_str().unwrap(), "one");
            assert_eq!((*second).name.as_str().unwrap(), "two");
            assert_eq!((*second).tailq.tqe_prev, &raw mut (*first).tailq.tqe_next);
            clear_workspace_head(&mut *head);
            clear_workspace_head(&mut *head);
        }
        unsafe {
            assert!((*head).tqh_first.is_null());
            assert_eq!((*head).tqh_last, &raw mut (*head).tqh_first);
        }
    }
}
