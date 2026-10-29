// Rust 1.99 merged VaListImpl and the borrowed VaList into one owned VaList.
// Keep the C translation's explicit clone() calls as va_copy. At forwarding
// sites, move that owned cursor on newer Rust; reborrow it on older Rust.
// No extra clone here: it would silently change which cursor C consumes.
#[macro_export]
macro_rules! i333_va_list {
    ($args:expr) => {{
        #[cfg(i333_new_va_list)]
        {
            $args
        }
        #[cfg(not(i333_new_va_list))]
        {
            $args.as_va_list()
        }
    }};
}
