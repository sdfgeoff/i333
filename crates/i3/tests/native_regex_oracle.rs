#![feature(c_variadic)]
#![allow(dead_code, stable_features)]
#[path = "../src/regex.rs"]
mod regex;
mod log {
    pub unsafe extern "C" fn errorlog(_: *mut core::ffi::c_char, _: ...) {}
    pub unsafe extern "C" fn verboselog(_: *mut core::ffi::c_char, _: ...) {}
}
fn unhex(value: &str) -> Vec<u8> {
    (0..value.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&value[i..i + 2], 16).unwrap())
        .collect()
}
fn hex(value: &[u8]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}
fn probe_main() {
    use std::io::BufRead;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (pattern, subject) = line.split_once('\t').unwrap();
        match regex::Regex::compile_bytes(&unhex(pattern)) {
            Err(error) => println!(
                "compile_error\t{}\t{}\t{}",
                error.offset,
                error
                    .code
                    .map(|code| code.to_string())
                    .unwrap_or_else(|| "none".to_owned()),
                hex(error.message.as_bytes())
            ),
            Ok(expression) => match expression.is_match_bytes(&unhex(subject)) {
                Ok(matched) => println!("match\t{}", u8::from(matched)),
                Err(error) => println!("match_error\t{}", error.code()),
            },
        }
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
    i3_test_support::run_python("test-regex-parity.py", &args);
}
