use std::collections::HashMap;
use enigo::{Enigo, Key, KeyboardControllable};
use napi::{Error, JsFunction, Status};

use crate::mapper::KeyboardMapper;

/// 辅助键 (ctrl / shift / alt 中的 0/1/2/3 个)
#[napi(object)]
#[derive(Debug)]
pub struct ExtraKey {
    pub ctrl: Option<bool>,
    pub alt: Option<bool>,
    pub shift: Option<bool>,
}

/// 组合键情况 (目标键 + 辅助键)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct KeyCombination {
    /// 目标键 (可用值见 mapper 文件)
    pub key: String,

    /// 辅助键 (ctrl / shift / alt 中的 0/1 个)
    #[napi(ts_type = "'ctrl' | 'alt' | 'shift'")]
    pub extra: Option<String>,
}

#[napi]
pub struct KeyboardController {
    /// 已注册的监听表
    channels: HashMap<KeyCombination, String>,
}

#[napi]
impl KeyboardController {
    #[napi(constructor)]
    pub fn new() -> Self {
        KeyboardController { channels: HashMap::new() }
    }

    /// 模拟目标按键
    /// `key` 目标键 (可用值见 mapper 文件)  
    /// `extra` 辅助键 (ctrl / shift / alt 中的 0/1/2/3 个)
    #[napi]
    pub fn simulate(&self, key: String, extra: Option<ExtraKey>) -> napi::Result<()> {
        let mut player = Enigo::new();

        match KeyboardMapper::front_to_enigo(&key) {
            Some(target_key) => {
                let mut _ctrl = false;
                let mut _alt = false;
                let mut _shift = false;

                match extra {
                    Some(v) => {
                        _ctrl = v.ctrl.is_some() && v.ctrl.unwrap();
                        _alt = v.alt.is_some() && v.alt.unwrap();
                        _shift = v.shift.is_some() && v.shift.unwrap();
                    }
                    None => {}
                }

                if _ctrl {
                    player.key_down(Key::Control);
                }
                if _alt {
                    player.key_down(Key::Alt);
                }
                if _shift {
                    player.key_down(Key::Shift);
                }

                player.key_click(target_key);

                if _shift {
                    player.key_up(Key::Shift);
                }
                if _alt {
                    player.key_up(Key::Alt);
                }
                if _ctrl {
                    player.key_up(Key::Control);
                }

                Ok(())
            }
            None => Err(Error::new(Status::InvalidArg, String::from("invalid key")))
        }
    }

    /// 监听目标
    #[napi]
    pub fn listen(&self, channel: KeyCombination, executor: JsFunction) -> napi::Result<()> {
        executor.call_without_args(None)?;
        Ok(())
    }

    /// 更新监听目标的响应执行函数
    pub fn update(&self, channel: String) {}

    /// 取消监听
    pub fn unlisten(&self) {}

    /// 销毁实例 (必须调用! 否则会过度持有引用造成内存泄露)
    pub fn dispose() {}
}

#[cfg(test)]
mod unit_test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn tt() {
        let mut m: HashMap<ExtraKey, String> = HashMap::new();

        m.insert(ExtraKey {
            ctrl: Some(true),
            alt: Some(true),
            shift: Some(true),
        }, "hello".to_string());

        m.insert(ExtraKey {
            ctrl: Some(true),
            alt: Some(true),
            shift: Some(true),
        }, "hello2".to_string());

        let p = ExtraKey {
            ctrl: Some(true),
            alt: Some(true),
            shift: Some(false),
        };

        println!("{:?}", m);

        println!("{:?}", m.get(&p));
    }
}