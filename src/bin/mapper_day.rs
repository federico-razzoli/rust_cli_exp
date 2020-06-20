use std::collections::HashMap;

extern crate rand;
use rand::Rng;

extern crate console;
use console::Style;

extern crate common;
use common::scanner::long_range_scanner;
use common::stylesheet::stylesheet;
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

#[derive(Debug)]
struct Event {
    level: &'static str,
    message: &'static str,
}

impl Event {
    pub fn print(&self, sheet: &HashMap<&str, Style>) {
        stylesheet::println(self.message, &sheet, self.level);
    }
}

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

    let mut sheet = stylesheet::new();
    stylesheet::add_style(&mut sheet, "danger", StyleProperties {
        transformation: [StyleTransformation::Bold, StyleTransformation::Blink].to_vec(), color: Some(StyleColor::Red), background: None
    });
    stylesheet::add_style(&mut sheet, "info", StyleProperties {
        transformation: [].to_vec(), color: Some(StyleColor::Green), background: None
    });
    stylesheet::add_style(&mut sheet, "complain", StyleProperties {
        transformation: [].to_vec(), color: Some(StyleColor::Yellow), background: None
    });

    let please_count: u64 = options.occurrences_of("please");
    if please_count > MAX_PLEASE {
        let message: String = format!("{}{}{}", "You said please ", &please_count, " times... please stop!");
        stylesheet::println(message, &sheet, "complain");
    }

    long_range_scanner::scan();

    // Detect a random event
    let possible_events: [Event; 4] = [
        Event { level: "info", message: "Whormhole detected" },
        Event { level: "info", message: "Vulcan ship deteced" },
        Event { level: "danger", message: "Romulan ship approaching!" },
        Event { level: "danger", message: "Borg cube approaching!" },
    ];
    let rnd = rand::thread_rng().gen_range(0, possible_events.len());
    let event: &Event = possible_events.get(rnd).unwrap();
    event.print(&sheet);
}
