use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Type, TypePath};

#[proc_macro_derive(FromAccountViews)]
pub fn derive_from_account_views(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let lifetimes: Vec<syn::Lifetime> = input
        .generics
        .lifetimes()
        .map(|lt_def| lt_def.lifetime.clone())
        .collect();

    let info_lt = match lifetimes.as_slice() {
        [] => {
            return syn::Error::new(
                input.span(),
                "FromAccountViews can only be derived for structs with a lifetime parameter \
                 (e.g. `struct Foo<'ix> { ... }`).",
            )
            .to_compile_error()
            .into();
        }
        [lt] => lt,
        _ => {
            return syn::Error::new(
                input.span(),
                "FromAccountViews derive currently supports structs with exactly one lifetime \
                 parameter.",
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named) => &named.named,
            _ => {
                return syn::Error::new(
                    s.fields.span(),
                    "FromAccountViews can only be derived for structs with named fields",
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new(
                input.span(),
                "FromAccountViews can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };

    // For each field:
    // let field_name = OuterType::try_from_account_view(account_views.next()?)?;
    let mut let_bindings = Vec::with_capacity(fields.len());
    let mut field_idents = Vec::with_capacity(fields.len());

    for f in fields {
        let ident = match &f.ident {
            Some(i) => i,
            None => continue,
        };
        field_idents.push(ident);

        let outer = match outer_type_ident(&f.ty) {
            Ok(o) => o,
            Err(e) => return e.to_compile_error().into(),
        };

        let_bindings.push(quote! {
            let #ident = #outer::try_from_account_view(account_views.next()?)?;
        });
    }

    // Use struct literal like:
    // Ok(StructName { a, b, c })
    let expanded = quote! {
        impl #impl_generics FromAccountViews<#info_lt> for #struct_name #ty_generics #where_clause {
            #[inline(always)]
            fn try_from_account_views(account_views: &mut AccountIter<#info_lt>) -> Result<Self> {
                #(#let_bindings)*

                Ok(#struct_name {
                    #(#field_idents,)*
                })
            }
        }
    };

    expanded.into()
}

fn outer_type_ident(ty: &Type) -> Result<syn::Ident, syn::Error> {
    // We expect something like Mut<'a, Signer<'a>> or Program<'a, System>, etc.
    // Extract the first path segment ident (Mut, Program, ...).
    let tp = match ty {
        Type::Path(TypePath { path, .. }) => path,
        other => {
            return Err(syn::Error::new(
                other.span(),
                "Field type must be a path type like Mut<...> or Program<...>",
            ));
        }
    };

    let seg = tp.segments.first().ok_or_else(|| {
        syn::Error::new(tp.span(), "Expected a non-empty type path for field type")
    })?;

    Ok(seg.ident.clone())
}
