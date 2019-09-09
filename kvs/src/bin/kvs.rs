extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process;

use kvs::SetCommand;

fn main() {
    let matches = App::new("A simple distributed key-value store.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("get")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("key"),
                )
                .about("Get value associated with key."),
        )
        .subcommand(
            SubCommand::with_name("set")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("key"),
                )
                .arg(
                    Arg::with_name("value")
                        .takes_value(true)
                        .required(true)
                        .value_name("value"),
                )
                .about("Associates passed value with key."),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(
                    Arg::with_name("key")
                        .takes_value(true)
                        .required(true)
                        .value_name("key"),
                )
                .about("Removes key."),
        )
        .get_matches();

    // match matches.subcommand_name() {
    //     Some("get") => {
    //         eprintln!("unimplemented");
    //         panic!();
    //     }
    //     Some("set") => {
    //         eprintln!("unimplemented");
    //         panic!();
    //     }
    //     Some("rm") => {
    //         eprintln!("unimplemented");
    //         panic!();
    //     }
    //     None => {
    //         process::exit(1);
    //     }
    //     Some(_) => {
    //         eprintln!("Unknown command.");
    //         process::exit(2);
    //     }
    // };

    if let Some(set) = matches.subcommand_matches("set") {
        let key = set.value_of("key").unwrap();
        let value = set.value_of("value").unwrap();
        let set = SetCommand::new(String::from(key), String::from(value));
    }
}
