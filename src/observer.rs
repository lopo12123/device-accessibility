use std::collections::HashMap;
use napi::{Env, JsFunction, Ref};
use crate::utils::{KeyCombination};

#[napi]
pub struct Observer {
    /// 已注册的按键事件表
    key_evs: HashMap<KeyCombination, Ref<()>>,
    // mouse_evs: HashMap
}

#[napi]
impl Observer {
    #[napi(constructor)]
    pub fn new() -> Self {
        Observer {
            key_evs: HashMap::new()
        }
    }

    /// 开始监听
    pub fn start(&self) {}

    /// 停止监听
    pub fn stop(&self) {}

    /// 注册/更新按键监听事件 (支持组合键)
    #[napi]
    pub fn on_keys(&mut self, env: Env, keys: KeyCombination, executor: JsFunction) -> napi::Result<bool> {
        // 释放旧的引用
        match self.key_evs.get_mut(&keys) {
            Some(js_ref) => {
                js_ref.unref(env)?;
            }
            None => {}
        }

        match env.create_reference(executor) {
            Ok(js_ref) => {
                self.key_evs.insert(keys, js_ref);
                Ok(true)
            }
            Err(_) => Ok(false)
        }
    }

    /// 移除已注册的监听
    #[napi]
    pub fn off_keys(&mut self, env: Env, keys: KeyCombination) -> napi::Result<()> {
        // 释放旧的引用
        match self.key_evs.get_mut(&keys) {
            Some(js_ref) => {
                js_ref.unref(env)?;
            }
            None => {}
        }

        // 取消注册
        self.key_evs.remove(&keys);

        Ok(())
    }

    /// 主动触发已注册的按键事件 (返回值表示该组合键是否已注册)
    #[napi]
    pub fn touch(&self, env: Env, keys: KeyCombination) -> napi::Result<bool> {
        match self.key_evs.get(&keys) {
            Some(js_ref) => {
                let executor: JsFunction = env.get_reference_value(js_ref)?;
                executor.call_without_args(None)?;
                Ok(true)
            }
            None => Ok(false)
        }
    }

    /// 销毁实例 (必须调用! 否则可能会由于过度持有引用造成内存泄露)
    #[napi]
    pub fn dispose(&mut self, env: Env) -> napi::Result<()> {
        // 停止监听
        self.stop();

        // 释放引用
        for key_ev in &mut self.key_evs {
            key_ev.1.unref(env)?;
        }

        // 移除记录
        self.key_evs.clear();

        // 结束
        Ok(())
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn test() {}
}