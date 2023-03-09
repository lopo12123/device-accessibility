use napi::Either;

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
    /// 目标键
    #[napi(ts_type = "'F1' | 'F2' | 'F3' | 'F4' | 'F5' | 'F6' | 'F7' | 'F8' | 'F9' | 'F10' | 'F11' | 'F12' | 'Digit0' | 'Digit1' | 'Digit2' | 'Digit3' | 'Digit4' | 'Digit5' | 'Digit6' | 'Digit7' | 'Digit8' | 'Digit9' | 'KeyA' | 'KeyB' | 'KeyC' | 'KeyD' | 'KeyE' | 'KeyF' | 'KeyG' | 'KeyH' | 'KeyI' | 'KeyJ' | 'KeyK' | 'KeyL' | 'KeyM' | 'KeyN' | 'KeyO' | 'KeyP' | 'KeyQ' | 'KeyR' | 'KeyS' | 'KeyT' | 'KeyU' | 'KeyV' | 'KeyW' | 'KeyX' | 'KeyY' | 'KeyZ' | 'Meta' | 'Escape' | 'Tab' | 'CapsLock' | 'Shift' | 'Control' | 'Alt' | 'Space' | 'ArrowUp' | 'ArrowRight' | 'ArrowDown' | 'ArrowLeft' | 'Enter' | 'Backspace' | 'Delete' | 'Home' | 'PageUp' | 'PageDown' | 'End' | 'Backquote' | 'Minus' | 'Equal' | 'BracketLeft' | 'BracketRight' | 'Comma' | 'Period' | 'Semicolon' | 'Quote' | 'Slash' | 'BackSlash' | 'ShiftLeft' | 'ShiftRight' | 'ControlLeft' | 'ControlRight' | 'AltLeft' | 'AltRight' | 'Numpad0' | 'Numpad1' | 'Numpad2' | 'Numpad3' | 'Numpad4' | 'Numpad5' | 'Numpad6' | 'Numpad7' | 'Numpad8' | 'Numpad9' | 'NumpadAdd' | 'NumpadSubtract' | 'NumpadMultiply' | 'NumpadDivide'")]
    pub key: String,
    /// 辅助键 见[ExtraKey]
    pub extra: Option<ExtraKey>,
}

/// 按键事件 (目标键 + 辅助键 + 按键状态)
#[napi(object)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct KeyEv {
    /// 目标键
    #[napi(ts_type = "'F1' | 'F2' | 'F3' | 'F4' | 'F5' | 'F6' | 'F7' | 'F8' | 'F9' | 'F10' | 'F11' | 'F12' | 'Digit0' | 'Digit1' | 'Digit2' | 'Digit3' | 'Digit4' | 'Digit5' | 'Digit6' | 'Digit7' | 'Digit8' | 'Digit9' | 'KeyA' | 'KeyB' | 'KeyC' | 'KeyD' | 'KeyE' | 'KeyF' | 'KeyG' | 'KeyH' | 'KeyI' | 'KeyJ' | 'KeyK' | 'KeyL' | 'KeyM' | 'KeyN' | 'KeyO' | 'KeyP' | 'KeyQ' | 'KeyR' | 'KeyS' | 'KeyT' | 'KeyU' | 'KeyV' | 'KeyW' | 'KeyX' | 'KeyY' | 'KeyZ' | 'Meta' | 'Escape' | 'Tab' | 'CapsLock' | 'Shift' | 'Control' | 'Alt' | 'Space' | 'ArrowUp' | 'ArrowRight' | 'ArrowDown' | 'ArrowLeft' | 'Enter' | 'Backspace' | 'Delete' | 'Home' | 'PageUp' | 'PageDown' | 'End' | 'Backquote' | 'Minus' | 'Equal' | 'BracketLeft' | 'BracketRight' | 'Comma' | 'Period' | 'Semicolon' | 'Quote' | 'Slash' | 'BackSlash' | 'ShiftLeft' | 'ShiftRight' | 'ControlLeft' | 'ControlRight' | 'AltLeft' | 'AltRight' | 'Numpad0' | 'Numpad1' | 'Numpad2' | 'Numpad3' | 'Numpad4' | 'Numpad5' | 'Numpad6' | 'Numpad7' | 'Numpad8' | 'Numpad9' | 'NumpadAdd' | 'NumpadSubtract' | 'NumpadMultiply' | 'NumpadDivide'")]
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
