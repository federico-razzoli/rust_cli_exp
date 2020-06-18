use std::collections::HashMap;

pub mod long_range_scanner {
    extern crate console;
    use self::console::Style;

    pub fn scan() {
        let mut styles: [String; 3] = [
            String::from(""),
            String::from(""),
            String::from(""),
        ];

        let alert = Style::new().red().bold();
        println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
    }
}
