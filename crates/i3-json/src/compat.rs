//! Transitional adapters for the file-by-file port's callback-based JSON users.
//!
//! These are ordinary Rust functions, not native-library bindings. New callers
//! should own [`Generator`] or [`Parser`] directly. Pointer/context invariants
//! are confined here while existing application files migrate independently.
#![allow(non_camel_case_types, non_upper_case_globals)]
use crate::{Event, EventHandler, GenerateError, Generator, ParseErrorKind, Parser};
use std::{
    ffi::{c_char, c_double, c_int, c_longlong, c_uchar, c_uint, c_void, CString},
    ptr, slice,
};

#[repr(C)]
pub struct yajl_gen_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct yajl_handle_t {
    _private: [u8; 0],
}
pub type yajl_gen = *mut yajl_gen_t;
pub type yajl_handle = *mut yajl_handle_t;
pub type yajl_gen_status = c_uint;
pub type yajl_status = c_uint;
pub type yajl_option = c_uint;
pub type yajl_malloc_func = Option<unsafe fn(*mut c_void, usize) -> *mut c_void>;
pub type yajl_realloc_func = Option<unsafe fn(*mut c_void, *mut c_void, usize) -> *mut c_void>;
pub type yajl_free_func = Option<unsafe fn(*mut c_void, *mut c_void)>;
#[derive(Clone, Copy)]
pub struct yajl_alloc_funcs {
    pub malloc: yajl_malloc_func,
    pub realloc: yajl_realloc_func,
    pub free: yajl_free_func,
    pub ctx: *mut c_void,
}

pub const yajl_gen_status_ok: yajl_gen_status = 0;
pub const yajl_gen_keys_must_be_strings: yajl_gen_status = 1;
pub const yajl_max_depth_exceeded: yajl_gen_status = 2;
pub const yajl_gen_in_error_state: yajl_gen_status = 3;
pub const yajl_gen_generation_complete: yajl_gen_status = 4;
pub const yajl_gen_invalid_number: yajl_gen_status = 5;
pub const yajl_gen_no_buf: yajl_gen_status = 6;
pub const yajl_gen_invalid_string: yajl_gen_status = 7;
pub const yajl_status_ok: yajl_status = 0;
pub const yajl_status_client_canceled: yajl_status = 1;
pub const yajl_status_error: yajl_status = 2;
pub const yajl_allow_comments: yajl_option = 1;
pub const yajl_dont_validate_strings: yajl_option = 2;
pub const yajl_allow_trailing_garbage: yajl_option = 4;
pub const yajl_allow_multiple_values: yajl_option = 8;
pub const yajl_allow_partial_values: yajl_option = 16;

#[derive(Clone, Copy, Default)]
pub struct yajl_callbacks {
    pub yajl_null: Option<unsafe fn(*mut c_void) -> c_int>,
    pub yajl_boolean: Option<unsafe fn(*mut c_void, c_int) -> c_int>,
    pub yajl_integer: Option<unsafe fn(*mut c_void, c_longlong) -> c_int>,
    pub yajl_double: Option<unsafe fn(*mut c_void, c_double) -> c_int>,
    pub yajl_number: Option<unsafe fn(*mut c_void, *const c_char, usize) -> c_int>,
    pub yajl_string: Option<unsafe fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_start_map: Option<unsafe fn(*mut c_void) -> c_int>,
    pub yajl_map_key: Option<unsafe fn(*mut c_void, *const c_uchar, usize) -> c_int>,
    pub yajl_end_map: Option<unsafe fn(*mut c_void) -> c_int>,
    pub yajl_start_array: Option<unsafe fn(*mut c_void) -> c_int>,
    pub yajl_end_array: Option<unsafe fn(*mut c_void) -> c_int>,
}
struct Callbacks {
    callbacks: yajl_callbacks,
    context: *mut c_void,
    error: Option<String>,
}
impl EventHandler for Callbacks {
    fn on_event(&mut self, event: Event<'_>) -> bool {
        let c = &self.callbacks;
        // SAFETY: The adapter's constructor requires callback/context lifetimes
        // to extend until the parser is freed. All byte arguments live through
        // the synchronous call, including borrowed decoded-string storage.
        let result = unsafe {
            match event {
                Event::Null => c.yajl_null.map(|f| f(self.context)).unwrap_or(1),
                Event::Boolean(value) => c
                    .yajl_boolean
                    .map(|f| f(self.context, value.into()))
                    .unwrap_or(1),
                Event::String(value) => c
                    .yajl_string
                    .map(|f| f(self.context, value.as_ptr(), value.len()))
                    .unwrap_or(1),
                Event::MapKey(value) => c
                    .yajl_map_key
                    .map(|f| f(self.context, value.as_ptr(), value.len()))
                    .unwrap_or(1),
                Event::StartMap => c.yajl_start_map.map(|f| f(self.context)).unwrap_or(1),
                Event::EndMap => c.yajl_end_map.map(|f| f(self.context)).unwrap_or(1),
                Event::StartArray => c.yajl_start_array.map(|f| f(self.context)).unwrap_or(1),
                Event::EndArray => c.yajl_end_array.map(|f| f(self.context)).unwrap_or(1),
                Event::Number(value) => {
                    if let Some(f) = c.yajl_number {
                        f(self.context, value.as_ptr().cast(), value.len())
                    } else if value.contains(['.', 'e', 'E']) {
                        if let Some(f) = c.yajl_double {
                            match value.parse::<f64>() {
                                Ok(number) if number.is_finite() => f(self.context, number),
                                _ => {
                                    self.error = Some(
                                        "parse error: numeric (floating point) overflow".into(),
                                    );
                                    0
                                }
                            }
                        } else {
                            1
                        }
                    } else if let Some(f) = c.yajl_integer {
                        match value.parse::<i64>() {
                            // YAJL accumulates the magnitude before applying the
                            // sign, so its legacy integer callback rejects MIN.
                            Ok(number) if number != i64::MIN => f(self.context, number),
                            _ => {
                                self.error = Some("parse error: integer overflow".into());
                                0
                            }
                        }
                    } else {
                        1
                    }
                }
            }
        };
        result != 0
    }
    fn error_message(&self) -> Option<&str> {
        self.error.as_deref()
    }
}
fn status(result: Result<(), crate::ParseError>) -> yajl_status {
    match result {
        Ok(()) => yajl_status_ok,
        Err(error) if error.kind == ParseErrorKind::Canceled => yajl_status_client_canceled,
        Err(_) => yajl_status_error,
    }
}
fn generate_status(result: Result<(), GenerateError>) -> yajl_gen_status {
    match result {
        Ok(()) => yajl_gen_status_ok,
        Err(GenerateError::KeysMustBeStrings) => yajl_gen_keys_must_be_strings,
        Err(GenerateError::DepthExceeded) => yajl_max_depth_exceeded,
        Err(GenerateError::Complete) => yajl_gen_generation_complete,
        Err(GenerateError::InvalidNumber) => yajl_gen_invalid_number,
        Err(GenerateError::InvalidString) => yajl_gen_invalid_string,
        Err(GenerateError::InvalidState) => yajl_gen_in_error_state,
    }
}

/// # Safety
/// Callback context and any references used by callbacks must remain live until
/// `yajl_free`. Non-null custom allocators are unsupported by the Rust backend.
pub unsafe fn yajl_alloc(
    callbacks: *const yajl_callbacks,
    allocators: *mut yajl_alloc_funcs,
    context: *mut c_void,
) -> yajl_handle {
    if !allocators.is_null() {
        return ptr::null_mut();
    }
    let callbacks = if callbacks.is_null() {
        yajl_callbacks::default()
    } else {
        unsafe { *callbacks }
    };
    let mut parser = Parser::new(Callbacks {
        callbacks,
        context,
        error: None,
    });
    parser.options_mut().allow_legacy_whitespace = true;
    Box::into_raw(Box::new(parser)).cast()
}
/// # Safety
/// The handle is a live allocation returned by `yajl_alloc`, freed once.
pub unsafe fn yajl_free(handle: yajl_handle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle.cast::<Parser>()));
        }
    }
}
/// # Safety
/// Handle and input are live; input has `length` initialized bytes.
pub unsafe fn yajl_parse(handle: yajl_handle, input: *const u8, length: usize) -> yajl_status {
    let input = if length == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(input, length) }
    };
    status(unsafe { &mut *handle.cast::<Parser>() }.parse(input))
}
/// # Safety
/// Handle is live and uniquely borrowed for this synchronous call.
pub unsafe fn yajl_complete_parse(handle: yajl_handle) -> yajl_status {
    status(unsafe { &mut *handle.cast::<Parser>() }.finish())
}
/// # Safety
/// Handle is live and uniquely borrowed for this synchronous call.
pub unsafe fn yajl_config(handle: yajl_handle, option: yajl_option, value: c_int) -> c_int {
    let options = unsafe { &mut *handle.cast::<Parser>() }.options_mut();
    let value = value != 0;
    match option {
        yajl_allow_comments => options.allow_comments = value,
        yajl_dont_validate_strings => options.allow_invalid_utf8 = value,
        yajl_allow_multiple_values => options.allow_multiple_values = value,
        yajl_allow_partial_values => options.allow_partial_values = value,
        yajl_allow_trailing_garbage => options.allow_trailing_garbage = value,
        _ => return 0,
    }
    1
}
/// # Safety
/// Handle is live for the duration of the call.
pub unsafe fn yajl_get_bytes_consumed(handle: yajl_handle) -> usize {
    unsafe { &*handle.cast::<Parser>() }.bytes_consumed()
}
/// # Safety
/// Handle is live. The returned C string is owned and must be freed with `yajl_free_error`.
pub unsafe fn yajl_get_error(
    handle: yajl_handle,
    _verbose: c_int,
    _input: *const u8,
    _length: usize,
) -> *mut u8 {
    let error = unsafe { &*handle.cast::<Parser>() }
        .error()
        .map(ToString::to_string)
        .unwrap_or_else(|| "JSON parse error".into());
    CString::new(format!("{error}\n"))
        .expect("parser messages contain no NUL")
        .into_raw()
        .cast()
}
/// # Safety
/// Error is null or an owned pointer returned once by `yajl_get_error`.
pub unsafe fn yajl_free_error(_handle: yajl_handle, error: *mut u8) {
    if !error.is_null() {
        unsafe {
            drop(CString::from_raw(error.cast()));
        }
    }
}

pub unsafe fn yajl_gen_alloc(allocators: *const yajl_alloc_funcs) -> yajl_gen {
    if !allocators.is_null() {
        return ptr::null_mut();
    }
    Box::into_raw(Box::new(Generator::new())).cast()
}
pub unsafe fn yajl_gen_free(handle: yajl_gen) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle.cast::<Generator>()));
        }
    }
}
pub unsafe fn yajl_gen_integer(handle: yajl_gen, value: c_longlong) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.integer(value))
}
pub unsafe fn yajl_gen_double(handle: yajl_gen, value: c_double) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.double(value))
}
pub unsafe fn yajl_gen_bool(handle: yajl_gen, value: c_int) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.boolean(value != 0))
}
pub unsafe fn yajl_gen_null(handle: yajl_gen) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.null())
}
pub unsafe fn yajl_gen_string(
    handle: yajl_gen,
    value: *const u8,
    length: usize,
) -> yajl_gen_status {
    let bytes = if length == 0 {
        &[]
    } else {
        unsafe { slice::from_raw_parts(value, length) }
    };
    match std::str::from_utf8(bytes) {
        Ok(value) => generate_status(unsafe { &mut *handle.cast::<Generator>() }.string(value)),
        Err(_) => yajl_gen_invalid_string,
    }
}
pub unsafe fn yajl_gen_array_open(handle: yajl_gen) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.start_array())
}
pub unsafe fn yajl_gen_array_close(handle: yajl_gen) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.end_array())
}
pub unsafe fn yajl_gen_map_open(handle: yajl_gen) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.start_object())
}
pub unsafe fn yajl_gen_map_close(handle: yajl_gen) -> yajl_gen_status {
    generate_status(unsafe { &mut *handle.cast::<Generator>() }.end_object())
}
pub unsafe fn yajl_gen_clear(handle: yajl_gen) {
    unsafe { &mut *handle.cast::<Generator>() }.clear();
}
pub unsafe fn yajl_gen_get_buf(
    handle: yajl_gen,
    output: *mut *const u8,
    length: *mut usize,
) -> yajl_gen_status {
    let generator = unsafe { &mut *handle.cast::<Generator>() };
    let size = generator.bytes().len();
    let terminated = generator.terminated_bytes();
    unsafe {
        *output = terminated.as_ptr();
        *length = size;
    }
    yajl_gen_status_ok
}

#[cfg(test)]
mod tests {
    use super::*;
    unsafe fn integer(_context: *mut c_void, _value: i64) -> i32 {
        1
    }
    unsafe fn number(context: *mut c_void, _value: *const c_char, _length: usize) -> i32 {
        unsafe {
            *context.cast::<usize>() += 1;
        }
        1
    }
    #[test]
    fn raw_number_callback_has_priority_and_preserves_large_values() {
        let mut count = 0_usize;
        let callbacks = yajl_callbacks {
            yajl_number: Some(number),
            yajl_integer: Some(integer),
            ..Default::default()
        };
        unsafe {
            let parser = yajl_alloc(
                &callbacks,
                ptr::null_mut(),
                ptr::from_mut(&mut count).cast(),
            );
            let bytes = b"[-9223372036854775808,9223372036854775808,1e400]";
            assert_eq!(
                yajl_parse(parser, bytes.as_ptr(), bytes.len()),
                yajl_status_ok
            );
            assert_eq!(yajl_complete_parse(parser), yajl_status_ok);
            assert_eq!(count, 3);
            yajl_free(parser);
        }
    }
    #[test]
    fn integer_overflow_is_a_parse_error_instead_of_callback_cancellation() {
        let callbacks = yajl_callbacks {
            yajl_integer: Some(integer),
            ..Default::default()
        };
        for bytes in [
            b"9223372036854775808 ".as_slice(),
            b"-9223372036854775808 ".as_slice(),
        ] {
            unsafe {
                let parser = yajl_alloc(&callbacks, ptr::null_mut(), ptr::null_mut());
                assert_eq!(
                    yajl_parse(parser, bytes.as_ptr(), bytes.len()),
                    yajl_status_error
                );
                let error = yajl_get_error(parser, 0, bytes.as_ptr(), bytes.len());
                assert!(!error.is_null());
                yajl_free_error(parser, error);
                yajl_free(parser);
            }
        }
    }
}
