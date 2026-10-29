// libtest discards #[test] functions in this harness-free probe; its included
// unit-test module therefore has no remaining use for its import.
#[allow(unused_imports)]
#[path = "../src/parse_json_header.rs"]
mod header;
use std::io::{self, BufRead};
fn probe_main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let bytes: Vec<u8> = line
            .as_bytes()
            .chunks(2)
            .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
            .collect();
        let h = header::parse_json_header(&bytes);
        println!(
            "{} {} {} {} {}",
            h.version,
            h.stop_signal,
            h.cont_signal,
            h.click_events
                .map(|v| if v { "1" } else { "0" })
                .unwrap_or("-"),
            h.consumed
        );
    }
}

fn main() {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--probe")) {
        probe_main();
        return;
    }
    let args = vec![
        std::ffi::OsString::from("--probe"),
        std::env::current_exe()
            .expect("current test executable")
            .into_os_string(),
    ];
    i3_test_support::run_python("test-header-parity.py", &args);
}
