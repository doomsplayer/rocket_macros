#![feature(proc_macro)]
extern crate rocket;
#[macro_use]
extern crate rocket_macros;

use rocket_macros::post;

#[post("/", format = "application/x-custom")]
fn get() -> &'static str {
    "hi"
}

fn main() {}
