use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Ident, Type,
    TypePath,
};

#[proc_macro_derive(FromAccountViews, attributes(args))]
pub fn derive_from_account_views(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // ---- extract exactly one lifetime ('ix)
    let info_lt = match input.generics.lifetimes().collect::<Vec<_>>().as_slice() {
        [lt] => &lt.lifetime,
        [] => {
            return syn::Error::new(
                input.span(),
                "FromAccountViews requires exactly one lifetime parameter",
            )
            .to_compile_error()
            .into();
        }
        _ => {
            return syn::Error::new(
                input.span(),
                "FromAccountViews supports exactly one lifetime parameter",
            )
            .to_compile_error()
            .into();
        }
    };

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(n) => &n.named,
            _ => {
                return syn::Error::new(
                    s.fields.span(),
                    "FromAccountViews supports named fields only",
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

    let mut bindings = Vec::new();
    let mut field_idents = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        field_idents.push(ident);

        let outer = match outer_type_ident(&field.ty) {
            Ok(o) => o,
            Err(e) => return e.to_compile_error().into(),
        };

        let meta_expr = match parse_args(&field.attrs, &outer, info_lt) {
            Ok(m) => m,
            Err(e) => return e.to_compile_error().into(),
        };

        bindings.push(quote! {
            let #ident =
                #outer::try_from_account_view(
                    account_views.next()?,
                    #meta_expr,
                )?;
        });
    }

    let expanded = quote! {
        impl #impl_generics FromAccountViews<#info_lt>
            for #struct_name #ty_generics #where_clause
        {
            fn try_from_account_views(
                account_views: &mut AccountIter<#info_lt>
            ) -> Result<Self> {
                #(#bindings)*

                Ok(Self {
                    #(#field_idents,)*
                })
            }
        }
    };

    expanded.into()
}


fn outer_type_ident(ty: &Type) -> Result<Ident, syn::Error> {
    match ty {
        Type::Path(TypePath { path, .. }) => Ok(path.segments.first().unwrap().ident.clone()),
        _ => Err(syn::Error::new(ty.span(), "expected path type")),
    }
}

fn parse_args(
    attrs: &[syn::Attribute],
    outer: &Ident,
    info_lt: &syn::Lifetime,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    for attr in attrs {
        if attr.path().is_ident("args") {
            let args = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::MetaNameValue, syn::Token![,]>::parse_terminated,
            )?;

            // Named args only; we pass VALUES in declaration order
            let values = args.iter().map(|kv| &kv.value);

            return Ok(quote! {
                <#outer as FromAccountView<#info_lt>>::Meta::new(
                    #(#values),*
                )
            });
        }
    }

    Ok(quote! { () })
}

