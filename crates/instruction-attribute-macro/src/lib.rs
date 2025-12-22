// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, FnArg, Ident, ItemFn, Pat, PatIdent, PatType, ReturnType, Type, LitStr,
};

#[proc_macro_attribute]
pub fn instruction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &input_fn.sig.ident;
    let ix_name = to_pascal_case(fn_name);
    let struct_name = with_instruction_ident(ix_name.clone());
    let struct_ident = format_ident!("{struct_name}");

    let mut fields = Vec::new();

    for (i, arg) in input_fn.sig.inputs.iter().enumerate() {
        if i == 0 {
            // Expect ctx: Context<...>, skip the first arg
            continue;
        }

        let (ident, ty) = match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let ident = match pat.as_ref() {
                    Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
                    _ => {
                        return syn::Error::new_spanned(
                            pat,
                            "expected a simple identifier pattern like `some_data: u64`",
                        )
                        .to_compile_error()
                        .into();
                    }
                };
                (ident, ty.as_ref().clone())
            }
            FnArg::Receiver(_) => {
                return syn::Error::new_spanned(arg, "methods are not supported")
                    .to_compile_error()
                    .into();
            }
        };

        // Reject references in instruction data for POD
        if contains_reference(&ty) {
            return syn::Error::new_spanned(
                ty,
                "instruction arguments must be owned types (no references) to derive Pod/Zeroable safely",
            )
            .to_compile_error()
            .into();
        }

        fields.push((ident, ty));
    }

    if !returns_result_unit(&input_fn.sig.output) {
        return syn::Error::new_spanned(
            &input_fn.sig.output,
            "expected return type `Result<()>` (or equivalent)",
        )
        .to_compile_error()
        .into();
    }

    let field_defs = fields.iter().map(|(ident, ty)| quote! { pub #ident: #ty, });

    let mut instrumented_fn = input_fn.clone();
    let msg = LitStr::new(&format!("Instruction: {}", ix_name), Span::call_site());
    instrumented_fn
        .block
        .stmts
        .insert(0, syn::parse_quote! { log!(#msg); });

    let expanded = quote! {
        #instrumented_fn

        #[derive(Pod, Zeroable, Discriminator, Clone, Copy)]
        #[repr(C)]
        pub struct #struct_ident {
            #(#field_defs)*
        }
    };

    expanded.into()
}

fn to_pascal_case(ident: &Ident) -> String {
    let s = ident.to_string();
    let parts = s.split('_').filter(|p| !p.is_empty());
    let mut out = String::new();
    for p in parts {
        let mut chars = p.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.extend(chars.flat_map(|c| c.to_lowercase()));
        }
    }

    out
}

fn with_instruction_ident(mut str: String) -> String {
    if !str.ends_with("Instruction") {
        str.push_str("Instruction");
    }

    str
}

fn returns_result_unit(ret: &ReturnType) -> bool {
    match ret {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => {
            let s = quote!(#ty).to_string().replace(' ', "");
            s == "Result<()>" || s.ends_with("::Result<()>")
        }
    }
}

fn contains_reference(ty: &Type) -> bool {
    match ty {
        Type::Reference(_) => true,
        Type::Array(a) => contains_reference(&a.elem),
        Type::Group(g) => contains_reference(&g.elem),
        Type::Paren(p) => contains_reference(&p.elem),
        Type::Path(_) => false, // could recurse generics, but keep simple
        _ => false,
    }
}
