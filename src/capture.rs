use screenshots::Screen;

/// 捕获所有屏幕图像
pub fn capture_all() -> Vec<Vec<u8>> {
    // 获取所有屏幕
    let screens = Screen::all().unwrap();

    // 储存所有屏幕截图
    let mut shoots: Vec<Vec<u8>> = vec![];

    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.to_png().unwrap();
        shoots.push(buffer);
    }

    shoots
}

#[napi]
pub struct Captor {}

#[napi]
impl Captor {
    /// Capture images of all of the user's screen
    #[napi]
    pub fn capture_all_screen() -> napi::Result<Vec<Vec<u8>>> {
        Ok(capture_all())
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;

    #[test]
    fn ttt() {
        shot();
    }

    #[test]
    fn ttt() {
        shot();
    }
}