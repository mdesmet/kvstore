use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Super cool key value store")
        .arg(
            Arg::with_name("version")
                .short("V")
                .long("version")
                .help("Version info"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve value from key value store")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("retrieve value from key value store")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .requires("value")
                        .required(true),
                )
                .arg(Arg::with_name("value").index(2)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("retrieve value from key value store")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .get_matches();

    // We can find out whether or not debugging was turned on
    if matches.is_present("version") {
        println!("Installed version: {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    match matches.subcommand() {
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1)
        }
        ("rm", Some(_matches)) => {
            eprintln!("unimplemented");
            std::process::exit(1);
        }
        _ => unreachable!(),
    }
}
