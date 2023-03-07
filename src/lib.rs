#[macro_use]
extern crate napi_derive;

mod mapper;

pub mod keyboard;

pub mod utils;
pub mod controller;
pub mod observer;

mod rdev_controller;


#[napi]
pub fn helloworld() -> String {
    "Just a classic hello-world.".to_string()
}
