use grant::Grant;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, ItemFn, ReturnType, Type};

mod grant;

/// Return a TokenStream containing that modifies the input function's identifier
/// to `__aarch64_purecap_rt_main`. The input TokenStream must be a function that returns never,
/// is generic over nothing and has no arguments.
pub fn process(args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as ItemFn);
    if !has_valid_signature(&fun) {
        return Error::new(fun.span(), "Function signature is not supported")
            .into_compile_error()
            .into();
    }

    let grant = parse_macro_input!(args as Grant);
    let grant_struct = match grant.struct_() {
        Ok(tokens) => tokens,
        Err(err) => return err.into_compile_error().into(),
    };

    let block = fun.block;
    let ident = Ident::new("__aarch64_purecap_rt_main", Span::call_site());

    quote! {
        #grant_struct

        #[doc(hidden)]
        #[no_mangle]
        #[link_section = ".text"]
        pub unsafe extern "C" fn #ident() -> ! {
            #block
        }
    }
    .into()
}

/// Check if the input function has the right signature for the [`entry`] macro (no arguments, no
/// generics, not variadic and the return type is ! =never)
fn has_valid_signature(fun: &ItemFn) -> bool {
    fn is_never(ret: &ReturnType) -> bool {
        match ret {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        }
    }

    fun.sig.inputs.is_empty()
        && fun.sig.generics.where_clause.is_none()
        && fun.sig.generics.params.is_empty()
        && fun.sig.variadic.is_none()
        && is_never(&fun.sig.output)
}
