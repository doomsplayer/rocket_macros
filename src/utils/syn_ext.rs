use syn::{Expr, ExprKind, FnArg, FnDecl, Item, ItemKind, Lit, Pat, Ty};
use errors::*;
use errors::ErrorKind::*;

pub trait ExprExt {
    fn path_string(&self) -> Result<String>;
}

impl ExprExt for Expr {
    fn path_string(&self) -> Result<String> {
        match &self.node {
            &ExprKind::Path(_, ref path) => {
                if path.segments.len() != 1 {
                    bail!(PathMultipleSegments)
                } else {
                    Ok(path.segments[0].ident.to_string())
                }
            }
            _ => bail!(ExprNotAPath),
        }
    }
}


pub trait PatExt {
    fn path_string(&self) -> Result<String>;
    fn ident_string(&self) -> Result<String>;
}

impl PatExt for Pat {
    fn path_string(&self) -> Result<String> {
        match self {
            &Pat::Path(_, ref path) => {
                if path.segments.len() != 1 {
                    bail!(PathMultipleSegments)
                }
                Ok(path.segments[0].ident.to_string())
            }
            _ => bail!(PatNotAPath),
        }
    }
    fn ident_string(&self) -> Result<String> {
        match self {
            &Pat::Ident(_, ref ident, _) => Ok(ident.to_string()),
            _ => bail!(PatNotAPath),
        }
    }
}


pub trait LitString {
    fn lit_string(&self) -> Result<String>;
}

impl LitString for Expr {
    fn lit_string(&self) -> Result<String> {
        match &self.node {
            &ExprKind::Lit(Lit::Str(ref value, _)) => Ok(value.to_string()),
            _ => bail!(ExprNotALit),
        }
    }
}

pub trait KV {
    fn kv(&self) -> Result<(String, &Lit)>;
}

impl KV for Expr {
    fn kv(&self) -> Result<(String, &Lit)> {
        match &self.node {
            &ExprKind::Assign(ref key, ref value) => {
                let lit = match value.node {
                    ExprKind::Lit(ref lit) => lit,
                    _ => bail!(AssignValueInvalid),
                };
                Ok((key.path_string()?, lit))
            } 
            _ => bail!(ExprNotAAssign),
        }
    }
}

pub trait FnDeclExt {
    fn find_input_ty(&self, name: &str) -> Option<&Ty>;
    fn arg_names(&self) -> Vec<String>;
}

impl FnDeclExt for FnDecl {
    fn find_input_ty(&self, name: &str) -> Option<&Ty> {
        for arg in &self.inputs {
            match arg {
                &FnArg::Captured(ref pat, ref ty) => {
                    match pat {
                        &Pat::Ident(_, ref ident, ..) if ident.to_string() == name => {
                            return Some(ty)
                        }
                        _ => continue,
                    }
                }
                _ => continue,
            }
        }
        None
    }
    fn arg_names(&self) -> Vec<String> {
        self.inputs
            .iter()
            .map(|arg| {
                let pat = match arg {
                    &FnArg::Captured(ref pat, _) => pat,
                    &FnArg::Ignored(_) => panic!(""),
                    _ => panic!(""),
                };
                match pat {
                    &Pat::Ident(_, ref ident, ..) => ident.to_string(),
                    _ => panic!(""),
                }
            })
            .collect()
    }
}

pub trait TupItems {
    fn tup_items(&self) -> Result<Vec<&Expr>>;
    fn tup_len(&self) -> Result<usize>;
}

impl TupItems for Expr {
    fn tup_items(&self) -> Result<Vec<&Expr>> {
        let exprs: Vec<&Expr> = match &self.node {
            &ExprKind::Tup(ref exprs) => exprs.iter().map(|e| &*e).collect(),
            &ExprKind::Paren(ref boxed_expr) => vec![&*boxed_expr],
            _ => bail!(ExprNotATupOrParen),
        };
        Ok(exprs)
    }
    fn tup_len(&self) -> Result<usize> {
        let len = match &self.node {
            &ExprKind::Tup(ref exprs) => exprs.len(),
            &ExprKind::Paren(_) => 1,
            _ => bail!(ExprNotATupOrParen),
        };
        Ok(len)
    }
}

pub trait FnArgExt {
    fn name(&self) -> Result<String>;
    fn ty(&self) -> Result<&Ty>;
}

impl FnArgExt for FnArg {
    fn name(&self) -> Result<String> {
        let name = match self {
            &FnArg::Captured(ref pat, _) => pat.ident_string()?,
            _ => bail!(FnArgHasNoName),
        };
        Ok(name)
    }

    fn ty(&self) -> Result<&Ty> {
        let name = match self {
            &FnArg::Captured(_, ref ty) => ty,
            &FnArg::Ignored(ref ty) => ty,
            _ => bail!(FnArgHasNoTy),
        };
        Ok(name)
    }
}

pub trait LitExt {
    fn as_isize(&self) -> Result<isize>;
    fn as_string(&self) -> Result<String>;
}

impl LitExt for Lit {
    fn as_isize(&self) -> Result<isize> {
        match self {
            &Lit::Int(i, _) => Ok(i as isize),
            _ => bail!(LitNotAnISize),
        }
    }
    fn as_string(&self) -> Result<String> {
        match self {
            &Lit::Str(ref s, _) => Ok(s.clone()),
            _ => bail!(LitNotAString),
        }
    }
}

pub trait TyExt {
    fn strip_lifetime(&self) -> Ty;
}

impl TyExt for Ty {
    fn strip_lifetime(&self) -> Ty {
        match self {
            &Ty::Rptr(_, ref ty) => Ty::Rptr(None, ty.clone()),
            other_ty => other_ty.clone(),
        }
    }
}

pub trait ItemExt {
    fn fn_decl(&self) -> Result<&FnDecl>;
}

impl ItemExt for Item {
    fn fn_decl(&self) -> Result<&FnDecl> {
        match &self.node {
            &ItemKind::Fn(ref decl, ..) => Ok(&**decl),
            _ => bail!(ItemNotAFnDecl),
        }
    }
}