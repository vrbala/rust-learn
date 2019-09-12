extern crate clap;

use clap::{App, Arg, SubCommand};
use std::process;

use kvs::KvStore;

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
        let mut kvs = KvStore::new();
        kvs.set(String::from(key), String::from(value))
            .expect("Unable to set key value pair.");
        process::exit(0);
    } else if let Some(rm) = matches.subcommand_matches("rm") {
        let key = rm.value_of("key").unwrap();
        let mut kvs = KvStore::new();
        kvs.remove(String::from(key))
            .expect("Unable to remove key.");
        process::exit(0);
    } else if let Some(get) = matches.subcommand_matches("get") {
        let key = get.value_of("key").unwrap();
        let kvs = KvStore::new();
        match kvs.get(String::from(key)) {
            Ok(Some(value)) => {
                println!("{}", value);
                process::exit(0);
            }
            Ok(None) => process::exit(1),
            Err(_) => process::exit(2),
        }
    }
}
