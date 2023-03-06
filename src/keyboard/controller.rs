use std::collections::{HashMap, HashSet};
use enigo::{Enigo, Key, KeyboardControllable};
use napi::{Env, Error, JsFunction, Ref, Status};

use crate::mapper::KeyboardMapper;
use crate::utils::{ExtraKey, KeyCombination};


/// 键盘控制类 (监听 + 模拟)
#[napi]
pub struct KeyboardController {
    /// 已注册的监听表
    registered_keys: HashMap<KeyCombination, Ref<()>>,
    /// 当前按下的键
    pressed_key: HashSet<String>,
    // 内部监听器
    // worker: String
}

#[napi]
impl KeyboardController {
    #[napi(constructor)]
    pub fn new() -> Self {
        KeyboardController {
            registered_keys: HashMap::new(),
            pressed_key: HashSet::new(),
        }
    }

    /// 已注册的组合键列表 (使用数组返回但其值为集合, 可保证无重复)
    #[napi(getter)]
    pub fn registered(&self) -> napi::Result<Vec<KeyCombination>> {
        let mut _keys: Vec<KeyCombination> = vec![];

        for key in self.registered_keys.keys() {
            _keys.push(key.clone());
        }

        Ok(_keys)
    }

    /// 模拟目标按键
    /// `key` 目标键 (可用值见 mapper 文件)  
    /// `extra` 辅助键 (ctrl / shift / alt 中的 0/1/2/3 个)
    #[napi]
    pub fn simulate(&self, keys: KeyCombination) -> napi::Result<()> {
        let mut player = Enigo::new();

        match KeyboardMapper::front_to_enigo(&keys.key) {
            Some(target_key) => {
                let mut _ctrl = false;
                let mut _alt = false;
                let mut _shift = false;

                match keys.extra {
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
            None => Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 注册按键监听事件 (支持组合键)
    #[napi]
    pub fn listen(&mut self, env: Env, keys: KeyCombination, executor: JsFunction) -> napi::Result<bool> {
        match env.create_reference(executor) {
            Ok(js_ref) => {
                self.registered_keys.insert(keys, js_ref);
                Ok(true)
            }
            Err(_) => Ok(false)
        }
    }

    /// 更新(不存在则回退为注册)监听目标的响应执行函数
    #[napi]
    pub fn update(&mut self, env: Env, keys: KeyCombination, executor: JsFunction) -> napi::Result<bool> {
        // 释放旧的引用
        match self.registered_keys.get_mut(&keys) {
            Some(js_ref) => {
                js_ref.unref(env)?;
            }
            None => {}
        }

        self.listen(env, keys, executor)
    }

    /// 取消已注册的监听
    #[napi]
    pub fn unlisten(&mut self, env: Env, keys: KeyCombination) -> napi::Result<()> {
        // 释放旧的引用
        match self.registered_keys.get_mut(&keys) {
            Some(js_ref) => {
                js_ref.unref(env)?;
            }
            None => {}
        }

        // 取消注册
        self.registered_keys.remove(&keys);

        Ok(())
    }

    /// 主动触发已注册的按键事件 (返回值表示该组合键是否已注册)
    #[napi]
    pub fn touch(&self, env: Env, keys: KeyCombination) -> napi::Result<bool> {
        match self.registered_keys.get(&keys) {
            Some(js_ref) => {
                let executor: JsFunction = env.get_reference_value(&js_ref)?;
                executor.call_without_args(None)?;
                Ok(true)
            }
            None => Ok(false)
        }
    }

    /// 销毁实例 (必须调用! 否则可能会由于过度持有引用造成内存泄露)
    #[napi]
    pub fn dispose(&mut self, env: Env) -> napi::Result<()> {
        for registered_key in &mut self.registered_keys {
            registered_key.1.unref(env)?;
        }

        self.registered_keys.clear();

        Ok(())
    }
}

#[cfg(test)]
mod unit_test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn tt() {
        let mut m: HashMap<KeyCombination, String> = HashMap::new();

        m.insert(KeyCombination {
            key: "key1".to_string(),
            extra: Some(ExtraKey {
                ctrl: Some(true),
                alt: Some(true),
                shift: Some(true),
            }),
        }, "hello".to_string());


        let p = KeyCombination {
            key: "key1".to_string(),
            extra: Some(ExtraKey {
                ctrl: Some(true),
                alt: Some(true),
                shift: Some(true),
            }),
        };

        println!("{:?}", m);

        println!("{:?}", m.get(&p));
    }
}