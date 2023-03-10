#[macro_use]
extern crate napi_derive;

use crate::mapper::{EnigoMapper};

mod mapper;

pub mod utils;
pub mod controller;
pub mod observer;

/// 检查键盘按键名是否合法
#[napi]
pub fn check_key(key: String) -> napi::Result<bool> {
    match EnigoMapper::decode_key(key) {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}

/// 检查鼠标按键名是否合法
#[napi]
pub fn check_mouse(key: String) -> napi::Result<bool> {
    match EnigoMapper::decode_mouse(key) {
        Some(_) => Ok(true),
        None => Ok(false)
    }
}

#[napi]
pub fn helloworld() -> String {
    "Just a classic hello-world.".to_string()
}
