// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn event(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as ItemStruct);
    let name = &s.ident;

    let fields = match &s.fields {
        syn::Fields::Named(f) => &f.named,
        _ => panic!("#[event] requires named fields"),
    };

    let field_sizes: Vec<_> =
        fields.iter().map(|f| {
            let ty = &f.ty;
            quote! { <#ty as EventField>::SIZE }
        }).collect();

    // offsets
    let mut offset = quote! { 8usize };
    let writes = fields.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;

        let start = offset.clone();
        let end = quote! { #start + <#ty as EventField>::SIZE };

        offset = end.clone();

        quote! {
            self.#ident.write(&mut __buf[#start .. #end]);
        }
    });

    let total_size = quote! {
        8usize #( + #field_sizes )*
    };

    let expanded = quote! {
        #[derive(Discriminator)]
        #s

        impl EventBuilder for #name {
            fn emit(&self) {
                use ::core::{mem::MaybeUninit, ptr::copy_nonoverlapping};

                const __TOTAL_SIZE: usize = #total_size;

                /* ---- raw event buffer ---- */
                let mut __buf = [0u8; __TOTAL_SIZE];

                // discriminator
                __buf[..8].copy_from_slice(&Self::DISCRIMINATOR);

                // fields
                #(#writes)*

                /* ---- hex encoding ---- */
                const __HEX_LEN: usize = __TOTAL_SIZE * 2;
                let mut __hex: [u8; __HEX_LEN] = [0u8; __HEX_LEN];

                {
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    let mut i = 0;
                    while i < __TOTAL_SIZE {
                        let b = __buf[i];
                        __hex[2*i]     = HEX[(b >> 4) as usize];
                        __hex[2*i + 1] = HEX[(b & 0x0f) as usize];
                        i += 1;
                    }
                }

                const __PREFIX_LEN: usize = 7;
                const __LOG_LEN: usize = __PREFIX_LEN + __HEX_LEN;

                let mut __logger = logger::Logger::<__LOG_LEN>::default();
                __logger.append("EVENT: ");
                // SAFETY: hex output is always valid ASCII
                __logger.append(unsafe {
                    core::str::from_utf8_unchecked(&__hex)
                });
                __logger.log();
            }
        }
    };

    expanded.into()
}
