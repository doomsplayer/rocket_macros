#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use rocket_macros::get;

#[get("/test/<one>/<two>/<three>")]
fn get(one: &str, two: usize, three: isize) -> &'static str {
    "hi"
}

fn main() {
    let _ = routes![get];
}
