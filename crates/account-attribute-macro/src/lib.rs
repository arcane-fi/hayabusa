// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemStruct, Result};

fn strip_account_attr(attrs: &[Attribute]) -> Vec<Attribute> {
    attrs
        .iter()
        .cloned()
        .filter(|attr| !attr.path().is_ident("account"))
        .collect()
}

/// Expands to:
/// ```
/// #[derive(::bytemuck::Pod, ::bytemuck::Zeroable)]
/// #[derive(Discriminator, Len, ZcDeserialize, Copy, Clone)]
/// #[repr(C)]
/// ```
#[proc_macro_attribute]
pub fn account(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !proc_macro2::TokenStream::from(attr.clone()).is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(attr),
            "#[account] does not take arguments",
        )
        .to_compile_error()
        .into();
    }

    let input = parse_macro_input!(item as ItemStruct);

    match expand_account(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_account(input: ItemStruct) -> Result<proc_macro2::TokenStream> {
    let ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        semi_token,
        ..
    } = input;

    if semi_token.is_some() {
        return Err(syn::Error::new_spanned(
            ident,
            "#[account] does not support tuple/unit structs",
        ));
    }

    let preserved_struct_attrs = strip_account_attr(&attrs);
    let (impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        #(#preserved_struct_attrs)*
        #[derive(::bytemuck::Pod, ::bytemuck::Zeroable, Discriminator, Len, ZcDeserialize, Copy, Clone)]
        #[repr(C)]
        #vis struct #ident #impl_generics #fields #where_clause
    })
}
