use enigo::{Enigo, Key as EnigoKey, MouseButton as EnigoMouse, KeyboardControllable, MouseControllable};
use napi::{Error, Status};

use crate::mapper::EnigoMapper;
use crate::utils::{KeyCombination, MouseLocation};

#[napi]
pub struct Controller {}

#[napi]
impl Controller {
    #[napi(constructor)]
    pub fn new() -> Self {
        Controller {}
    }

    /// 键盘 -- 按下
    #[napi]
    pub fn key_down(&self, key: String) -> napi::Result<()> {
        match EnigoMapper::decode_key(key) {
            Some(_key) => {
                Enigo::new().key_down(_key);
                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 键盘 -- 释放
    #[napi]
    pub fn key_up(&self, key: String) -> napi::Result<()> {
        match EnigoMapper::decode_key(key) {
            Some(_key) => {
                Enigo::new().key_up(_key);
                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 键盘 -- 点击 (即 `key_down - 20ms - key_up`)
    #[napi]
    pub fn key_click(&self, keys: KeyCombination) -> napi::Result<()> {
        let mut player = Enigo::new();

        match EnigoMapper::decode_key(keys.key) {
            Some(target_key) => {
                let mut _ctrl = false;
                let mut _alt = false;
                let mut _shift = false;
                let mut _meta = false;

                match keys.extra {
                    Some(v) => {
                        _ctrl = v.ctrl.is_some() && v.ctrl.unwrap();
                        _alt = v.alt.is_some() && v.alt.unwrap();
                        _shift = v.shift.is_some() && v.shift.unwrap();
                        _meta = v.meta.is_some() && v.meta.unwrap();
                    }
                    None => {}
                }

                if _ctrl {
                    player.key_down(EnigoKey::Control);
                }
                if _alt {
                    player.key_down(EnigoKey::Alt);
                }
                if _shift {
                    player.key_down(EnigoKey::Shift);
                }
                if _meta {
                    player.key_down(EnigoKey::Meta);
                }

                player.key_click(target_key);

                if _meta {
                    player.key_up(EnigoKey::Meta);
                }
                if _shift {
                    player.key_up(EnigoKey::Shift);
                }
                if _alt {
                    player.key_up(EnigoKey::Alt);
                }
                if _ctrl {
                    player.key_up(EnigoKey::Control);
                }

                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 键盘 -- 输入
    #[napi]
    pub fn key_type(&self, sentence: String) -> napi::Result<()> {
        Enigo::new().key_sequence(&sentence);
        Ok(())
    }

    /// 鼠标 -- 按下
    #[napi]
    pub fn mouse_down(&self, key: String) -> napi::Result<()> {
        match EnigoMapper::decode_mouse(key) {
            Some(_key) => {
                Enigo::new().mouse_down(_key);
                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 鼠标 -- 释放
    #[napi]
    pub fn mouse_up(&self, key: String) -> napi::Result<()> {
        match EnigoMapper::decode_mouse(key) {
            Some(_key) => {
                Enigo::new().mouse_up(_key);
                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 鼠标 -- 点击 (即 `mouse_down - 20ms - mouse_up`)
    #[napi]
    pub fn mouse_click(&self, key: String) -> napi::Result<()> {
        match EnigoMapper::decode_mouse(key) {
            Some(_key) => {
                Enigo::new().mouse_click(_key);
                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 鼠标 -- 滚动
    /// `scale`: 整数. 正向左/上, 负向右/下
    /// `horizontal`: 是否水平滚动, 默认 `false`
    #[napi]
    pub fn mouse_scroll(&self, scale: i32, horizontal: Option<bool>) -> napi::Result<()> {
        let is_x = match horizontal {
            Some(v) => v,
            None => false
        };
        if is_x {
            Enigo::new().mouse_scroll_x(scale);
        } else {
            Enigo::new().mouse_scroll_y(scale);
        }
        Ok(())
    }

    /// 鼠标 -- 移动
    /// `direction`: 移动方向 (默认为绝对定位: 屏幕左上角为原点,向右向下为正)
    /// `relative`: 是否使用相对定位(相对当前鼠标位置), 默认 `false`
    #[napi]
    pub fn mouse_move(&self, direction: MouseLocation, relative: Option<bool>) -> napi::Result<()> {
        let is_relative = match relative {
            Some(v) => v,
            None => false
        };
        if is_relative {
            Enigo::new().mouse_move_relative(direction.x, direction.y);
        } else {
            Enigo::new().mouse_move_to(direction.x, direction.y);
        }
        Ok(())
    }

    /// 鼠标 -- 当前坐标
    #[napi]
    pub fn mouse_location(&self) -> napi::Result<MouseLocation> {
        let location = Enigo::new().mouse_location();
        Ok(MouseLocation {
            x: location.0,
            y: location.1,
        })
    }
}

#[cfg(test)]
mod unit_test {
    use std::thread;
    use std::time::Duration;
    use super::*;

    #[test]
    fn key_test() {
        thread::sleep(Duration::from_secs(2));
        let lo = Controller::new().mouse_location().unwrap();
        println!("{:?}", lo);
        // thread::sleep(Duration::from_secs(1));
        // Controller::new().mouse_up("Right".to_string()).unwrap();
    }

    #[test]
    fn key_test2() {
        thread::sleep(Duration::from_secs(2));
        Enigo::new().key_click(EnigoKey::Layout('8'));
    }
}