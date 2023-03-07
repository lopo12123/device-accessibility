use std::collections::HashMap;
use device_query::{CallbackGuard, Keycode as DQKey, KeyboardCallback, DeviceState, DeviceEvents};
use napi::{Env, JsFunction, Ref};
use crate::utils::{KeyEv};

// type KeyEvCallback = Fn(&DQKey) + Sync + Send + 'static;

#[napi]
pub struct Observer {
    /// guard -- key down
    // guard_keydown: Option<CallbackGuard<impl Fn(&DQKey) + Sync + Send + 'static>>,
    // guard_keydown: Option<CallbackGuard<>>,
    /// guard -- key up
    // guard_keyup: Option<>

    /// 已注册的按键事件表
    key_evs: HashMap<KeyEv, Ref<()>>,
    // mouse_evs: HashMap
}

#[napi]
impl Observer {
    #[napi(constructor)]
    pub fn new() -> Self {
        Observer {
            key_evs: HashMap::new(),
            // guard_keydown: None,
        }
    }

    /// 已注册的按键事件 (使用数组返回, 其值可视为集合, 无重复)
    #[napi(getter)]
    pub fn registered_key_events(&self) -> napi::Result<Vec<KeyEv>> {
        let mut _key_evs = vec![];
        for key in self.key_evs.keys() {
            _key_evs.push(key.clone());
        }
        Ok(_key_evs)
    }

    /// 开始监听
    pub fn start_listen(&mut self) {
        // let dq = DeviceState::new();

        // let _guard = dq.on_key_down(|ev| {
        //     println!("down: {:#?}", ev)
        // });
        // self.guard_keydown = Some(_guard);

        // let bb: Box<CallbackGuard<fn(&DQKey)>> = Box::new(_guard);
        // let guard2: CallbackGuard<Box<dyn Fn(&DQKey) + Send + Sync + 'static>> = _guard._callback;
    }

    /// 注册/更新按键监听事件 (支持组合键)
    #[napi]
    pub fn on_keys(&mut self, env: Env, keys: KeyEv, executor: JsFunction) -> napi::Result<bool> {
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
    pub fn off_keys(&mut self, env: Env, keys: KeyEv) -> napi::Result<()> {
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
    pub fn touch(&self, env: Env, keys: KeyEv) -> napi::Result<bool> {
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
        // self.stop();

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
    use device_query::{DeviceEvents, DeviceState};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let dq = DeviceState::new();
        let _guard = dq.on_key_down(|ev| {
            println!("down: {:#?}", ev)
        });

        // hellohello

        thread::sleep(Duration::from_secs(5));

        drop(_guard);//aabasfazasdfgqwert12345678

        thread::sleep(Duration::from_secs(5));
    }
}