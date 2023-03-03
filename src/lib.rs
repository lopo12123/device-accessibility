mod keyboard;
mod mouse;
mod mapper;

#[macro_use]
extern crate napi_derive;

pub use keyboard::{ExtraKey, KeyboardController};

#[napi]
pub fn hello_world() -> String {
    "Just a classic hello-world.".to_string()
}
