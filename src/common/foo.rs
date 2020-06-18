use std::collections::HashMap;

extern crate console;
use self::console::Style;


pub fn foo() {
    println!("fooo!");

    let alert = Style::new().red().bold();
    println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
}
