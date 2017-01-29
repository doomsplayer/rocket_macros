#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use rocket_macros::get;

#[get("/", rank = 1)]
fn get1() -> &'static str {
    "hi"
}

#[get("/", rank = 2)]
fn get2() -> &'static str {
    "hi"
}

#[get("/", rank = 3)]
fn get3() -> &'static str {
    "hi"
}

fn main() {}
