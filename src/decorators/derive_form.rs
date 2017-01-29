use proc_macro::TokenStream;
use syn::parse_item;
use syn::{ItemKind, Field, Ident, Lifetime, LifetimeDef, VariantData};
use quote::Tokens;

use errors::*;
use errors::ErrorKind::*;
use utils::*;

static PRIVATE_LIFETIME: &'static str = "'rocket";

// TODO: Use proper logging to emit the error messages.
pub fn from_form_derive(input: TokenStream) -> Result<TokenStream> {
    let item = parse_item(&input.to_string()).unwrap();
    let struct_name_ident = item.ident.clone();
    let (struct_fields, generics) = match &item.node {
        &ItemKind::Struct(VariantData::Struct(ref fields), ref generics) => {
            (fields, generics.clone())
        }
        _ => bail!(FromFormOnlySupportNamedStruct),
    };

    if generics.ty_params.len() != 0 {
        bail!(FromFormContainTypeParam)
    }

    if generics.lifetimes.len() > 1 {
        bail!(FromFormTooMuchLifetimeParam)
    }

    let lifetime = if generics.lifetimes.len() == 0 {
        LifetimeDef {
            attrs: vec![],
            lifetime: Lifetime { ident: Ident::new(PRIVATE_LIFETIME.to_string()) },
            bounds: vec![],
        }
    } else {
        generics.lifetimes[0].clone()
    };

    let (mut pres, mut matches, mut failure_conditions, mut result_fields) =
        (vec![], vec![], vec![], vec![]);
    for field in struct_fields {
        let (pre, match_stmt, failure_condition, result_field) = from_form_derive_field(field);
        pres.push(pre);
        matches.push(match_stmt);
        failure_conditions.push(failure_condition);
        result_fields.push(result_field);
    }

    let return_err_stmt = quote! { return Err(::rocket::Error::BadParse) };
    // The error type in the derived implementation.
    let tokens = quote! {
        impl #generics ::rocket::request::FromForm<#lifetime> for #struct_name_ident #generics {
            type Error = ::rocket::Error;
            fn from_form_string(form_string: &#lifetime str) -> Result<Self, Self::Error> {
                #(#pres);*
                for (k, v) in ::rocket::request::FormItems(form_string) {
                    match k {
                        #(#matches)*
                        field if field == "_method" => {
                            /* This is a Rocket-specific field. If the user hasn't asked
                            * for it, just let it go by without error. This should stay
                            * in sync with Rocket::preprocess. */
                        }
                        _ => {
                            println!("    => {}={} has no matching field in struct.", k, v);
                            #return_err_stmt
                        }
                    }
                }
                
                if #(#failure_conditions)||* {
                    #return_err_stmt;
                }

                Ok(#struct_name_ident { #(#result_fields),* })
            }
        }
    };
    Ok(tokens.parse().unwrap())
}

fn from_form_derive_field(field: &Field) -> (Tokens, Tokens, Tokens, Tokens) {
    let return_err_stmt = quote! { return Err(::rocket::Error::BadParse) };

    let ty = field.ty.strip_lifetime();
    let ident = field.ident.clone().unwrap();
    let ident_name = ident.to_string();
    // Generate the let bindings for parameters that will be unwrapped and
    // placed into the final struct. They start out as `None` and are changed
    // to Some when a parse completes, or some default value if the parse was
    // unsuccessful and default() returns Some.
    let pre_stmt = quote! {
        let mut #ident: ::std::option::Option<#ty> = None;
    };

    let match_branch_stmt = quote! {
        #ident_name => {
            #ident = match ::rocket::request::FromFormValue::from_form_value(v) {
                Ok(v) => Some(v),
                Err(e) => {
                    println!("    => Error parsing form val '{}': {:?}", #ident_name, e);
                    #return_err_stmt
                }
            };
        }
    };

    let failure_condition = quote! {
        if #ident.is_none() &&
            <#ty as ::rocket::request::FromFormValue>::default().is_none() {
            println!("    => '{}' did not parse.", stringify!(#ident));
            true
        } else { false }
    };

    let result_field = quote! {
        #ident: #ident.unwrap_or_else(||
            <#ty as ::rocket::request::FromFormValue>::default().unwrap()
        )
    };

    (pre_stmt, match_branch_stmt, failure_condition, result_field)
}
