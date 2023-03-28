use arboard::{Clipboard as ARBoard};
use napi::{Error, Status};
use crate::utils::{ClipboardItem, RawImage};

#[napi]
pub struct Clipboard {
    /// 历史记录存储队列的大小
    length: u32,
    /// 历史记录存储队列
    queue: Vec<ClipboardItem>,
}

#[napi]
impl Clipboard {
    /// 队列的当前长度
    #[napi(getter)]
    pub fn len(&self) -> napi::Result<u32> {
        return Ok(self.queue.len() as u32);
    }

    /// 队列的最大长度
    #[napi(getter)]
    pub fn max_len(&self) -> napi::Result<u32> {
        return Ok(self.length as u32);
    }

    /// 当前的存储队列
    #[napi(getter)]
    pub fn records(&self) -> napi::Result<Vec<ClipboardItem>> {
        Ok(self.queue.clone())
    }

    #[napi(constructor)]
    pub fn new(length: u32) -> Clipboard {
        Clipboard { length, queue: vec![] }
    }

    /// Synchronize latest item from system clipboard.
    ///
    /// Return the latest item read from system clipboard.
    #[napi]
    pub fn sync(&mut self) -> napi::Result<ClipboardItem> {
        match ARBoard::new() {
            Ok(mut operator) => {
                let item_txt = operator.get_text();
                let item_img = operator.get_image();

                // 都失败 -- 无可用项
                if item_txt.is_err() && item_img.is_err() {
                    return Err(Error::new(Status::GenericFailure, format!("There are currently no items available in the system clipboard! (neither text nor image).")));
                }

                let item = if item_txt.is_ok() {
                    ClipboardItem {
                        is_image: false,
                        text_data: Some(item_txt.unwrap()),
                        image_data: None,
                    }
                } else {
                    let img = item_img.unwrap();
                    ClipboardItem {
                        is_image: false,
                        text_data: None,
                        image_data: Some(RawImage {
                            w: img.width as u32,
                            h: img.height as u32,
                            bytes: img.bytes.to_vec(),
                        }),
                    }
                };

                match self.put_item(item.clone()) {
                    Ok(_) => Ok(item),
                    Err(_) => Err(Error::new(Status::GenericFailure, format!("Successfully fetched from the system clipboard but encountered an error while inserting into the queue!"))),
                }
            }
            Err(err) => Err(Error::new(Status::GenericFailure, format!("Failed to interact with the system clipboard! details: {}", err)))
        }
    }

    /// Get the item at `offset` (default to `0`) in the queue, `offset` equal to zero means the most recent item.
    #[napi]
    pub fn get_item(&self, offset: Option<u32>) -> napi::Result<ClipboardItem> {
        let idx = match offset {
            Some(v) => v,
            None => 0
        } as usize;

        match self.queue.get(idx) {
            Some(r) => Ok(r.clone()),
            None => Err(Error::new(Status::InvalidArg, format!("The offset cannot exceed the length of the queue!"))),
        }
    }

    /// Get the text at `offset` (default to `0`) in the queue, `offset` equal to zero means the most recent text.
    #[napi]
    pub fn get_text(&self, offset: Option<u32>) -> napi::Result<String> {
        let mut step = match offset {
            Some(v) => v,
            None => 0
        } as usize;

        // 总数比对
        if step >= self.queue.len() {
            return Err(Error::new(Status::InvalidArg, format!("The offset cannot exceed the length of the queue!")));
        }

        // 遍历查询
        for item in &self.queue {
            if !item.is_image {
                if step == 0 {
                    return Ok(item.text_data.clone().unwrap());
                } else {
                    step -= 1;
                }
            }
        }

        // 遍历结束抛出异常
        Err(Error::new(Status::InvalidArg, format!("The offset cannot exceed the length of the queue!")))
    }

    /// Get the image at `offset` (default to `0`) in the queue, `offset` equal to zero means the most recent image.
    ///
    /// Here are two example situation where the user would copy the pixel values.
    /// - When you right click on an image in a browser and then click on "Copy image" (works in Firefox and Chrome)
    /// - When you select an area of an image in an image editor software and press `Control(Command)+C`
    #[napi]
    pub fn get_image(&self, offset: Option<u32>) -> napi::Result<RawImage> {
        let mut step = match offset {
            Some(v) => v,
            None => 0
        } as usize;

        // 总数比对
        if step >= self.queue.len() {
            return Err(Error::new(Status::InvalidArg, format!("The offset cannot exceed the length of the queue!")));
        }

        // 遍历查询
        for item in &self.queue {
            if item.is_image {
                if step == 0 {
                    return Ok(item.image_data.clone().unwrap());
                } else {
                    step -= 1;
                }
            }
        }

        // 遍历结束抛出异常
        Err(Error::new(Status::InvalidArg, format!("The offset cannot exceed the length of the queue!")))
    }

    /// Put an item at the head of the queue, if the queue reaches the maximum length, the last item will be discarded.
    ///
    /// Return new length of the queue.
    #[napi]
    pub fn put_item(&mut self, item: ClipboardItem) -> napi::Result<u32> {
        // 头部插入新元素
        self.queue.insert(0, item);

        // 超出长度限制则弹出
        if self.queue.len() >= self.length as usize {
            self.queue.pop();
        }

        // 返回新长度
        Ok(self.queue.len() as u32)
    }

    /// Put an text at the head of the queue, if the queue reaches the maximum length, the last item will be discarded.
    ///
    /// Return new length of the queue.
    #[napi]
    pub fn put_text(&mut self, text: String) -> napi::Result<u32> {
        self.put_item(ClipboardItem {
            is_image: false,
            text_data: Some(text),
            image_data: None,
        })
    }

    /// Put an image at the head of the queue, if the queue reaches the maximum length, the last item will be discarded.
    ///
    /// Return new length of the queue.
    #[napi]
    pub fn put_image(&mut self, image: RawImage) -> napi::Result<u32> {
        self.put_item(ClipboardItem {
            is_image: true,
            text_data: None,
            image_data: Some(image),
        })
    }
}


#[cfg(test)]
mod unit_test {
    use arboard::Clipboard;

    #[test]
    fn get_text() {
        let mut clipboard = Clipboard::new().unwrap();

        match clipboard.get_text() {
            Ok(t) => {
                println!("text: {:?}", t);
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }

    #[test]
    fn put_text() {
        let mut clipboard = Clipboard::new().unwrap();

        match clipboard.set_text(String::from("hello")) {
            Ok(_) => println!("done."),
            Err(err) => println!("error: {:?}", err),
        }
    }

    #[test]
    fn get_image() {
        let mut clipboard = Clipboard::new().unwrap();

        match clipboard.get_image() {
            Ok(img) => {
                println!("length: {} | image: {:?}", img.bytes.len(), img);
            }
            Err(err) => {
                println!("error: {:?}", err);
            }
        }
    }

    #[test]
    fn put_image() {}

    #[test]
    fn get_both() {
        let mut clipboard = Clipboard::new().unwrap();

        let txt = clipboard.get_text();
        let img = clipboard.get_image();

        println!("txt: {:?} | img: {:?}", txt, img);
    }
}