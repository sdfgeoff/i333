/* i3bar - Maintaining the outputs list. See LICENSE. */
//! JSON parsing owns its records. Native windows and intrusive lists are
//! modified only at the explicitly unsafe event-thread boundary.
use crate::child::{i3_ws, outputs_head, rect_t, tc_head, trayclient, ws_head, OutputLink};
use crate::config::config;
use crate::workspaces::free_workspaces;
use crate::xcb::destroy_window;
use i3_common::draw_util::surface_t;
use i3_common::json_stream::{Event, EventHandler, ParseError, Parser};
use std::cell::RefCell;
use std::ffi::CString;
use std::ptr;
use std::rc::Rc;

// Intrusive heads publish self-tail pointers. Convert the allocation into raw
// ownership before deriving those links; moving this owner never retags a Box.
struct OwnedQueueHead<T> {
    pointer: std::ptr::NonNull<T>,
}
impl<T> OwnedQueueHead<T> {
    fn new(value: T) -> Self {
        Self {
            pointer: std::ptr::NonNull::new(Box::into_raw(Box::new(value)))
                .expect("Box is non-null"),
        }
    }
    fn as_ptr(&self) -> *mut T {
        self.pointer.as_ptr()
    }
}
impl<T> Drop for OwnedQueueHead<T> {
    fn drop(&mut self) {
        // The output cleanup path drains its lists before releasing the heads.
        // No Box or reference to the head exists while intrusive aliases are live.
        unsafe {
            drop(Box::from_raw(self.pointer.as_ptr()));
        }
    }
}

/// Box-owned node in the event-thread output list. The head owners keep their
/// addresses stable; public raw aliases are only for legacy native callers.
/// No Drop traverses those forgeable raw links or releases native resources.
pub struct Output {
    pub name: Vec<u8>,
    pub active: bool,
    pub primary: bool,
    pub visible: bool,
    pub ws: i32,
    pub rect: rect_t,
    pub buffer: surface_t,
    pub statusline_buffer: surface_t,
    pub statusline_width: i32,
    pub bar: surface_t,
    pub workspaces: *mut ws_head,
    pub trayclients: *mut tc_head,
    pub slist: OutputLink,
    _workspace_head: OwnedQueueHead<ws_head>,
    _trayclient_head: OwnedQueueHead<tc_head>,
}
impl Output {
    pub fn new(name: Vec<u8>) -> Self {
        let workspace_head = OwnedQueueHead::new(ws_head {
            tqh_first: ptr::null_mut(),
            tqh_last: ptr::null_mut(),
        });
        unsafe {
            (*workspace_head.as_ptr()).tqh_last = &raw mut (*workspace_head.as_ptr()).tqh_first;
        }
        let trayclient_head = OwnedQueueHead::new(tc_head {
            tqh_first: ptr::null_mut(),
            tqh_last: ptr::null_mut(),
        });
        unsafe {
            (*trayclient_head.as_ptr()).tqh_last = &raw mut (*trayclient_head.as_ptr()).tqh_first;
        }
        Self {
            name,
            active: false,
            primary: false,
            visible: false,
            ws: 0,
            rect: rect_t {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            },
            buffer: surface_t::empty(),
            statusline_buffer: surface_t::empty(),
            statusline_width: 0,
            bar: surface_t::empty(),
            workspaces: workspace_head.as_ptr(),
            trayclients: trayclient_head.as_ptr(),
            slist: OutputLink {
                sle_next: ptr::null_mut(),
            },
            _workspace_head: workspace_head,
            _trayclient_head: trayclient_head,
        }
    }
}

#[derive(Default)]
struct OutputRecord {
    name: Option<Vec<u8>>,
    active: bool,
    primary: bool,
    ws: i32,
    rect: [i32; 4],
}
#[derive(Default)]
struct OutputReader {
    key: Option<Vec<u8>>,
    current: Option<OutputRecord>,
    records: Vec<OutputRecord>,
    in_rect: bool,
}
struct SharedReader(Rc<RefCell<OutputReader>>);
fn prefix(bytes: &[u8]) -> &[u8] {
    bytes.split(|byte| *byte == 0).next().unwrap()
}

// The string form historically uses locale-sensitive strtol, accepting empty
// strings as zero and wrapping the final C-int conversion. A bounded owned C
// string replaces printf/strlen storage, while retaining those exact rules.
fn workspace_number(value: &[u8]) -> Option<i32> {
    let value = CString::new(prefix(value)).expect("NUL-free prefix");
    let mut end = ptr::null_mut();
    // SAFETY: value is a terminated owned string; end and errno are thread-local
    // writable native storage, and strtol's returned end stays inside value.
    unsafe {
        *libc::__errno_location() = 0;
        let number = libc::strtol(value.as_ptr(), &raw mut end, 10);
        (*libc::__errno_location() == 0 && !end.is_null() && *end == 0).then_some(number as i32)
    }
}
impl EventHandler for SharedReader {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let mut reader = self.0.borrow_mut();
        match event {
            Event::MapKey(key) => reader.key = Some(prefix(key).to_vec()),
            Event::StartMap if reader.key.is_none() => {
                reader.current = Some(OutputRecord::default())
            }
            Event::StartMap => {
                if reader.key.as_deref() == Some(b"rect") {
                    reader.in_rect = true;
                }
            }
            Event::EndMap if reader.in_rect => reader.in_rect = false,
            Event::EndMap => {
                let Some(record) = reader.current.take() else {
                    return false;
                };
                // Missing names formerly reached strcmp(NULL), never a valid output.
                if record.name.is_none() {
                    return false;
                }
                reader.records.push(record);
            }
            Event::Null => reader.key = None,
            Event::Boolean(value) => {
                let key = reader.key.take();
                let Some(record) = reader.current.as_mut() else {
                    return false;
                };
                match key.as_deref() {
                    Some(b"active") => record.active = value,
                    Some(b"primary") => record.primary = value,
                    _ => return false,
                }
            }
            Event::Number(value) if !value.contains(['.', 'e', 'E']) => {
                let Ok(value) = value.parse::<i64>() else {
                    return false;
                };
                if value == i64::MIN {
                    return false;
                }
                let key = reader.key.take();
                let Some(record) = reader.current.as_mut() else {
                    return false;
                };
                let value = value as i32;
                match key.as_deref() {
                    Some(b"current_workspace") => record.ws = value,
                    Some(b"x") => record.rect[0] = value,
                    Some(b"y") => record.rect[1] = value,
                    Some(b"width") => record.rect[2] = value,
                    Some(b"height") => record.rect[3] = value,
                    _ => return false,
                }
            }
            Event::String(value) => {
                let key = reader.key.take();
                let Some(record) = reader.current.as_mut() else {
                    return false;
                };
                match key.as_deref() {
                    Some(b"name") => record.name = Some(prefix(value).to_vec()),
                    Some(b"current_workspace") => {
                        if let Some(value) = workspace_number(value) {
                            record.ws = value;
                        }
                    }
                    _ => return false,
                }
            }
            _ => {}
        }
        true
    }
}
fn read_outputs(json: &[u8]) -> Result<Vec<OutputRecord>, ParseError> {
    let reader = Rc::new(RefCell::new(OutputReader::default()));
    let mut parser = Parser::new(SharedReader(Rc::clone(&reader)));
    parser.options_mut().allow_legacy_whitespace = true;
    // A complete output is published even if the outer array is unfinished;
    // an incomplete output itself is never published.
    parser.parse(json)?;
    drop(parser);
    Ok(Rc::try_unwrap(reader)
        .ok()
        .expect("parser released reader")
        .into_inner()
        .records)
}

pub static mut outputs: *mut outputs_head = ptr::null_mut();
/// # Safety
/// Called once by the event thread before any output-list use.
pub unsafe fn init_outputs() {
    outputs = Box::into_raw(Box::new(outputs_head {
        slh_first: ptr::null_mut(),
    }));
}

/// # Safety
/// The initialized event-thread graph is exclusively modified by this call.
unsafe fn apply_outputs(records: Vec<OutputRecord>) {
    for record in records {
        let mut record = record;
        let name = record.name.take().expect("reader checked name");
        if !config.outputs.is_empty() {
            let accepted = config.outputs.iter().any(|requested| {
                native_name_equal(&name, requested)
                    || native_name_equal(requested, b"primary") && record.primary
                    || native_name_equal(requested, b"nonprimary") && !record.primary
            });
            if !accepted {
                if config.verbose {
                    let mut message = b"[../i3bar/src/outputs.c:205] Ignoring output \"".to_vec();
                    message.extend_from_slice(&name);
                    message.extend_from_slice(b"\", not configured to handle it.\n");
                    libc::fwrite(
                        message.as_ptr().cast(),
                        1,
                        message.len(),
                        i3_common::native_stdio::stdout,
                    );
                }
                continue;
            }
        }
        publish_output(outputs, record, name);
    }
}

// head owns distinct Output boxes and is exclusively accessed on this thread.
unsafe fn publish_output(head: *mut outputs_head, record: OutputRecord, name: Vec<u8>) {
    let existing = find_output(head, &name);
    let target = if existing.is_null() {
        let mut output = Box::new(Output::new(name));
        output.slist.sle_next = (*head).slh_first;
        let output = Box::into_raw(output);
        (*head).slh_first = output;
        output
    } else {
        existing
    };
    (*target).active = record.active;
    (*target).primary = record.primary;
    (*target).ws = record.ws;
    let [x, y, w, h] = record.rect;
    (*target).rect = rect_t { x, y, w, h };
}

/// # Safety
/// json points to size readable bytes. The initialized native output graph and
/// configuration belong to this event thread and have no concurrent borrowers.
pub unsafe fn parse_outputs_json(json: *const u8, size: usize) {
    let json = if size == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(json, size)
    };
    match read_outputs(json) {
        Ok(records) => apply_outputs(records),
        Err(_) => {
            eprintln!("[../i3bar/src/outputs.c:282] ERROR: Could not parse outputs reply!");
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/// # Safety
/// Each output is a distinct Box transferred by apply_outputs. Workspace and
/// tray lists are valid acyclic event-thread allocations. No native operation
/// or iterator retains a node; native window destruction is allowed here.
pub unsafe fn free_outputs() {
    free_workspaces();
    if outputs.is_null() {
        return;
    }
    // Tray teardown sends native notifications that consult the complete
    // output graph. Keep every node live until all native windows are closed.
    let mut output = (*outputs).slh_first;
    while !output.is_null() {
        destroy_window(output);
        output = (*output).slist.sle_next;
    }
    output = (*outputs).slh_first;
    (*outputs).slh_first = ptr::null_mut();
    while !output.is_null() {
        let next = (*output).slist.sle_next;
        let head = (*output).trayclients;
        if !head.is_null() {
            let mut client = (*head).tqh_first;
            (*head).tqh_first = ptr::null_mut();
            (*head).tqh_last = &raw mut (*head).tqh_first;
            while !client.is_null() {
                let next = (*client).tailq.tqe_next;
                libc::free((*client).class_class.cast());
                libc::free((*client).class_instance.cast());
                libc::free(client.cast());
                client = next;
            }
        }
        drop(Box::from_raw(output));
        output = next;
    }
}

/// # Safety
/// name is a live terminated string or null. The initialized event-thread
/// output graph stays live and unmodified while searching.
pub unsafe fn get_output_by_name(name: &[u8]) -> *mut Output {
    if outputs.is_null() {
        return ptr::null_mut();
    }
    find_output(outputs, prefix(name))
}

pub(crate) fn native_name_equal(left: &[u8], right: &[u8]) -> bool {
    let left = CString::new(prefix(left)).expect("bounded prefix");
    let right = CString::new(prefix(right)).expect("bounded prefix");
    // SAFETY: both native strings are owned for this synchronous locale comparison.
    unsafe { libc::strcasecmp(left.as_ptr(), right.as_ptr()) == 0 }
}

// head and its names are live, acyclic, and unmodified during this search.
unsafe fn find_output(head: *mut outputs_head, name: &[u8]) -> *mut Output {
    let primary = native_name_equal(name, b"primary");
    let mut output = (*head).slh_first;
    while !output.is_null() {
        if primary && (*output).primary || (*output).name == name {
            break;
        }
        output = (*output).slist.sle_next;
    }
    output
}

/// # Safety
/// output and its workspace list are live on the event thread, with no mutation.
pub unsafe fn output_has_focus(output: *mut Output) -> bool {
    let mut workspace = (*(*output).workspaces).tqh_first;
    while !workspace.is_null() {
        if (*workspace).focused {
            return true;
        }
        workspace = (*workspace).tailq.tqe_next;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_complete_records_publish_and_names_own_nul_prefixes() {
        let input = br#"[{"name\u0000ignored":"DP-1\u0000ignored","active":true,"rect":{"x":-8,"y":4,"width":1920,"height":1080}},{"name":"partial""#.to_vec();
        let records = read_outputs(&input).unwrap();
        drop(input);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].name.as_deref(), Some(b"DP-1".as_slice()));
        assert_eq!(records[0].rect, [-8, 4, 1920, 1080]);
        assert!(records[0].active);
        assert!(read_outputs(br#"[{"name":"partial""#).unwrap().is_empty());
    }

    #[test]
    fn numeric_callbacks_retain_bounds_wrapping_and_ignored_floats() {
        let records = read_outputs(br#"[{"name":"x","current_workspace":4294967297,"rect":{"x":4294967295,"width":2.5},"ignored":null}]"#).unwrap();
        assert_eq!(records[0].ws, 1);
        assert_eq!(records[0].rect, [-1, 0, 0, 0]);
        for input in [
            br#"[{"name":"x","current_workspace":9223372036854775808}]"#.as_slice(),
            br#"[{"name":"x","current_workspace":-9223372036854775808}]"#,
            br#"[{"name":"x","unknown":1}]"#,
            br#"[{"name":"x","unknown":true}]"#,
            br#"[{"name":"x","unknown":"value"}]"#,
            br#"[{}]"#,
        ] {
            assert!(read_outputs(input).is_err(), "{input:?}");
        }
    }

    #[test]
    fn string_workspace_numbers_preserve_strtol_rules_and_failed_updates() {
        for (value, expected) in [
            (b"".as_slice(), Some(0)),
            (b" \t+12", Some(12)),
            (b"12 ", None),
            (b"word", None),
            (b"4294967297", Some(1)),
            (b"9223372036854775808", None),
            (b"9\0ignored", Some(9)),
        ] {
            assert_eq!(workspace_number(value), expected);
        }
        let records =
            read_outputs(br#"[{"name":"x","current_workspace":7,"current_workspace":"invalid"}]"#)
                .unwrap();
        assert_eq!(records[0].ws, 7);
    }

    #[test]
    fn updates_preserve_node_identity_order_and_owned_head_addresses() {
        let mut head = outputs_head {
            slh_first: ptr::null_mut(),
        };
        let records = read_outputs(br#"[{"name":"one","primary":true},{"name":"two"},{"name":"one","active":true,"current_workspace":3}]"#).unwrap();
        unsafe {
            let mut first_node: *mut Output = ptr::null_mut();
            for mut record in records {
                let name = record.name.take().unwrap();
                publish_output(&raw mut head, record, name);
                if first_node.is_null() {
                    first_node = head.slh_first;
                }
            }
            let two = head.slh_first;
            assert_eq!((*two).name.as_slice(), b"two".as_slice());
            assert_eq!((*two).slist.sle_next, first_node);
            assert!((*first_node).active);
            assert!(!(*first_node).primary);
            assert_eq!((*first_node).ws, 3);
            // Moving the Output box or owner never moves either head allocation.
            assert_eq!((*two).workspaces, (*two)._workspace_head.as_ptr());
            assert_eq!(
                (*(*two).workspaces).tqh_last,
                &raw mut (*(*two).workspaces).tqh_first
            );
            assert_eq!((*two).trayclients, (*two)._trayclient_head.as_ptr());
            assert_eq!(
                (*(*two).trayclients).tqh_last,
                &raw mut (*(*two).trayclients).tqh_first
            );
            drop(Box::from_raw(two));
            drop(Box::from_raw(first_node));
        }
    }
}
