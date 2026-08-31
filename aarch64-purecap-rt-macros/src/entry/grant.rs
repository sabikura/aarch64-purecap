use proc_macro2::Ident;
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Error, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parenthesized, Token, Type};

/// Single grant item with the form `name: Type`, e.g. `uart: Mmio<0x1c09_0000, UartRegisters>`
pub(crate) struct ItemGrant {
    name: Ident,
    ty: Type,
}

impl Parse for ItemGrant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty = input.parse()?;
        Ok(Self { name, ty })
    }
}

/// List of grants given to the application
pub(crate) struct Grant {
    items: Punctuated<ItemGrant, Token![,]>,
}

impl Grant {
    /// Generate the `Grant` struct
    pub(crate) fn struct_(&self) -> syn::Result<proc_macro2::TokenStream> {
        let fields = self.items.iter().map(|item| {
            let name = &item.name;
            let ty = &item.ty;
            quote! { #name: ::aarch64_purecap_rt::grant::#ty }
        });

        Ok(quote! {
            struct Grant {
                #(#fields),*
            }
        })
    }

    /// Generate the `Grant` instance expression. Every item is derived from DDC,
    /// so this has to run before DDC is nulled.
    pub(crate) fn instance_(&self) -> proc_macro2::TokenStream {
        let fields = self.items.iter().map(|item| {
            let name = &item.name;
            let ty = &item.ty;
            quote! { #name: <::aarch64_purecap_rt::grant::#ty>::from_ddc() }
        });

        quote! {
            Grant {
                #(#fields),*
            }
        }
    }
}

impl Parse for Grant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self {
                items: Punctuated::new(),
            });
        }

        let ident: Ident = input.parse()?;
        if ident != "grant" {
            return Err(Error::new(ident.span(), "expected `grant`"));
        }
        let content;
        parenthesized!(content in input);
        let items: Punctuated<ItemGrant, Token![,]> = Punctuated::parse_terminated(&content)?;

        let mut names = HashSet::new();
        for item in &items {
            // - three supported types: `Heap`, `SealedRange` and `Mmio`
            // - `heap` ident should match with the `Heap` type
            // - `seal` ident should match with the `Seal` type
            // - any other idents should match with the `Mmio` type
            // - duplicated items should not exist
            // - types should be plain, without any path segments leading to them
            if !names.insert(item.name.to_string()) {
                return Err(Error::new(
                    item.name.span(),
                    format!("duplicate grant item `{}`", item.name),
                ));
            }

            let expected = if item.name == "heap" {
                "Heap"
            } else if item.name == "seal" {
                "SealRange"
            } else {
                "Mmio"
            };

            let ty_ident = match &item.ty {
                Type::Path(path)
                    if path.qself.is_none()
                        && path.path.leading_colon.is_none()
                        && path.path.segments.len() == 1 =>
                {
                    path.path.segments.first().map(|segment| &segment.ident)
                }
                _ => None,
            };
            if !ty_ident.is_some_and(|ident| ident == expected) {
                return Err(Error::new(
                    item.ty.span(),
                    format!("expected `{expected}` for grant item `{}`", item.name),
                ));
            }
        }

        Ok(Self { items })
    }
}
