use proc_macro::TokenStream;
use syn::{ExprKind, FnArg, Ident, Ty};
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
                ExprKind::Lit(lit) => lit.as_isize()? as u16,
                _ => bail!(ErrorCodeNotInteger),
            }
        }
        _ => bail!(ErrorTooMuchParam),
    };

    let func = parse_item(&item.to_string()).unwrap();

    let fn_name = func.ident.to_string();
    let fn_vis = func.vis.clone();

    let fn_ident = Ident::new(fn_name);

    let err_param_ident = Ident::new(ERR_PARAM);
    let req_param_ident = Ident::new(REQ_PARAM);

    let fn_decl = func.fn_decl()?;
    if fn_decl.inputs.len() > 2 {
        bail!(ErrorHandleTooMuchParam);
    }
    let mut input_name_idents = vec![];
    for fn_arg in &fn_decl.inputs {
        let ident = match fn_arg {
            &FnArg::Captured(_, ref ty) => {
                match ty {
                    &Ty::Rptr(..) => req_param_ident.clone(),
                    &Ty::Path(..) => err_param_ident.clone(),
                    _ => bail!(ErrorHandleUnexpectedParam),
                }
            }
            &FnArg::Ignored(ref ty) => {
                match ty {
                    &Ty::Rptr(..) => req_param_ident.clone(),
                    &Ty::Path(..) => err_param_ident.clone(),
                    _ => bail!(ErrorHandleUnexpectedParam),
                }
            }
            _ => bail!(ErrorHandleContainSelf),
        };
        input_name_idents.push(ident);
    }

    let out = quote! {

        #[allow(non_camel_case_types)]
        #fn_vis struct #fn_ident;
     
        impl #fn_ident {
            #fn_vis fn code() -> u16 { #code }
            #fn_vis fn handler<'_a>(#err_param_ident: ::rocket::Error,
                                    #req_param_ident: &'_a ::rocket::Request) -> ::rocket::response::Result<'_a> {

                #func
                        
                let user_response = #fn_ident(#(#input_name_idents),*);
                let response = ::rocket::response::Responder::respond(user_response)?;
                let status = ::rocket::http::Status::raw(#code);
                ::rocket::response::Response::build().status(status).merge(response).ok()
            }
        }
    };
    Ok(out.parse().unwrap())
}
