extern crate common;
use common::foo;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    foo::foo();

    println!("Hello, world!");
}