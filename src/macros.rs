macro_rules! define_key {
    () => {
        "println!(\"hello world\");"
    };
}

macro_rules! define_mouse {
    () => {
        "'Left' | 'Middle' | 'Right'"
    };
}

#[cfg(test)]
mod unit_test {
    #[test]
    fn test() {
        // println!("{}", define_mouse!());
        define_key!();
    }
}