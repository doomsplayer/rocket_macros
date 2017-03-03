use {ROUTE_STRUCT_PREFIX};
use proc_macro::TokenStream;
use syn::{parse_token_trees, TokenTree, Token};
use quote;

/*
use syntax::codemap::Span;
use syntax::tokenstream::TokenTree;
use syntax::ast::{Path, Expr};
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult, MacEager};
use syntax::parse::token::Token;
use syntax::ptr::P;

#[inline]
pub fn prefix_paths(prefix: &str, paths: &mut Vec<Path>) {
    for p in paths {
        let last = p.segments.len() - 1;
        let last_seg = &mut p.segments[last];
        last_seg.identifier = last_seg.identifier.prepend(prefix);
    }
}

pub fn prefixing_vec_macro<F>(prefix: &str,
                              mut to_expr: F,
                              ecx: &mut ExtCtxt,
                              sp: Span,
                              args: &[TokenTree])
                              -> Box<MacResult + 'static>
    where F: FnMut(&ExtCtxt, Path) -> P<Expr>
{
    let mut parser = ecx.new_parser_from_tts(args);
    let paths = parser.parse_paths();
    if let Ok(mut paths) = paths {
        // Prefix each path terminator and build up the P<Expr> for each path.
        prefix_paths(prefix, &mut paths);
        let path_exprs: Vec<P<Expr>> = paths.into_iter()
            .map(|path| to_expr(ecx, path))
            .collect();

        // Now put them all in one vector and return the thing.
        let path_list = sep_by_tok(ecx, &path_exprs, Token::Comma);
        let output = quote_expr!(ecx, vec![$path_list]).unwrap();
        MacEager::expr(P(output))
    } else {
        paths.unwrap_err().emit();
        DummyResult::expr(sp)
    }
}

#[rustfmt_skip]
pub fn routes(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
    prefixing_vec_macro(ROUTE_STRUCT_PREFIX, |ecx, path| {
        quote_expr!(ecx, ::rocket::Route::from(&$path))
    }, ecx, sp, args)
}

#[rustfmt_skip]
pub fn errors(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
    prefixing_vec_macro(CATCH_STRUCT_PREFIX, |ecx, path| {
        quote_expr!(ecx, rocket::Catcher::from(&$path))
    }, ecx, sp, args)
}
*/

fn get_list_static_path(input: Vec<TokenTree>, prefix: &str) -> Vec<String> {
    let mut path : String = String::default();
    let mut current    = String::default();
    let mut static_routes = vec![];

    for x in input {
        let q = match x {
            TokenTree::Delimited(_) => panic!("Unsupported TokenTree::Delimited"),
            TokenTree::Token(x) => x,
        };

        match q {
            Token::Ident(x) => {
                current = x.to_string();
                println!("Debug: routes::token::ident {}", current);
            },
            Token::ModSep => {
                if path.is_empty() {
                    path = current;
                } else {
                    let x = format!("{}::{}", path, current);
                    path = x;
                }
                current = String::default();
                println!("Debug: routes::token::modsep {}", path);
            },
            Token::Comma => {
                let x = format!("{}{}", prefix, current);
                if path.is_empty() {
                    static_routes.push(x);
                } else {
                    let y = format!("{}::{}", path, x);
                    println!("Debug: routes::token::comma {}", y);
                    static_routes.push(y);
                }
                path = String::default();
                current = String::default();
            },
            _ => panic!("Unexpected token in routes macro"),
        }
    }   

    if !current.is_empty() {
        let x = format!("{}{}", prefix, current);
        if path.is_empty() {
            static_routes.push(x);
        } else {
            let y = format!("{}::{}", path, x);
            println!("Debug: routes::token::comma {}", y);
            static_routes.push(y);
        }
    }

    static_routes
}

pub fn routes_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let tt = parse_token_trees(&input)
        .unwrap();

    let static_routes = get_list_static_path(tt, ROUTE_STRUCT_PREFIX);
    let q = static_routes.into_iter()
        .map(quote::Ident::from)
        .map(|x| {
            quote! {
                v.push((&#x).into());
            }
        });

    let out = quote! {
        {
            let mut v = Vec::new();
            #(#q)*
            v
        }
    };
    out.parse().unwrap()
}

pub fn errors_macro(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let tt = parse_token_trees(&input).unwrap();
    let static_errors = get_list_static_path(tt, "");
    let q = static_errors.into_iter().map(quote::Ident::from)
        .map(|x| {
            quote! {
                v.push(::rocket::Catcher::new(
                    #x::code(),
                    #x::handler
                ));
            }
        });
    let out = quote! {
        {
            let mut v = Vec::new();
            #(#q)*
            v
        }
    };
    out.parse().unwrap()
}