#![feature(proc_macro)]
extern crate rocket;
#[macro_use]
extern crate rocket_macros;

#[derive(Debug, PartialEq, FromForm)]
struct TodoTask<'a> {
    c: &'a str,
    description: String,
    completed: bool,
}

fn main() {}
