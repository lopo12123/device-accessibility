#[macro_use]
extern crate napi_derive;

pub mod keyboard;
mod mouse;
mod mapper;

pub mod utils;
pub mod controller;
pub mod observer;
pub mod rdev_controller;


#[napi]
pub fn helloworld() -> String {
    "Just a classic hello-world.".to_string()
}
