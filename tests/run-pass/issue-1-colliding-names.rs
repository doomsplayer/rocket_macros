#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use rocket_macros::get;

#[get("/<todo>")]
fn todo(todo: &str) -> &str {
    todo
}

fn main() {
    let _ = routes![todo];
}
