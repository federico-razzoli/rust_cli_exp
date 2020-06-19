extern crate common;
use common::scanner::long_range_scanner;
use common::stylesheet::stylesheet;
use common::stylesheet::StyleProperties;
use common::stylesheet::StyleTransformation;
use common::stylesheet::StyleColor;


// NOTE:
// using this crate with default features enabled
// and doing nearly nothing with it
// makes a binary more than 600K bigger.
extern crate clap;
use clap::{Arg, App};


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
    App::new(APP_INFO.name)
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
    stylesheet::add_style(&mut sheet, "danger", StyleProperties {
        transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: Some(StyleColor::White)
    });

    stylesheet::println("ALERT: Romulan ship approaching!", sheet, "danger");
}
