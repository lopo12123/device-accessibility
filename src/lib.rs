#[macro_use]
extern crate napi_derive;

mod mapper;

pub mod utils;
pub mod controller;

mod observer;


#[napi]
pub fn helloworld() -> String {
    "Just a classic hello-world.".to_string()
}
