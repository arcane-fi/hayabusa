use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemEnum};

/// Usage:
///   #[error]
///   pub enum ArcaneError { A, B, C }
///
/// Expands to:
///   #[repr(u32)]
///   pub enum ArcaneError { A = 200, B, C }
///   impl From<ArcaneError> for ProgramError { ... }
#[proc_macro_attribute]
pub fn error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_enum = parse_macro_input!(item as ItemEnum);

    // Ensure it's an enum with at least one variant.
    if input_enum.variants.is_empty() {
        return syn::Error::new_spanned(&input_enum, "#[error] requires at least one enum variant")
            .to_compile_error()
            .into();
    }

    // Force first variant discriminant to = 200 if it doesn't already have one.
    let first = input_enum.variants.iter_mut().next().unwrap();
    if first.discriminant.is_none() {
        let expr: syn::Expr = syn::parse_quote!(200u32);
        first.discriminant = Some((syn::token::Eq::default(), expr));
    }

    // Add #[repr(u32)] if not already present.
    let has_repr_u32 = input_enum
        .attrs
        .iter()
        .any(|a| a.path().is_ident("repr") && a.to_token_stream().to_string().contains("u32"));
    if !has_repr_u32 {
        input_enum.attrs.push(syn::parse_quote!(#[repr(u32)]));
    }

    let enum_ident = &input_enum.ident;

    // NOTE: We assume ProgramError is in scope at the call site.
    let expanded = quote! {
        #input_enum

        impl From<#enum_ident> for ProgramError {
            fn from(error: #enum_ident) -> ProgramError {
                ProgramError::Custom(error as u32)
            }
        }
    };

    expanded.into()
}
