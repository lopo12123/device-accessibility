use std::{
    collections::HashMap,
    time::Duration,
    thread,
};
use std::sync::{Arc, Mutex};
use device_query::{
    DeviceState, DeviceEvents,
};
use napi::{Env, JsFunction, Ref};
use napi::threadsafe_function::{
    ErrorStrategy,
    ThreadsafeFunction,
    ThreadsafeFunctionCallMode,
};
use crate::mapper::DQMapper;
use crate::utils::{KeyEv};

#[napi]
pub struct Observer {
    /// 子进程守护 -- 为 `false` 表示结束守护
    guard: Arc<Mutex<bool>>,

    /// 已注册的按键事件表
    key_evs: HashMap<KeyEv, Ref<()>>,
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

    #[napi(constructor)]
    pub fn new() -> Self {
        Observer {
            guard: Arc::new(Mutex::new(true)),
            key_evs: HashMap::new(),
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

    /// 开始监听 todoajhdf
    #[napi(ts_args_type = "callback: (err: null | Error, result: string) => void")]
    pub fn start(&self, callback: JsFunction) -> napi::Result<()> {
        let signal = self.guard.clone();
        let tsfn: ThreadsafeFunction<String, ErrorStrategy::CalleeHandled> = callback
            .create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?;
        let evcb = tsfn.clone();

        thread::spawn(move || {
            let _guard = DeviceState::new().on_key_down(move |ev| {
                let keycode = match DQMapper::encode_key(ev) {
                    Some(v) => v,
                    None => String::from("Unknown"),
                };
                evcb.call(Ok(keycode), ThreadsafeFunctionCallMode::NonBlocking);
            });

            while *signal.lock().unwrap() {
                // 检查间隔 -- 100ms
                thread::sleep(Duration::from_millis(100));
            };
        });

        Ok(())
    }

    /// 结束监听
    #[napi]
    pub fn stop(&mut self) -> napi::Result<()> {
        *self.guard.lock().unwrap() = false;

        Ok(())
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
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let dq = DeviceState::new();
        let _guard = dq.on_key_down(|ev| {
            println!("{}", ev);
        });

        // hellohello

        thread::sleep(Duration::from_secs(5));

        drop(_guard);//aabasfazasdfgqwert12345678

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