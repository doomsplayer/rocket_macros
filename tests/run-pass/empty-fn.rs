#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use rocket_macros::get;

#[get("")]
fn get() -> &'static str {
    "hi"
}

#[get("/")]
fn get_empty() {}

fn main() {}
