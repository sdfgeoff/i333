/*
 * vim:ts=4:sw=4:expandtab
 *
 * i3 - an improved tiling window manager
 * © 2009 Michael Stapelberg and contributors (see also: LICENSE)
 *
 * regex.c: Interface to libPCRE (perl compatible regular expressions).
 *
 */
// Mechanical file-by-file Rust translation of src/regex.c.
// Generated with C2Rust 0.22.1; C ABI and original ownership retained.
extern "C" {
    pub type pcre2_real_general_context_8;
    pub type pcre2_real_compile_context_8;
    pub type pcre2_real_match_context_8;
    pub type pcre2_real_code_8;
    pub type pcre2_real_match_data_8;
    fn verboselog(fmt: *mut ::core::ffi::c_char, ...);
    fn errorlog(fmt: *mut ::core::ffi::c_char, ...);
    fn scalloc(num: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn sstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn pcre2_compile_8(
        _: PCRE2_SPTR8,
        _: size_t,
        _: uint32_t,
        _: *mut ::core::ffi::c_int,
        _: *mut size_t,
        _: *mut pcre2_compile_context_8,
    ) -> *mut pcre2_code_8;
    fn pcre2_match_data_create_from_pattern_8(
        _: *const pcre2_code_8,
        _: *mut pcre2_general_context_8,
    ) -> *mut pcre2_match_data_8;
    fn pcre2_match_data_free_8(_: *mut pcre2_match_data_8);
    fn pcre2_match_8(
        _: *const pcre2_code_8,
        _: PCRE2_SPTR8,
        _: size_t,
        _: size_t,
        _: uint32_t,
        _: *mut pcre2_match_data_8,
        _: *mut pcre2_match_context_8,
    ) -> ::core::ffi::c_int;
    fn pcre2_get_error_message_8(
        _: ::core::ffi::c_int,
        _: *mut PCRE2_UCHAR8,
        _: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type PCRE2_UCHAR8 = uint8_t;
pub type PCRE2_SPTR8 = *const PCRE2_UCHAR8;
pub type pcre2_general_context_8 = pcre2_real_general_context_8;
pub type pcre2_compile_context_8 = pcre2_real_compile_context_8;
pub type pcre2_match_context_8 = pcre2_real_match_context_8;
pub type pcre2_code_8 = pcre2_real_code_8;
pub type pcre2_match_data_8 = pcre2_real_match_data_8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: *mut ::core::ffi::c_char,
    pub regex: *mut pcre2_code_8,
}
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PCRE2_UCP: ::core::ffi::c_uint = 0x20000 as ::core::ffi::c_uint;
pub const PCRE2_UTF: ::core::ffi::c_uint = 0x80000 as ::core::ffi::c_uint;
pub const PCRE2_ERROR_NOMATCH: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PCRE2_ZERO_TERMINATED: size_t = !(0 as ::core::ffi::c_int as size_t);
#[no_mangle]
pub unsafe extern "C" fn regex_new(mut pattern: *const ::core::ffi::c_char) -> *mut regex {
    let mut errorcode: ::core::ffi::c_int = 0;
    let mut offset: size_t = 0;
    let mut re: *mut regex =
        scalloc(1 as size_t, ::core::mem::size_of::<regex>() as size_t) as *mut regex;
    (*re).pattern = sstrdup(pattern);
    let mut options: uint32_t = PCRE2_UTF as uint32_t;
    options = (options as ::core::ffi::c_uint | PCRE2_UCP) as uint32_t;
    (*re).regex = pcre2_compile_8(
        pattern as PCRE2_SPTR8,
        PCRE2_ZERO_TERMINATED,
        options,
        &raw mut errorcode,
        &raw mut offset,
        ::core::ptr::null_mut::<pcre2_compile_context_8>(),
    );
    if (*re).regex.is_null() {
        let mut buffer: [PCRE2_UCHAR8; 256] = [0; 256];
        pcre2_get_error_message_8(
            errorcode,
            &raw mut buffer as *mut PCRE2_UCHAR8,
            ::core::mem::size_of::<[PCRE2_UCHAR8; 256]>() as size_t,
        );
        errorlog(
            b"ERROR: PCRE regular expression compilation failed at %zu: %s\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            offset,
            &raw mut buffer as *mut PCRE2_UCHAR8,
        );
        regex_free(re);
        return ::core::ptr::null_mut::<regex>();
    }
    return re;
}
#[no_mangle]
pub unsafe extern "C" fn regex_free(mut regex: *mut regex) {
    if regex.is_null() {
        return;
    }
    free((*regex).pattern as *mut ::core::ffi::c_void);
    (*regex).pattern = ::core::ptr::null_mut::<::core::ffi::c_char>();
    free((*regex).regex as *mut ::core::ffi::c_void);
    (*regex).regex = ::core::ptr::null_mut::<pcre2_code_8>();
    free(regex as *mut ::core::ffi::c_void);
    regex = ::core::ptr::null_mut::<regex>();
}
#[no_mangle]
pub unsafe extern "C" fn regex_matches(
    mut regex: *const regex,
    mut input: *const ::core::ffi::c_char,
) -> bool {
    let mut match_data: *mut pcre2_match_data_8 = pcre2_match_data_create_from_pattern_8(
        (*regex).regex,
        ::core::ptr::null_mut::<pcre2_general_context_8>(),
    );
    let rc: ::core::ffi::c_int = pcre2_match_8(
        (*regex).regex,
        input as PCRE2_SPTR8,
        strlen(input),
        0 as size_t,
        0 as uint32_t,
        match_data,
        ::core::ptr::null_mut::<pcre2_match_context_8>(),
    ) as ::core::ffi::c_int;
    pcre2_match_data_free_8(match_data);
    if rc > 0 as ::core::ffi::c_int {
        verboselog(
            b"Regular expression \"%s\" matches \"%s\"\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*regex).pattern,
            input,
        );
        return true_0 != 0;
    }
    if rc == PCRE2_ERROR_NOMATCH {
        verboselog(
            b"Regular expression \"%s\" does not match \"%s\"\n\0" as *const u8
                as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            (*regex).pattern,
            input,
        );
        return false_0 != 0;
    }
    errorlog(
        b"ERROR: PCRE error %d while trying to use regular expression \"%s\" on input \"%s\", see pcreapi(3)\n\0"
            as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        rc,
        (*regex).pattern,
        input,
    );
    return false_0 != 0;
}
