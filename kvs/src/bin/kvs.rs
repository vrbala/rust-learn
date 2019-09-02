extern crate clap;

use std::process;
use clap::{Arg, App, SubCommand};

fn main() {

    let matches = App::new("A simple distributed key-value store.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(SubCommand::with_name("get")
                    .arg(Arg::with_name("key")
                         .takes_value(true)
                         .required(true)                         
                         .value_name("key"))
                    .about("Get value associated with key."))
        .subcommand(SubCommand::with_name("set")
                    .arg(Arg::with_name("key")
                         .takes_value(true)
                         .required(true)
                         .value_name("key"))
                    .arg(Arg::with_name("value")
                         .takes_value(true)
                         .required(true)
                         .value_name("value"))                    
                    .about("Associated passed value with key."))
        .subcommand(SubCommand::with_name("rm")
                    .arg(Arg::with_name("key")
                         .takes_value(true)
                         .required(true)                         
                         .value_name("key"))
                    .about("Removes key."))        
        .get_matches();


    match matches.subcommand_name() {
        Some("get") => {
            eprintln!("unimplemented");
            panic!();
        },
        Some("set") => {
            eprintln!("unimplemented");
            panic!();
        },
        Some("rm") => {
            eprintln!("unimplemented");
            panic!();
        },        
        None => {
            process::exit(1);
        },
        Some(_) => {
            eprintln!("Unknown command.");
            process::exit(2);
        },
    };
}

