use ::{CATCH_STRUCT_PREFIX, CATCH_FN_PREFIX};

use proc_macro::TokenStream;
use syn::{ExprKind, Ident};
use syn::{parse_expr, parse_item};

use errors::*;
use errors::ErrorKind::*;
use utils::*;

const ERR_PARAM: &'static str = "_error";
const REQ_PARAM: &'static str = "_request";

pub fn error_decorator(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let expr = parse_expr(&args.to_string()).unwrap();
    let code = match expr.node {
        ExprKind::Paren(expr) => {
            match expr.node {
                ExprKind::Lit(lit) => lit.as_isize()?,
                _ => bail!(ErrorCodeNotInteger),
            }
        }
        _ => bail!(ErrorTooMuchParam),
    };

    let func = parse_item(&item.to_string()).unwrap();
    let user_fn_name = func.ident.to_string();

    let catch_fn_ident = Ident::new(CATCH_FN_PREFIX.to_string() + &user_fn_name);
    let struct_ident = Ident::new(CATCH_STRUCT_PREFIX.to_string() + &user_fn_name);
    let user_fn_ident = Ident::new(user_fn_name);
    let err_param_ident = Ident::new(ERR_PARAM);
    let req_param_ident = Ident::new(REQ_PARAM);

    let fndecl = func.fn_decl()?;

    let input_names = fndecl.arg_name_idents();

    if input_names.len() > 2 {
        bail!(ErrorHandleTooMuchParam);
    }

    let out = quote! {
        
        #func
           
        fn #catch_fn_ident<'_b>(#err_param_ident: ::rocket::Error,
                                #req_param_ident: &'_b ::rocket::Request)
                               -> ::rocket::response::Result<'_b> {
            let user_response = #user_fn_ident(#(#input_names),*);
            let response = ::rocket::response::Responder::respond(user_response)?;
            let status = ::rocket::http::Status::raw(#code as u16);
            ::rocket::response::Response::build().status(status).merge(response).ok()
        }

        #[allow(non_upper_case_globals)]
        pub static #struct_ident: ::rocket::StaticCatchInfo =
            ::rocket::StaticCatchInfo {
                code: #code as u16,
                handler: #catch_fn_ident
            };
    };
    Ok(out.parse().unwrap())
}
