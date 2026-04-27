extern crate proc_macro;

mod entry;
mod exception;

/// The entry function of the `aarch64-purecap-rt` runtime.
#[proc_macro_attribute]
pub fn entry(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    entry::process(args, input)
}

/// Define an exception handler for the `aarch64-purecap-rt` runtime.
#[proc_macro_attribute]
pub fn exception(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    exception::process(args, input)
}
