use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, LitStr};

pub fn expand_next_fn(input: &DataEnum) -> syn::Result<TokenStream> {
    let variants = input.variants.iter().map(|variant| {
        if let syn::Fields::Unit = variant.fields {
            &variant.ident
        } else {
            unimplemented!("SelectValue can only be derived on enums with unit variants")
        }
    });

    let branches = variants
        .clone()
        .zip(variants.cycle().skip(1))
        .map(|(current, next)| {
            quote! { Self::#current => Self::#next }
        });

    Ok(quote! {
        *self = match self {
            #(#branches),*
        };
    })
}

pub fn expand_name_fn(input: &DataEnum) -> syn::Result<TokenStream> {
    let variants = input
        .variants
        .iter()
        .map(|v| {
            let mut manual_name = None;
            for attr in &v.attrs {
                if attr.path().is_ident("display_as") {
                    if manual_name.is_some() {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "Only one display_as can be placed on each variant",
                        ));
                    }

                    manual_name = Some(attr.parse_args::<LitStr>()?.value());
                }
            }

            let path = &v.ident;
            let name_str = manual_name.unwrap_or_else(|| v.ident.to_string());

            Ok(quote! {
                Self::#path => #name_str
            })
        })
        .collect::<Result<Vec<_>, syn::Error>>()?;

    Ok(quote! {
        match self {
            #(#variants),*
        }
    })
}

pub fn expand_select_value(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Enum(data) = &input.data else {
        unimplemented!("SelectValue can only be derived on enums");
    };

    let enum_name = input.ident;

    let next_body = expand_next_fn(data)?;
    let name_body = expand_name_fn(data)?;

    Ok(quote! {
        impl embedded_menu::items::menu_item::SelectValue for #enum_name {
            fn next(&mut self) {
                #next_body
            }

            fn marker(&self) -> &str {
                #name_body
            }
        }
    })
}
