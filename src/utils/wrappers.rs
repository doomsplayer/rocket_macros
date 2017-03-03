use rocket::http::{Method, ContentType};
use quote::{ToTokens, Tokens};

pub struct ContentTypeWrapper(pub ContentType);
impl ToTokens for ContentTypeWrapper {
    fn to_tokens(&self, buf: &mut Tokens) {
        let (top, sub) = (&*self.0.ttype.string, &*self.0.subtype.string);
        let tokens = quote! {
            ::rocket::http::ContentType {
                ttype: ::std::borrow::Cow::Borrowed(#top),
                subtype: ::std::borrow::Cow::Borrowed(#sub),
                params: None
            }
        };
        buf.append(&tokens.to_string());
    }
}


pub struct MethodWrapper(pub Method);
impl ToTokens for MethodWrapper {
    fn to_tokens(&self, buf: &mut Tokens) {
        let tokens = match self.0 {
            Method::Connect => quote! { ::rocket::http::Method::Connect },
            Method::Delete => quote! { ::rocket::http::Method::Delete },
            Method::Get => quote! { ::rocket::http::Method::Get },
            Method::Head => quote! { ::rocket::http::Method::Head },
            Method::Options => quote! { ::rocket::http::Method::Options },
            Method::Patch => quote! { ::rocket::http::Method::Patch },
            Method::Post => quote! { ::rocket::http::Method::Post },
            Method::Put => quote! { ::rocket::http::Method::Put },
            Method::Trace => quote! { ::rocket::http::Method::Trace },
        };
        buf.append(&tokens.to_string());
    }
}