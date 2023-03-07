use std::collections::HashMap;
use napi::Ref;
use crate::utils::{KeyCombination, MouseLocation};

#[napi]
pub struct Observer {
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

    pub fn listen_keys() {}

    pub fn unlisten_keys() {}
    
    pub fn test() {
        rdev::listen(|ev|{
            // ev.event_type
        });
    }
}