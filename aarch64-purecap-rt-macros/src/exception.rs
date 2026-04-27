use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Ident, ItemFn, ReturnType};

pub fn process(args: TokenStream, input: TokenStream) -> TokenStream {
    let fun = parse_macro_input!(input as ItemFn);
    if !has_valid_signature(&fun) {
        return syn::Error::new(fun.span(), "Function signature is not supported")
            .into_compile_error()
            .into();
    }

    let args = parse_macro_input!(args as ExceptionArgs);
    let ident = Ident::from(args);
    let block = fun.block;

    quote! {
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = ".text"]
        pub unsafe extern "C" fn #ident() {
            #block
        }
    }
    .into()
}

/// Check if the input function has the right signature for the [`exception`] macro
/// (no arguments, no generics, not variadic and the return type is default)
fn has_valid_signature(fun: &ItemFn) -> bool {
    fun.sig.inputs.is_empty()
        && fun.sig.generics.where_clause.is_none()
        && fun.sig.generics.params.is_empty()
        && fun.sig.variadic.is_none()
        && matches!(fun.sig.output, ReturnType::Default)
}

/// Type of exceptions
enum Exception {
    Irq,
    Sync,
}

impl TryFrom<Ident> for Exception {
    type Error = syn::Error;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "Irq" | "irq" | "IRQ" => Ok(Exception::Irq),
            "Sync" | "sync" | "SYNC" => Ok(Exception::Sync),
            _ => Err(syn::Error::new(
                value.span(),
                "expected exception type Irq or Sync",
            )),
        }
    }
}

/// Type of exception levels
enum ExceptionLevel {
    El0,
    El1,
}

impl TryFrom<Ident> for ExceptionLevel {
    type Error = syn::Error;
    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            "El0" | "el0" | "EL0" => Ok(ExceptionLevel::El0),
            "El1" | "el1" | "EL1" => Ok(ExceptionLevel::El1),
            _ => Err(syn::Error::new(
                value.span(),
                "expected exception level El0 or El1",
            )),
        }
    }
}

/// Supported arguments for the `exception` macro
struct ExceptionArgs {
    exception: Exception,
    level: ExceptionLevel,
}

impl syn::parse::Parse for ExceptionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exception_ident: syn::Ident = input.parse()?;
        let exception = Exception::try_from(exception_ident)?;
        input.parse::<syn::token::Comma>()?;
        let el_ident: syn::Ident = input.parse()?;
        let level = ExceptionLevel::try_from(el_ident)?;

        Ok(Self { exception, level })
    }
}

impl From<ExceptionArgs> for Ident {
    fn from(value: ExceptionArgs) -> Self {
        let exception = match value.exception {
            Exception::Irq => "irq",
            Exception::Sync => "sync",
        };
        let level = match value.level {
            ExceptionLevel::El0 => "el0",
            ExceptionLevel::El1 => "el1",
        };

        // Format is __aarch64_purecap_rt_{el}_{exc}
        let ident = format!("__aarch64_purecap_rt_{}_{}", level, exception);

        Ident::new(&ident, Span::call_site())
    }
}
