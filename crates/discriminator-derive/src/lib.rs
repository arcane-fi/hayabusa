// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use sha2::{Sha256, Digest};

#[proc_macro_derive(Discriminator)]
pub fn derive_discriminator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Generate the discriminator using the hasher
    let name_str = name.to_string();
    let hasher = DiscriminatorHasher::new(&name_str);
    let discriminator = hasher.hash_and_extract_discriminator();

    let expanded = quote! {
        impl Discriminator for #name {
            const DISCRIMINATOR: &'static [u8] = [#(#discriminator),*];
        }
    };

    TokenStream::from(expanded)
}

struct DiscriminatorHasher<'a> {
    pub identifier: &'a str,
}

impl<'a> DiscriminatorHasher<'a> {
    pub fn new(identifier: &'a str) -> Self {
        Self {
            identifier,
        }
    }

    pub fn hash_and_extract_discriminator(&self) -> [u8; 8] {
        let mut hasher = Sha256::new();
        hasher.update(self.identifier);
        
        let hash = hasher.finalize();
        let mut discriminator = [0u8; 8];

        discriminator.copy_from_slice(&hash[..8]);
        discriminator
    }
}