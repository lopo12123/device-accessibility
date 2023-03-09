use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
    thread,
};
use device_query::{DeviceState, DeviceEvents, DeviceQuery, Keycode as DQKey};
use napi::{Error, JsFunction, Status};
use napi::threadsafe_function::{
    ErrorStrategy,
    ThreadsafeFunction,
    ThreadsafeFunctionCallMode,
};
use crate::mapper::DQMapper;
use crate::utils::{ExtraKey, KeyEv, KeyEvRegister};

/// 子线程检查间隔 -- ms
const LOOP_GAP: u64 = 1000;

#[napi]
pub struct Observer {
    /// 是否监听 -- 为 `false` 表示结束
    guard: Arc<Mutex<bool>>,

    /// 按键事件监听注册表
    key_evs: Arc<Mutex<HashMap<KeyEvRegister, ThreadsafeFunction<()>>>>,

    /// 监听全部事件的回调函数
    global_key_cb: Arc<Mutex<Option<ThreadsafeFunction<KeyEv>>>>,
}

#[napi]
impl Observer {
    /// thread-safe function test
    #[napi(ts_args_type = "callback: (err: null | Error, result: string) => void")]
    pub fn tsfn_test(&self, callback: JsFunction) -> napi::Result<()> {
        let tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
            .create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?;

        let fn1 = tsfn.clone();
        thread::spawn(move || {
            fn1.call(Ok(String::from("thread1 -- immediate")), ThreadsafeFunctionCallMode::NonBlocking);
        });

        let fn2 = tsfn.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            fn2.call(Ok(String::from("thread2 -- sleep 1s")), ThreadsafeFunctionCallMode::NonBlocking);
        });

        let fn3 = tsfn.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            fn3.call(Ok(String::from("thread3 -- sleep 2s")), ThreadsafeFunctionCallMode::NonBlocking);
            fn3.call(Ok(String::from("thread3 -- sleep 2s -- multiple call")), ThreadsafeFunctionCallMode::NonBlocking);
        });

        Ok(())
    }

    /// 初始化 -- 子线程中监听
    fn setup(&self) {
        // 终止信号
        let signal = self.guard.clone();
        // 全部按键事件监听回调
        let keydown_cb_all = self.global_key_cb.clone();
        let keyup_cb_all = self.global_key_cb.clone();
        // 特定按键事件监听回调
        let keydown_cb_spec = self.key_evs.clone();
        let keyup_cb_spec = self.key_evs.clone();

        thread::spawn(move || {
            // 状态监听
            let listener = DeviceState::new();

            // 按键按下监听
            let _guard = listener.on_key_down(move |keycode| {
                // 状态扫描
                let scanner = DeviceState::new();

                // 对全部事件的监听
                match keydown_cb_all.lock().unwrap().deref() {
                    Some(cb) => {
                        cb.call(Ok(KeyEv {
                            key: match DQMapper::encode_key(keycode) {
                                Some(v) => v,
                                None => String::from("Unknown")
                            },
                            extra: None,
                            down: Some(true),
                        }), ThreadsafeFunctionCallMode::NonBlocking);
                    }
                    None => {}
                };

                // 对注册事件的监听
                let key = DQMapper::encode_key(keycode).unwrap();
                let (mut ctrl, mut alt, mut shift, mut meta) = (false, false, false, false);
                for _key in scanner.get_keys() {
                    match _key {
                        DQKey::LControl | DQKey::RControl => ctrl = true,
                        DQKey::LAlt | DQKey::RAlt => alt = true,
                        DQKey::LShift | DQKey::RShift => shift = true,
                        DQKey::Meta => meta = true,
                        _ => {}
                    };
                }
                let register_ev = KeyEvRegister::new(key, ctrl, alt, shift, meta, true);

                match keydown_cb_spec.lock().unwrap().get(&register_ev) {
                    Some(cb) => {
                        cb.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
                    }
                    None => {}
                }
            });

            // 按键释放监听
            let _guard = listener.on_key_up(move |keycode| {
                // 状态扫描
                let scanner = DeviceState::new();

                // 对全部事件的监听
                match keyup_cb_all.lock().unwrap().deref() {
                    Some(cb) => {
                        cb.call(Ok(KeyEv {
                            key: match DQMapper::encode_key(keycode) {
                                Some(v) => v,
                                None => String::from("Unknown")
                            },
                            extra: None,
                            down: Some(false),
                        }), ThreadsafeFunctionCallMode::NonBlocking);
                    }
                    None => {}
                };

                // 对注册事件的监听
                let key = DQMapper::encode_key(keycode).unwrap();
                let (mut ctrl, mut alt, mut shift, mut meta) = (false, false, false, false);
                for _key in scanner.get_keys() {
                    match _key {
                        DQKey::LControl | DQKey::RControl => ctrl = true,
                        DQKey::LAlt | DQKey::RAlt => alt = true,
                        DQKey::LShift | DQKey::RShift => shift = true,
                        DQKey::Meta => meta = true,
                        _ => {}
                    };
                }
                let register_ev = KeyEvRegister::new(key, ctrl, alt, shift, meta, false);

                match keyup_cb_spec.lock().unwrap().get(&register_ev) {
                    Some(cb) => {
                        cb.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
                    }
                    None => {}
                }
            });

            // 监听结束判断
            while *signal.lock().unwrap() {
                thread::sleep(Duration::from_millis(LOOP_GAP));
                println!("tick");
            };

            println!("listen finished.")
        });
    }

    #[napi(constructor)]
    pub fn new() -> Self {
        let instance = Observer {
            guard: Arc::new(Mutex::new(true)),
            key_evs: Arc::new(Mutex::new(HashMap::new())),
            global_key_cb: Arc::new(Mutex::new(None)),
        };

        instance.setup();

        instance
    }

    /// 检查键名是否合法
    #[napi]
    pub fn check_key(&self, key: String) -> napi::Result<bool> {
        match DQMapper::decode_key(key) {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    /// 已注册的按键事件 (使用数组返回, 其值可视为集合, 无重复)
    #[napi(getter)]
    pub fn registered_keys(&self) -> napi::Result<Vec<KeyEv>> {
        let mut _key_evs = vec![];

        let evs = self.key_evs.lock().unwrap();
        for key in evs.keys() {
            _key_evs.push(key.to_key_ev());
        }

        Ok(_key_evs)
    }

    /// 注册/更新按键监听事件 (支持组合键)
    #[napi]
    pub fn on_key(&mut self, keys: KeyEv, callback: JsFunction) -> napi::Result<()> {
        if self.check_key(keys.key.clone()).unwrap() {
            let mut evs = self.key_evs.lock().unwrap();

            let register_ev = KeyEvRegister::from_key_ev(keys);
            evs.insert(register_ev, callback.create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?);
            Ok(())
        } else {
            Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 移除已注册的监听
    #[napi]
    pub fn off_key(&mut self, keys: KeyEv) -> napi::Result<()> {
        if self.check_key(keys.key.clone()).unwrap() {
            let mut evs = self.key_evs.lock().unwrap();
            let register_ev = KeyEvRegister::from_key_ev(keys);
            evs.remove(&register_ev);
            Ok(())
        } else {
            Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 注册/更新对全部按键的监听事件
    #[napi(ts_args_type = "callback: (err: null | Error, keycode: string) => void")]
    pub fn on_key_all(&self, callback: JsFunction) -> napi::Result<()> {
        let tsfn = callback.create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        })?;

        *self.global_key_cb.lock().unwrap() = Some(tsfn);

        Ok(())
    }

    /// 移除对全部按键的监听事件
    #[napi]
    pub fn off_key_all(&self) -> napi::Result<()> {
        *self.global_key_cb.lock().unwrap() = None;

        Ok(())
    }

    /// 主动触发已注册的按键事件 (返回值表示该组合键是否已注册)
    #[napi]
    pub fn touch(&self, keys: KeyEv) -> napi::Result<bool> {
        if self.check_key(keys.key.clone()).unwrap() {
            let evs = self.key_evs.lock().unwrap();
            let register_ev = KeyEvRegister::from_key_ev(keys);
            match evs.get(&register_ev) {
                Some(tsfn) => {
                    tsfn.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
                    Ok(true)
                }
                None => Ok(false)
            }
        } else {
            Err(Error::new(Status::InvalidArg, format!("Invalid Key!")))
        }
    }

    /// 结束监听 (必须调用! 否则会由于过度持有引用造成内存泄露)
    #[napi]
    pub fn dispose(&mut self) -> napi::Result<()> {
        let mut guard = self.guard.lock().unwrap();

        // 已结束 -- 直接返回
        if *guard {
            // 发送停止信号
            *guard = false;

            // 释放全部按键的回调函数
            *self.global_key_cb.lock().unwrap() = None;
            // 释放注册表中的回调函数
            let mut evs = self.key_evs.lock().unwrap();
            evs.clear();
        }

        Ok(())
    }
}

#[cfg(test)]
mod unit_test {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use device_query::DeviceQuery;

    #[test]
    fn test() {
        let dq = DeviceState::new();
        let _guard = dq.on_key_down(|ev| {
            let scanner = DeviceState::new();
            println!("keydown: {} | ctrl: {}", ev, scanner.get_keys().contains(&DQKey::LControl));
        });
        let _guard = dq.on_key_up(|ev| {
            let scanner = DeviceState::new();
            println!("keyup {} | ctrl: {}", ev, scanner.get_keys().contains(&DQKey::LControl));
        });

        thread::sleep(Duration::from_secs(5));

        drop(_guard);//aabasfazasdfgqwert123456123ac

        thread::sleep(Duration::from_secs(5));
    }

    #[test]
    fn thread_test() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("from spawned thread: {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("thread created but not joined.");

        thread::sleep(Duration::from_secs(5));


        for i in 1..5 {
            println!("from main thread: {}", i);
            thread::sleep(Duration::from_secs(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn mutex_test() {
        let n = Arc::new(Mutex::new(1));

        let signal = n.clone();
        let guard = thread::spawn(move || {
            for t in 1..5 {
                let mut n = signal.lock().unwrap();
                println!("t: {}; n: {}", t, n);
                *n += 1;
                thread::sleep(Duration::from_secs(1));
            }
        });

        guard.join().unwrap();
    }
}