use rdev::{simulate, listen};

#[napi]
pub struct RdevController {}

#[napi]
impl RdevController {
    #[napi(constructor)]
    pub fn new() -> Self {
        RdevController {}
    }

    pub fn simulate() {
        // rdev::simulate();
    }

    pub fn listen() {}
}

#[cfg(test)]
mod unit_test {
    use rdev::{EventType, Key};

    #[test]
    fn test() {
        std::thread::sleep(std::time::Duration::from_secs(2));
        rdev::simulate(&EventType::KeyPress(Key::KeyA));
    }
}