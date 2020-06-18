extern crate common;
use common::scanner::long_range_scanner;
use common::stylesheet::stylesheet;

extern crate console;
use console::Style;


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

    let alert = long_range_scanner::scan();

    let mut sheet = stylesheet::new();
    stylesheet::add_style(&mut sheet, "default", Style::new());
    stylesheet::add_style(&mut sheet, "danger", Style::new().red().bold());

    //println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
    let alert = stylesheet::get_style(sheet, "danger");
    println!("{}", alert.apply_to("ALERT: Romulan ship approaching!"));
}