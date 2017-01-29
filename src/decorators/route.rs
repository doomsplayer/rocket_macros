use std::collections::HashSet;
use proc_macro::TokenStream;


use rocket::http::Method;
use syn::Ident;
use syn::{parse_expr, parse_item};
use quote::Tokens;


use ::{ROUTE_STRUCT_PREFIX, ROUTE_FN_PREFIX, PARAM_PREFIX};
use utils::*;
use errors::*;
use errors::ErrorKind::*;

macro_rules! method_decorator {
    ($name:ident) => (
        pub fn $name(args: TokenStream, item: TokenStream) -> Result<TokenStream>  {
            let router_def = RouterDef::new(None, 
                                            parse_expr(&args.to_string()).unwrap(), 
                                            parse_item(&item.to_string()).unwrap())?;
            debug!("Route params: {:?}", router_def);
            Ok(generic_route_decorator(router_def)?)
        }
    );
    ($name:ident, $method:ident) => (
        pub fn $name(args: TokenStream, item: TokenStream) -> Result<TokenStream>  {
            let router_def = RouterDef::new(Some(Method::$method), 
                                            parse_expr(&args.to_string()).unwrap(), 
                                            parse_item(&item.to_string()).unwrap())?;
            debug!("Route params: {:?}", router_def);
            Ok(generic_route_decorator(router_def)?)
        }
    )
}

method_decorator!(route_decorator);
method_decorator!(get_decorator, Get);
method_decorator!(put_decorator, Put);
method_decorator!(post_decorator, Post);
method_decorator!(delete_decorator, Delete);
method_decorator!(head_decorator, Head);
method_decorator!(patch_decorator, Patch);
// TODO: Allow this once Diesel incompatibility is fixed.
// method_decorator!(options_decorator, Options);


// FIXME: Compilation fails when parameters have the same name as the function!
fn generic_route_decorator(router_def: RouterDef) -> Result<TokenStream> {
    let func = router_def.item.clone();
    let fn_arguments = router_def.fn_decl()?.arg_name_idents();
    let param_statements = generate_param_statements(&router_def)?;
    let data_statement = generate_data_statement(&router_def)?;
    let query_statement = generate_query_statement(&router_def)?;

    // Generate and emit the wrapping function with the Rocket handler signature.
    let user_fn_name = router_def.fn_name();
    let struct_ident = Ident::new(ROUTE_STRUCT_PREFIX.to_string() + &user_fn_name);
    let route_fn_ident = Ident::new(ROUTE_FN_PREFIX.to_string() + &user_fn_name);
    let user_fn_ident = Ident::new(user_fn_name);

    let path = router_def.path;
    let method = MethodWrapper(router_def.method);
    let content_type = router_def.format
        .map(|f| {
            let c = ContentTypeWrapper(f);
            quote! { Some(#c) }
        })
        .unwrap_or(quote! { None });
    let rank = router_def.rank.map(|r| quote! { Some(#r) }).unwrap_or(quote! { None });

    let tokens = quote! {
        #func
        
        fn #route_fn_ident<'_b>(_req: &'_b ::rocket::Request, _data: ::rocket::Data)    
                -> ::rocket::handler::Outcome<'_b> {
             #param_statements
             #query_statement
             #data_statement
             let responder = #user_fn_ident(#(#fn_arguments),*);
             ::rocket::handler::Outcome::of(responder)
        }

        // Generate and emit the static route info that uses the just generated
        // function as its handler. A proper Rocket route will be created from this.
        #[allow(non_upper_case_globals)]
        pub static #struct_ident: ::rocket::StaticRouteInfo =
            ::rocket::StaticRouteInfo {
                method: #method,
                path: #path,
                handler: #route_fn_ident,
                format: #content_type,
                rank: #rank,
            };
    };
    tokens.parse().map_err(|e| LexError(e).into())
}

fn generate_query_statement(router_def: &RouterDef) -> Result<Option<Tokens>> {
    if let Some(ref query_param_name) = router_def.query_param {
        let expr = quote!(match _req.uri().query() {
            Some(query) => query,
            None => return ::rocket::Outcome::Forward(_data),
        });

        if let Some(ty) = router_def.fn_decl()?.find_input_ty(query_param_name) {

            let name = Ident::new(PARAM_PREFIX.to_string() + query_param_name);

            Ok(Some(quote! {
                let #name: #ty =
                    match ::rocket::request::FromForm::from_form_string(#expr) {
                        Ok(v) => v,
                        Err(_) => return ::rocket::Outcome::Forward(_data)
                    };
            }))
        } else {
            bail!(RouteParamNotFoundInFnInput(query_param_name.clone()))
        }
    } else {
        Ok(None)
    }
}

fn generate_data_statement(router_def: &RouterDef) -> Result<Option<Tokens>> {
    if let Some(ref data_param_name) = router_def.data_param {
        if let Some(ty) = router_def.fn_decl()?.find_input_ty(data_param_name) {

            let name = Ident::new(PARAM_PREFIX.to_string() + data_param_name);

            Ok(Some(quote! {
                let #name: #ty =
                    match ::rocket::data::FromData::from_data(_req, _data) {
                        ::rocket::Outcome::Success(d) => d,
                        ::rocket::Outcome::Forward(d) =>
                            return ::rocket::Outcome::Forward(d),
                        ::rocket::Outcome::Failure((code, _)) => {
                            return ::rocket::Outcome::Failure(code);
                        }
                    };
            }))
        } else {
            bail!(RouteParamNotFoundInFnInput(data_param_name.clone()))
        }
    } else {
        Ok(None)
    }
}

// TODO: Add some kind of logging facility in Rocket to get be able to log
// an error/debug message if parsing a parameter fails.
fn generate_param_statements(router_def: &RouterDef) -> Result<Tokens> {
    let mut fn_param_statements = vec![];

    // Generate a statement for every declared paramter in the path.
    let mut declared_set = HashSet::new();

    for (i, param) in ParamIter::new(&router_def.path).enumerate() {
        let param_name = param.name().to_string();
        declared_set.insert(param_name.clone());

        let ty = router_def.fn_decl()?
            .find_input_ty(&param_name)
            .ok_or(RouteParamNotFoundInFnInput(param_name.clone()))?;

        let expr = match param {
            Param::Single(_) => {
                quote! {
                        match _req.get_param_str(#i) {
                            Some(s) => <#ty as ::rocket::request::FromParam>::from_param(s),
                            None => return ::rocket::Outcome::Forward(_data)
                        }
                    }
            }
            Param::Many(_) => {
                quote! {
                    match _req.get_raw_segments(#i) {
                        Some(s) => <#ty as ::rocket::request::FromSegments>::from_segments(s),
                        None => return ::rocket::Outcome::Forward(_data)
                    }
                }
            }
        };

        let param_ident = Ident::new(PARAM_PREFIX.to_string() + &param_name);
        let original_param_ident = Ident::new(param_name);
        fn_param_statements.push(quote! {
            let #param_ident: #ty = match #expr {
                Ok(v) => v,
                Err(e) => {
                    println!("    => Failed to parse '{}': {:?}",
                                stringify!(#original_param_ident), e);
                    return ::rocket::Outcome::Forward(_data)
                }
            };
        })
    }

    // Generate the code for `from_request` parameters.
    for arg in router_def.fn_decl()?.inputs.iter() {
        let arg_name = arg.name()?;
        let arg_ty = arg.ty()?;
        // A from_request parameter is one that isn't declared, data, or query.
        if declared_set.contains(&arg_name[..]) ||
           router_def.data_param
            .as_ref()
            .map_or(false, |data_param_name| arg_name == *data_param_name) ||
           router_def.query_param
            .as_ref()
            .map_or(false, |query_param_name| arg_name == *query_param_name) {
            continue;
        }

        let arg_ident = Ident::new(PARAM_PREFIX.to_string() + &arg_name);

        fn_param_statements.push(quote! {
            let #arg_ident: #arg_ty = match
                    ::rocket::request::FromRequest::from_request(_req) {
                        ::rocket::outcome::Outcome::Success(v) => v,
                        ::rocket::outcome::Outcome::Forward(_) =>
                            return ::rocket::Outcome::forward(_data),
                        ::rocket::outcome::Outcome::Failure((code, _)) => {
                            return ::rocket::Outcome::Failure(code)
                },
            };
        })
    }

    Ok(quote! {
        #(#fn_param_statements);*
    })
}
