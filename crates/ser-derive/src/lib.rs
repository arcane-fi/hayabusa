// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ZcDeserialize)]
pub fn derive_zc_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl ZcDeserialize for #name {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(FromBytesUnchecked)]
pub fn derive_from_bytes_unchecked(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl FromBytesUnchecked for #name {
            unsafe fn from_bytes_unchecked<'a>(bytes: &'a [u8]) -> &'a #name {
                &*(bytes.as_ptr() as *const #name)
            }
        }
    };

    TokenStream::from(expanded)
}