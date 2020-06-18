extern crate common;
use common::scanner::long_range_scanner;
use common::stylesheet::stylesheet;
use common::stylesheet::StyleProperties;
use common::stylesheet::StyleTransformation;
use common::stylesheet::StyleColor;

use std::option::Option;


// NOTE:
// using this crate with default features enabled
// and doing nearly nothing with it
// makes a binary more than 600K bigger.
extern crate clap;
use clap::{Arg, App, SubCommand};


struct AppInfo<'a> {
    name: &'a str,
    version: &'a str,
    description: &'a str,
}

const APP_INFO: AppInfo<'static> = AppInfo {
    name: "cli_exp",
    version: "0.1.0",
    description: "Research developing CLI tools in Rust",
};


fn main() {
    let options = App::new(APP_INFO.name)
        .about(APP_INFO.description)
        .version(APP_INFO.version)
        .arg(
            Arg::with_name("please")
            .short("p")
            .long("please")
            .help("No practical effect, but it's good to be kind")
            .takes_value(false)
        )
        .get_matches();

    long_range_scanner::scan();

    let mut sheet = stylesheet::new();
    stylesheet::add_style(&mut sheet, "default", StyleProperties {
        transformation: [].to_vec(), color: None, background: None
    });
    stylesheet::add_style(&mut sheet, "danger", StyleProperties {
        transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: Some(StyleColor::White)
    });

    //println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
    let alert = stylesheet::get_style(sheet, "danger");
    println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
}