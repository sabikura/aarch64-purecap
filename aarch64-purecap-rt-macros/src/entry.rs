use grant::Grant;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::parse::{Error, Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse_macro_input, FnArg, ItemFn, ReturnType, Type};

mod grant;

/// Emit the grant items, the user function and a `__aarch64_purecap_rt_main` entry that calls
/// the user function with the `Grant` instance. The input TokenStream must be a function that
/// returns never, is generic over nothing and takes one `Grant` argument.
pub fn process(args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as ItemFn);
    if !has_valid_signature(&fun) {
        return Error::new(fun.span(), "Function signature is not supported")
            .into_compile_error()
            .into();
    }

    let grant = parse_macro_input!(args as Grant);
    let grant_struct = grant.struct_();
    let grant_instance = grant.instance();
    let fun_ident = &fun.sig.ident;
    let ident = Ident::new("__aarch64_purecap_rt_main", Span::call_site());

    quote! {
        #grant_struct

        #fun

        #[doc(hidden)]
        #[no_mangle]
        #[link_section = ".text"]
        pub unsafe extern "C" fn #ident() -> ! {
            let grant = #grant_instance;
            ::aarch64_purecap_rt::__null_ddc();
            #fun_ident(grant)
        }
    }
    .into()
}

/// Check if the input function has the right signature for the [`entry`] macro (one `Grant`
/// argument, no generics, not variadic and the return type is ! =never)
fn has_valid_signature(fun: &ItemFn) -> bool {
    fn is_never(ret: &ReturnType) -> bool {
        match ret {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        }
    }

    fn is_grant(arg: Option<&FnArg>) -> bool {
        let Some(FnArg::Typed(pat)) = arg else {
            return false;
        };
        matches!(&*pat.ty, Type::Path(path) if path.qself.is_none() && path.path.is_ident("Grant"))
    }

    fun.sig.inputs.len() == 1
        && is_grant(fun.sig.inputs.first())
        && fun.sig.generics.where_clause.is_none()
        && fun.sig.generics.params.is_empty()
        && fun.sig.variadic.is_none()
        && is_never(&fun.sig.output)
}
