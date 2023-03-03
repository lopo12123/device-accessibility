#[napi]
pub struct KeyboardListener {}

#[napi]
impl KeyboardListener {
    #[napi(constructor)]
    pub fn new() -> Self {
        KeyboardListener {}
    }

    #[napi]
    pub fn async_task() {

    }
}