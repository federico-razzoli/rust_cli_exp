extern crate common;
use common::scanner::long_range_scanner;
use common::stylesheet::Stylesheet;
use common::stylesheet::StyleProperties;
use common::stylesheet::StyleTransformation;
use common::stylesheet::StyleColor;


const MAX_PLEASE: u64 = 3;


// NOTE:
// using this crate with default features enabled
// and doing nearly nothing with it
// makes a binary more than 600K bigger.
extern crate clap;
use clap::{Arg, ArgMatches, App};


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
    let options: ArgMatches = App::new(APP_INFO.name)
        .about(APP_INFO.description)
        .version(APP_INFO.version)
        .arg(
            Arg::with_name("please")
            .short("p")
            .long("please")
            .multiple(true)
            .help("No practical effect, but it's good to be kind. Specify multiple times to implore properly.")
            .takes_value(false)
        )
        .get_matches();

    let mut sheet: Stylesheet = Stylesheet::new();
    sheet.add_style("danger", StyleProperties {
        transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: None
    });
    sheet.add_style("info", StyleProperties {
        transformation: [].to_vec(), color: Some(StyleColor::Green), background: None
    });
    sheet.add_style("complain", StyleProperties {
        transformation: [].to_vec(), color: Some(StyleColor::Yellow), background: None
    });

    let please_count: u64 = options.occurrences_of("please");
    if please_count > MAX_PLEASE {
        let message: String = format!("{}{}{}", "You said please ", &please_count, " times... please stop!");
        sheet.println("complain", message);
    }

    long_range_scanner::scan().print(&sheet);
}
