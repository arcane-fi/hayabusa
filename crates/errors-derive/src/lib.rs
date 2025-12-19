// Copyright (c) 2025, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use jutsu_syn::Errors;

#[proc_macro_derive(JutsuError, attributes(msg))]
pub fn jutsu_error(input: TokenStream) -> TokenStream {
    let errors_token = parse_macro_input!(input as Errors);

    let name = &errors_token.name;

    let (to_str_arms, try_from_arms) = errors_token
        .variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.name;
            let msg = &variant.msg;
            let discriminant = &variant.discriminant;
            (
                quote!(#name::#variant_name => #msg,),
                quote!(#discriminant => Ok(#name::#variant_name),),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    quote! {
        impl TryFrom<u32> for #name {
            type Error = ProgramError;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match value {
                    #(#try_from_arms)*
                    _ => Err(ProgramError::InvalidArgument),
                }
            }
        }

        impl ToStr for #name {
            fn to_str<E>(&self) -> &'static str
            where
                E: 'static + ToStr + TryFrom<u32>,
            {
                match self {
                    #(#to_str_arms)*
                }
            }
        }

        impl From<#name> for Error {
            fn from(value: #name) -> Self {
                Error::new(ProgramError::Custom(value as u32))
            }
        }
    }
    .into_token_stream()
    .into()
}