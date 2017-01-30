#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use rocket::{Error, Request};
use rocket_macros::error;

#[error(404)]
pub fn err0() -> &'static str {
    "hi"
}

#[error(404)]
pub fn err1a(_err: Error) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err1b(_req: &Request) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err2a(_err: Error, _req: &Request) -> &'static str {
    "hi"
}

#[error(404)]
pub fn err2b<'a>(_err: Error, _req: &'a Request) -> &'a str {
    "hi"
}

fn main() {}
