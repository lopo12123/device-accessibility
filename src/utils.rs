/// 辅助键 (ctrl / shift / alt 中的 0/1/2/3 个)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ExtraKey {
    pub ctrl: Option<bool>,
    /// windows/linux -- `alt`; macos -- `option`
    pub alt: Option<bool>,
    pub shift: Option<bool>,
    /// windows -- `win`; linux -- `super`; macos -- `command`
    pub meta: Option<bool>,
}

/// 组合键情况 (目标键 + 辅助键)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyCombination {
    /// 目标键 (可用值见 mapper 文件)
    pub key: String,
    /// 辅助键 见[ExtraKey]
    pub extra: Option<ExtraKey>,
}

/// 按键事件 (目标键 + 辅助键 + 按键状态)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyEv {
    /// 目标键 (可用值见 mapper 文件)
    pub key: String,
    /// 辅助键 见[ExtraKey]
    pub extra: Option<ExtraKey>,
    /// 是否是按下状态 (默认为 `false`)
    pub down: Option<bool>,
}

/// KeyEvRegister(keycode, ctrl, alt, shift, meta, down)
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyEvRegister(String, bool, bool, bool, bool, bool);

impl KeyEvRegister {
    pub fn new(keycode: String, ctrl: bool, alt: bool, shift: bool, meta: bool, down: bool) -> Self {
        KeyEvRegister(keycode, ctrl, alt, shift, meta, down)
    }

    pub fn from_key_ev(key_ev: KeyEv) -> Self {
        let (mut ctrl, mut alt, mut shift, mut meta) = (false, false, false, false);
        match key_ev.extra {
            Some(v) => {
                ctrl = match v.ctrl {
                    Some(v) => v,
                    None => false
                };
                alt = match v.alt {
                    Some(v) => v,
                    None => false
                };
                shift = match v.shift {
                    Some(v) => v,
                    None => false
                };
                meta = match v.meta {
                    Some(v) => v,
                    None => false
                };
            }
            None => {}
        };

        KeyEvRegister(key_ev.key, ctrl, alt, shift, meta, match key_ev.down {
            Some(v) => v,
            None => false
        })
    }

    pub fn to_key_ev(&self) -> KeyEv {
        KeyEv {
            key: self.0.clone(),
            extra: Some(ExtraKey {
                ctrl: Some(self.1),
                alt: Some(self.2),
                shift: Some(self.3),
                meta: Some(self.4),
            }),
            down: Some(self.5),
        }
    }
}

/// 坐标
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct MouseLocation {
    /// x 方向 (`i32`)
    pub x: i32,
    /// y 方向 (`i32`)
    pub y: i32,
}


#[cfg(test)]
mod test {
    use crate::utils::{ExtraKey, KeyEv, KeyEvRegister};

    #[test]
    fn tt() {
        let ev = KeyEv {
            key: String::from("KeyA"),
            extra: Some(ExtraKey {
                ctrl: Some(true),
                alt: Some(true),
                shift: Some(true),
                meta: Some(false),
            }),
            down: Some(true),
        };

        let p = KeyEvRegister::from_key_ev(ev);
        println!("p: {:#?}", p);
    }
}
