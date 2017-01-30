#![feature(proc_macro)]
extern crate rocket;
extern crate rocket_macros;

use std::path::PathBuf;
use rocket::http::uri::SegmentError;
use rocket_macros::post;

#[post("/<a>/<b..>")]
fn get(a: String, b: PathBuf) -> String {
    format!("{}/{}", a, b.to_string_lossy())
}

#[post("/<a>/<b..>")]
fn get2(a: String, b: Result<PathBuf, SegmentError>) -> String {
    format!("{}/{}", a, b.unwrap().to_string_lossy())
}

fn main() {}

// fn get(a: String, b: PathBuf) -> String {
//     format!("{}/{}", a, b.to_string_lossy())
// }
// fn rocket_route_fn_get<'_b>(_req: &'_b ::rocket::Request,
//                             _data: ::rocket::Data)
//                             -> ::rocket::handler::Outcome<'_b> {
//     let rocket_param_a: String = match match _req.get_param_str(0usize) {
//         Some ( s ) => < String as :: rocket :: request :: FromParam > :: from_param (
// s ),
//         None => return ::rocket::Outcome::Forward(_data),
//     } {
//         Ok(v) => v,
//         Err(e) => {
//             println!("    => Failed to parse '{}': {:?}", stringify!(a), e);
//             return ::rocket::Outcome::Forward(_data);
//         }
//     };
//     let rocket_param_b: PathBuf = match match _req.get_raw_segments(1usize) {
//         Some ( s ) => < PathBuf as :: rocket :: request :: FromSegments > ::
// from_segments ( s ),
//         None => return ::rocket::Outcome::Forward(_data),
//     } {
//         Ok(v) => v,
//         Err(e) => {
//             println!("    => Failed to parse '{}': {:?}", stringify!(b), e);
//             return ::rocket::Outcome::Forward(_data);
//         }
//     };
//     let responder = get(a, b);
//     ::rocket::handler::Outcome::of(responder)
// }
// # [
// allow ( non_upper_case_globals ) ]
// pub static static_rocket_route_info_for_get: ::rocket::StaticRouteInfo =
//     ::rocket::StaticRouteInfo {
//         method: ::rocket::http::Method::Post,
//         path: "/<a>/<b..>",
//         handler: rocket_route_fn_get,
//         format: None,
//         rank: None,
//     };

// fn get2(a: String, b: Result<PathBuf, SegmentError>) -> String {
//     format!("{}/{}", a, b.unwrap().to_string_lossy())
// }
// fn rocket_route_fn_get2<'_b>(_req: &'_b ::rocket::Request,
//                              _data: ::rocket::Data)
//                              -> ::rocket::handler::Outcome<'_b> {
//     let rocket_param_a: String = match match _req.get_param_str(0usize) {
//         Some ( s ) => < String as :: rocket :: request :: FromParam > :: from_param (s ),
//         None => return ::rocket::Outcome::Forward(_data),
//     } {
//         Ok(v) => v,
//         Err(e) => {
//             println!("    => Failed to parse '{}': {:?}", stringify!(a), e);
//             return ::rocket::Outcome::Forward(_data);
//         }
//     };
//     let rocket_param_b: Result<PathBuf, SegmentError> =
//         match match _req.get_raw_segments(1usize) {
//             Some ( s ) => < Result < PathBuf , SegmentError > as :: rocket :: request ::FromSegments > :: from_segments ( s ),
//             None => return ::rocket::Outcome::Forward(_data),
//         } {
//             Ok(v) => v,
//             Err(e) => {
//                 println!("    => Failed to parse '{}': {:?}", stringify!(b), e);
//                 return ::rocket::Outcome::Forward(_data);
//             }
//         };
//     let responder = get2(a, b);
//     ::rocket::handler::Outcome::of(responder)
// }
// # [allow ( non_upper_case_globals ) ]
// pub static static_rocket_route_info_for_get2: ::rocket::StaticRouteInfo =
//     ::rocket::StaticRouteInfo {
//         method: ::rocket::http::Method::Post,
//         path: "/<a>/<b..>",
//         handler: rocket_route_fn_get2,
//         format: None,
//         rank: None,
//     };