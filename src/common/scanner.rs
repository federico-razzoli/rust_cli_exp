pub mod long_range_scanner {
    use std::collections::HashMap;

    extern crate console;
    use self::console::Style;


    pub fn scan() {
        let mut styles = HashMap::new();
        styles.insert("normal", Style::new());
        styles.insert("alert", Style::new().red().bold());
        println!("fooo!");

        let alert = Style::new().red().bold();
        let alert = styles.get("alert").unwrap();
        println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
    }
}
