// Copyright (c) 2026, Arcane Labs <dev@arcane.fi>
// SPDX-License-Identifier: Apache-2.0

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Fields, GenericParam,
    Generics, Ident, Lifetime, LifetimeParam, Type, TypeArray, TypePath, TypeReference, TypeSlice,
};

#[proc_macro_derive(DecodeIx)]
pub fn derive_decode_ix(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_decode_ix(input) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_decode_ix(input: DeriveInput) -> Result<TokenStream2, Error> {
    let ident = input.ident;

    // Require #[repr(C)] (layout/padding stability)
    if !has_repr_c(&input.attrs) {
        return Err(Error::new(
            Span::call_site(),
            "DecodeIx derive requires #[repr(C)] on the struct",
        ));
    }

    // Only structs with named fields
    let fields = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(n) => n.named,
            _ => {
                return Err(Error::new(
                    Span::call_site(),
                    "DecodeIx derive only supports structs with named fields",
                ))
            }
        },
        _ => return Err(Error::new(Span::call_site(), "DecodeIx derive only supports structs")),
    };

    // Decide the lifetime used for DecodeIx<'ix>:
    // - If the type has at least one lifetime parameter, use the first one (whatever its name).
    // - Otherwise, introduce a fresh 'ix only in the impl generics (type stays non-generic).
    let (ix_lt, impl_generics_ts, ty_generics_ts, where_clause_ts, type_has_lifetime) =
        lifetime_strategy(&input.generics)?;

    // Scan fields, classify the single borrowed byte slice (if present)
    let mut slice_field: Option<(Ident, Type)> = None;

    // We'll generate two decode passes:
    // 1) decode all fixed-size fields up to the slice, skipping slice
    // 2) compute slice_len = bytes.len() - fixed_total, decode slice at its position,
    //    and continue decoding remaining fixed-size fields.
    //
    // To do that, we record per-field decode "ops" in order, with a marker for the slice.
    enum Op {
        Fixed { ty: Type, size: TokenStream2, decode: TokenStream2, init: TokenStream2 },
        Slice { name: Ident, ty: Type },
    }

    let mut ops: Vec<Op> = Vec::new();
    let mut fixed_sizes: Vec<TokenStream2> = Vec::new();

    for field in fields.iter() {
        let name = field.ident.clone().unwrap();
        let ty = field.ty.clone();

        // Allow PhantomData to be initialized without consuming bytes.
        if is_phantom_data(&ty) {
            ops.push(Op::Fixed {
                ty: ty.clone(),
                size: quote!(0usize),
                decode: quote! { let #name: #ty = core::marker::PhantomData; },
                init: quote!(#name: #name),
            });
            continue;
        }

        // Borrowed slices:
        if is_any_slice_ref(&ty) {
            // Only allow &'ix [u8] (alignment-safe)
            if !is_u8_slice_ref(&ty) {
                return Err(Error::new(
                    ty.span(),
                    "DecodeIx derive only supports borrowed byte slices: &'ix [u8]. Borrowed &[T] is not safe on Solana because instruction data is only 1-byte aligned.",
                ));
            }

            // If the type has no lifetime params, it cannot contain &'ix [u8].
            if !type_has_lifetime {
                return Err(Error::new(
                    ty.span(),
                    "Struct contains a borrowed field (&'ix [u8]) but declares no lifetime parameter. Add a lifetime, e.g. `pub struct FooIx<'ix> { data: &'ix [u8], ... }`.",
                ));
            }

            // Only one slice field allowed (unambiguous remainder)
            if slice_field.is_some() {
                return Err(Error::new(
                    ty.span(),
                    "DecodeIx derive supports at most one borrowed byte slice (&'ix [u8]) because its length is derived as the remainder of the input.",
                ));
            }

            slice_field = Some((name.clone(), ty.clone()));
            ops.push(Op::Slice { name, ty });
            continue;
        }

        // Fixed-size fields
        let size_expr = size_of_type_expr(&ty)?;
        let decode_stmt = decode_fixed_field_stmt(&name, &ty);

        fixed_sizes.push(size_expr.clone());
        ops.push(Op::Fixed {
            ty: ty.clone(),
            size: size_expr,
            decode: decode_stmt,
            init: quote!(#name: #name),
        });
    }

    let fixed_total = if fixed_sizes.is_empty() {
        quote!(0usize)
    } else {
        fixed_sizes
            .into_iter()
            .reduce(|a, b| quote!((#a) + (#b)))
            .unwrap()
    };

    // Bounds check:
    // - If slice exists: bytes.len() >= fixed_total
    // - Else: bytes.len() == fixed_total
    let len_check = if slice_field.is_some() {
        quote! {
            if bytes.len() < #fixed_total {
                return Err(ProgramError::InvalidInstructionData);
            }
        }
    } else {
        quote! {
            if bytes.len() != #fixed_total {
                return Err(ProgramError::InvalidInstructionData);
            }
        }
    };

    // Generate decode body in order, inserting slice decode where it appears.
    let off = Ident::new("__off", Span::call_site());
    let mut decode_stmts: Vec<TokenStream2> = Vec::new();
    let mut inits: Vec<TokenStream2> = Vec::new();

    decode_stmts.push(quote! { let mut #off: usize = 0usize; });

    for op in ops {
        match op {
            Op::Fixed { ty: _ty, size, decode, init, .. } => {
                // For PhantomData we used size=0 and a direct let.
                decode_stmts.push(decode);
                // If size is 0usize, still ok to add; but avoid useless add for tidiness
                decode_stmts.push(quote! { #off += #size; });
                inits.push(init);
            }
            Op::Slice { name, ty } => {
                // Remainder slice: slice_len = bytes.len() - fixed_total
                // We place it exactly at current offset.
                decode_stmts.push(quote! {
                    let __slice_len: usize = bytes.len() - #fixed_total;
                    let #name: #ty = &bytes[#off .. #off + __slice_len];
                    #off += __slice_len;
                });
                inits.push(quote!(#name: #name));
            }
        }
    }

    // Remove the extra "off += size" for PhantomData (since size=0 it's harmless, but we can keep it).
    // Note: This macro assumes `Result` and `ProgramError` are in scope in the target crate.
    let expanded = quote! {
        impl #impl_generics_ts DecodeIx<#ix_lt> for #ident #ty_generics_ts #where_clause_ts {
            #[inline(always)]
            fn decode(bytes: &#ix_lt [u8]) -> Result<Self> {
                #len_check
                #(#decode_stmts)*

                Ok(Self {
                    #(#inits),*
                })
            }
        }
    };

    Ok(expanded)
}

fn has_repr_c(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        if !a.path().is_ident("repr") {
            return false;
        }
        // Cheap but robust: repr(C) will stringify containing "C"
        a.to_token_stream().to_string().contains("C")
    })
}

fn lifetime_strategy(
    generics: &Generics,
) -> Result<(Lifetime, TokenStream2, TokenStream2, TokenStream2, bool), Error> {
    if let Some(first_lt) = generics.lifetimes().next() {
        // Use the type's first lifetime as the DecodeIx lifetime too.
        let ix_lt = first_lt.lifetime.clone();

        let (impl_g, ty_g, where_c) = generics.split_for_impl();
        Ok((
            ix_lt,
            quote!(#impl_g),
            quote!(#ty_g),
            quote!(#where_c),
            true,
        ))
    } else {
        // Type has no lifetime params; add a fresh 'ix only to the impl generics.
        let ix_lt = Lifetime::new("'ix", Span::call_site());

        let mut impl_generics = generics.clone();
        impl_generics
            .params
            .insert(0, GenericParam::Lifetime(LifetimeParam::new(ix_lt.clone())));

        let (impl_g, _ty_g_for_impl, where_c) = impl_generics.split_for_impl();
        let (_impl_g_type, ty_g_type, _where_c_type) = generics.split_for_impl();

        Ok((
            ix_lt,
            quote!(#impl_g),
            quote!(#ty_g_type),
            quote!(#where_c),
            false,
        ))
    }
}

fn is_phantom_data(ty: &Type) -> bool {
    // Matches PhantomData<...> or core::marker::PhantomData<...>
    matches!(
        ty,
        Type::Path(TypePath { qself: None, path, .. })
            if path.segments.last().is_some_and(|s| s.ident == "PhantomData")
    )
}

fn is_any_slice_ref(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Reference(TypeReference { elem, .. })
            if matches!(elem.as_ref(), Type::Slice(_))
    )
}

fn is_u8_slice_ref(ty: &Type) -> bool {
    // &'a [u8]
    let Type::Reference(TypeReference { elem, .. }) = ty else {
        return false;
    };

    let Type::Slice(TypeSlice { elem, .. }) = elem.as_ref() else {
        return false;
    };

    matches!(
        elem.as_ref(),
        Type::Path(TypePath { qself: None, path, .. }) if path.is_ident("u8")
    )
}

fn size_of_type_expr(ty: &Type) -> Result<TokenStream2, Error> {
    // Only allow [u8; N] arrays (fixed, safe)
    if let Type::Array(TypeArray { elem, .. }) = ty {
        if !is_u8_type(elem.as_ref()) {
            return Err(Error::new(
                ty.span(),
                "DecodeIx derive only supports fixed arrays of bytes: [u8; N]",
            ));
        }
    }

    Ok(quote!(core::mem::size_of::<#ty>()))
}

fn is_u8_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::Path(TypePath { qself: None, path, .. }) if path.is_ident("u8")
    )
}

fn decode_fixed_field_stmt(name: &Ident, ty: &Type) -> TokenStream2 {
    quote! {
        let #name: #ty = unsafe {
            core::ptr::read_unaligned(
                bytes.as_ptr().add(__off) as *const #ty
            )
        };
    }
}