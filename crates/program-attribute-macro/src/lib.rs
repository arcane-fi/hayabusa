// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse_macro_input, FnArg, Item, ItemFn, ItemMod, Pat, Result as SynResult, Type, TypePath, PathArguments,
};
use heck::ToUpperCamelCase;

#[proc_macro_attribute]
pub fn program(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_program(parse_macro_input!(item as ItemMod))
        .unwrap()
        .into()
}

fn expand_program(module: ItemMod) -> SynResult<proc_macro2::TokenStream> {
    let mod_ident = &module.ident;
    let (_, items) = module.content.expect("inline module required");

    let mut instruction_structs = Vec::new();
    let mut dispatch_arms = Vec::new();
    let mut handlers = Vec::new();

    for item in items {
        if let Item::Fn(func) = item {
            extract_instruction(&func, &mut instruction_structs, &mut dispatch_arms);
            handlers.push(func);
        }
    }

    Ok(quote! {
        mod instructions {
            use super::*;
            #(#instruction_structs)*
        }

        #[cfg(not(feature = "no-entrypoint"))]
        mod #mod_ident {
            use super::*;
            use super::instructions::*;

            default_allocator!();
            nostd_panic_handler!();

            program_entrypoint!(dispatcher);

            fn dispatcher(
                program_id: &Address,
                views: &[AccountView],
                ix_data: &[u8],
            ) -> Result<()> {
                dispatch!(
                    program_id,
                    ix_data,
                    views,
                    #(#dispatch_arms,)*
                );
            }

            #(#handlers)*
        }
    })
}

fn extract_instruction(
    func: &ItemFn,
    instruction_structs: &mut Vec<proc_macro2::TokenStream>,
    dispatch_arms: &mut Vec<proc_macro2::TokenStream>,
) {
    let fn_name = &func.sig.ident;
    let mut fn_name_str = fn_name.to_string().to_upper_camel_case();
    fn_name_str.push_str("Ix");

    let struct_ident = Ident::new(
        &fn_name_str,
        Span::call_site(),
    );

    let mut fields = Vec::new();
    let mut args = Vec::new();
    let mut needs_ix_lifetime = false;

    // skip ctx
    for input in func.sig.inputs.iter().skip(1) {
        let FnArg::Typed(pat) = input else { continue };
        let Pat::Ident(pat_ident) = &*pat.pat else { continue };

        let ident = &pat_ident.ident;
        let mut ty = (*pat.ty).clone();

        // detect &[u8]
        if is_u8_slice_ref(&ty) {
            needs_ix_lifetime = true;
            ty = syn::parse_quote! { &'ix [u8] };
        }

        fields.push(quote! { pub #ident: #ty });
        args.push(quote! { #ident });
    }

    let generics = if needs_ix_lifetime {
        quote! { <'ix> }
    } else {
        quote! {}
    };

    instruction_structs.push(quote! {
        #[derive(Discriminator, DecodeIx)]
        #[repr(C)]
        pub struct #struct_ident #generics {
            #(#fields,)*
        }
    });

    dispatch_arms.push(quote! {
        #struct_ident => #fn_name(#(#args),*)
    });
}

fn is_u8_slice_ref(ty: &Type) -> bool {
    let Type::Reference(r) = ty else { return false };
    let Type::Slice(slice) = &*r.elem else { return false };
    let Type::Path(TypePath { path, .. }) = &*slice.elem else { return false };

    path.segments.len() == 1
        && path.segments[0].ident == "u8"
        && matches!(path.segments[0].arguments, PathArguments::None)
}
