use proc_macro2::Ident;
use quote::quote;
use std::collections::HashSet;
use syn::parse::{Error, Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parenthesized, GenericArgument, PathArguments, Token, Type};

/// Single grant item with the form `name: Type`, e.g. `uart: Mmio<0x1c09_0000, UartRegisters>`
pub(crate) struct ItemGrant {
    name: Ident,
    ty: Type,
}

impl ItemGrant {
    /// Target type `T` of an exact `Mmio<ADDRESS, T>` type, `None` for any other shape
    fn mmio_target(&self) -> Option<&Type> {
        let Type::Path(path) = &self.ty else {
            return None;
        };
        let segment = path.path.segments.first()?;
        if segment.ident != "Mmio" {
            return None;
        }
        let PathArguments::AngleBracketed(args) = &segment.arguments else {
            return None;
        };
        if args.args.len() != 2 {
            return None;
        }
        match args.args.last()? {
            GenericArgument::Type(ty) => Some(ty),
            _ => None,
        }
    }
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
    pub(crate) fn struct_(&self) -> proc_macro2::TokenStream {
        // Fields of the `__Grant` private struct used to generate `Grant` which will be given to
        // the user
        let __grant_fields = self.items.iter().map(|item| {
            let name = &item.name;
            let ty = &item.ty;
            quote! { #name: ::aarch64_purecap_rt::grant::#ty }
        });

        // Fields of the `Grant` struct which will be given to the user
        let grant_fields = self.items.iter().filter_map(|item| {
            let name = &item.name;
            let target = item.mmio_target()?;
            Some(quote! { pub(crate) #name: *mut #target })
        });

        // `__Grant` to `Grant` conversion. The `From` impls in the runtime crate read the
        // const generics, so nothing has to be extracted here.
        let into_grant_fields = self.items.iter().filter_map(|item| {
            let name = &item.name;
            item.mmio_target()?;
            Some(quote! { #name: self.#name.into() })
        });

        // `__Grant` initializers, one per granted item
        let __grant_init_fields = self.items.iter().map(|item| {
            let name = &item.name;
            let ty = if item.name == "heap" {
                quote! { Heap }
            } else if item.name == "seal" {
                quote! { SealRange }
            } else {
                quote! { Mmio }
            };
            // SAFETY: the markers match the grant list given to the `entry` macro
            quote! { #name: unsafe { ::aarch64_purecap_rt::grant::#ty::new() } }
        });

        quote! {
            mod __grant {
                use super::*;

                #[allow(dead_code)]
                struct __Grant {
                    #(#__grant_fields),*
                }

                pub(crate) struct Grant {
                    #(#grant_fields,)*
                    __private: (),
                }

                impl __Grant {
                    fn into_grant(self) -> Grant {
                        Grant {
                            #(#into_grant_fields,)*
                            __private: (),
                        }
                    }
                }

                pub(crate) fn instance() -> Grant {
                    __Grant {
                        #(#__grant_init_fields),*
                    }
                    .into_grant()
                }
            }

            pub(crate) use __grant::Grant;
        }
    }

    /// Generate the `Grant` instance expression. Every field is derived from DDC, so this
    /// has to run before DDC is nulled.
    pub(crate) fn instance(&self) -> proc_macro2::TokenStream {
        quote! { __grant::instance() }
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

            if expected == "Mmio" && item.mmio_target().is_none() {
                return Err(Error::new(
                    item.ty.span(),
                    format!("expected `Mmio<ADDRESS, T>` for grant item `{}`", item.name),
                ));
            }
        }

        Ok(Self { items })
    }
}
