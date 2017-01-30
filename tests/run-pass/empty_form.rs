#![feature(proc_macro)]
extern crate rocket;
#[macro_use]
extern crate rocket_macros;

use rocket::request::FromForm;

#[derive(PartialEq, Debug, FromForm)]
struct Form {  }

fn main() {
    // Same number of arguments: simple case.
    let task = Form::from_form_string("");
    assert_eq!(task, Ok(Form {}));
}
