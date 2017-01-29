use std::collections::HashMap;

use rocket::http::{Method, ContentType};
use syn::{Expr, ExprKind, FnDecl, Item};
use result::OptionResultExt;

use utils::*;
use errors::*;
use errors::ErrorKind::*;

#[derive(Debug)]
pub struct RouterDef {
    pub item: Item,
    pub method: Method,
    pub path: String,
    pub query_param: Option<String>,
    pub data_param: Option<String>,
    pub format: Option<ContentType>,
    pub rank: Option<isize>,
}

impl RouterDef {
    pub fn new(method: Option<Method>, meta: Expr, item: Item) -> Result<Self> {

        let mut params = meta.tup_items()?;
        if params.len() == 0 {
            bail!(RouteTooLessParam(0));
        }

        let method = if let Some(method) = method {
            method
        } else {
            if params.len() == 1 {
                bail!(RouteTooLessParam(1));
            }
            let method = params.remove(0).path_string()?;
            method.parse().unwrap()
        };

        let (path, query) = Self::parse_path(&params.remove(0))?;
        let mut kvmap = HashMap::new();
        for expr in params {
            let (key, value) = expr.kv()?;
            if kvmap.contains_key(&key) {
                bail!(RouteDuplicatedKey(key.clone()));
            } else {
                let value = match &key[..] {
                    "data" | "rank" | "format" => value,
                    _ => bail!(RouteKeyUnexpected(key.clone())),
                };
                kvmap.insert(key, value);
            }
        }

        let data_param = kvmap.remove("data")
            .map(|d| {
                d.as_string().map(|ds| {
                    ds.trim_left_matches("<")
                        .trim_right_matches(">")
                        .to_string()
                })
            })
            .invert()?;
        let format = kvmap.remove("format")
            .map(|value| value.as_string().map(|value| value.parse().unwrap()))
            .invert()?;
        let rank = kvmap.remove("rank")
            .map(|value| value.as_isize())
            .invert()?;

        Ok(RouterDef {
            item: item,
            method: method,
            path: path,
            query_param: query,
            data_param: data_param,
            format: format,
            rank: rank,
        })
    }

    fn parse_path(expr: &Expr) -> Result<(String, Option<String>)> {
        let raw_path = match &expr.node {
            &ExprKind::Lit(..) => expr.lit_string()?, 
            &ExprKind::Assign(ref key, ref value) => {
                let key = key.path_string()?;
                if key != "path" {
                    bail!(RouteFirstKeyNotPath(key));
                }
                value.lit_string()?
            }
            _ => bail!(RouteFirstKeyNotLitOrAssign),
        };

        if let Some(pos) = raw_path.find('?') {
            let path = &raw_path[..pos];
            let raw_query = &raw_path[pos..];
            let query = raw_query.trim_matches('"').to_string();
            Ok((path.to_string(), Some(query)))
        } else {
            Ok((raw_path, None))
        }
    }

    pub fn fn_decl(&self) -> Result<&FnDecl> {
        self.item.fn_decl()
    }

    pub fn fn_name(&self) -> String {
        self.item.ident.to_string()
    }
}