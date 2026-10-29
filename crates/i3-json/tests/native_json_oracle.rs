use i3_json::{Event, EventHandler, Generator, ParseError, ParseErrorKind, Parser, ParserOptions};
use serde_json::{json, Value};
use std::{
    cell::RefCell,
    io::{self, BufRead},
    rc::Rc,
};
fn unhex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap())
        .collect()
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|b| format!("{b:02x}")).collect()
}
struct Recorder {
    events: Rc<RefCell<Vec<Value>>>,
    mode: String,
    error: Option<String>,
    cancel: usize,
}
impl EventHandler for Recorder {
    fn on_event(&mut self, e: Event<'_>) -> bool {
        let v = match e {
            Event::Null => json!(["null"]),
            Event::Boolean(v) => json!(["bool", v]),
            Event::String(v) => json!(["string", hex(v)]),
            Event::MapKey(v) => json!(["key", hex(v)]),
            Event::StartMap => json!(["map"]),
            Event::EndMap => json!(["end_map"]),
            Event::StartArray => json!(["array"]),
            Event::EndArray => json!(["end_array"]),
            Event::Number(v) => {
                if self.mode == "none" {
                    return true;
                }
                if self.mode == "raw" {
                    json!(["number", v])
                } else if v.contains(['.', 'e', 'E']) {
                    match v.parse::<f64>() {
                        Ok(n) if n.is_finite() => json!(["double", n]),
                        _ => {
                            self.error = Some("numeric overflow".into());
                            return false;
                        }
                    }
                } else {
                    match v.parse::<i64>() {
                        Ok(n) if n != i64::MIN => json!(["integer", n]),
                        _ => {
                            self.error = Some("integer overflow".into());
                            return false;
                        }
                    }
                }
            }
        };
        self.events.borrow_mut().push(v);
        self.events.borrow().len() != self.cancel
    }
    fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
fn status(r: Result<(), ParseError>) -> u32 {
    match r {
        Ok(()) => 0,
        Err(e) if e.kind == ParseErrorKind::Canceled => 1,
        _ => 2,
    }
}
fn parse(v: &Value) -> Value {
    let events = Rc::new(RefCell::new(vec![]));
    let rec = Recorder {
        events: events.clone(),
        mode: v["mode"].as_str().unwrap_or("raw").into(),
        error: None,
        cancel: v["cancel"].as_u64().unwrap_or(0) as usize,
    };
    let mut p = Parser::new(rec);
    let flags = v["flags"].as_u64().unwrap_or(0);
    *p.options_mut() = ParserOptions {
        allow_comments: flags & 1 != 0,
        allow_invalid_utf8: flags & 2 != 0,
        allow_trailing_garbage: flags & 4 != 0,
        allow_multiple_values: flags & 8 != 0,
        allow_legacy_whitespace: true,
        allow_partial_values: flags & 16 != 0,
    };
    let mut steps = vec![];
    let mut last = 0;
    for chunk in v["chunks"].as_array().unwrap() {
        last = status(p.parse(&unhex(chunk.as_str().unwrap())));
        steps.push(
            json!({"status":last,"consumed":p.bytes_consumed(),"events":events.borrow().clone()}),
        );
        if last != 0 {
            break;
        }
    }
    let finish = if v["finish"].as_bool().unwrap_or(true) {
        status(p.finish())
    } else {
        last
    };
    json!({"steps":steps,"finish":finish,"events":events.borrow().clone()})
}
fn generate(v: &Value) -> Value {
    let mut g = Generator::new();
    let mut states = vec![];
    for op in v["ops"].as_array().unwrap() {
        let name = op[0].as_str().unwrap();
        let r = match name {
            "array" => g.start_array(),
            "end_array" => g.end_array(),
            "map" => g.start_object(),
            "end_map" => g.end_object(),
            "null" => g.null(),
            "bool" => g.boolean(op[1].as_bool().unwrap()),
            "integer" => g.integer(op[1].as_i64().unwrap()),
            "double" => g.double(op[1].as_str().unwrap().parse().unwrap()),
            "string" => g.string(op[1].as_str().unwrap()),
            "clear" => {
                g.clear();
                Ok(())
            }
            _ => panic!(),
        };
        let status = match r {
            Ok(()) => 0,
            Err(i3_json::GenerateError::KeysMustBeStrings) => 1,
            Err(i3_json::GenerateError::DepthExceeded) => 2,
            Err(i3_json::GenerateError::InvalidState) => 3,
            Err(i3_json::GenerateError::Complete) => 4,
            Err(i3_json::GenerateError::InvalidNumber) => 5,
            Err(i3_json::GenerateError::InvalidString) => 7,
        };
        states.push(json!({"status":status,"bytes":hex(g.bytes())}));
    }
    json!(states)
}
fn probe_main() {
    for line in io::stdin().lock().lines() {
        let v: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let out = if v.get("ops").is_some() {
            generate(&v)
        } else {
            parse(&v)
        };
        println!("{out}");
    }
}

fn main() {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--probe")) {
        probe_main();
        return;
    }
    let args = vec![
        std::ffi::OsString::from("--require-oracle"),
        std::ffi::OsString::from("--probe"),
        std::env::current_exe()
            .expect("current test executable")
            .into_os_string(),
    ];
    i3_test_support::run_python("test-json-parity.py", &args);
}
