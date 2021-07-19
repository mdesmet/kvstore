use clap::{App, Arg, SubCommand};
use std::path::Path;
use std::process::exit;
use kvs::Result;
use kvs::KvStore;

fn main() -> Result<()> {
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
                .about("remove value from key value store")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .get_matches();

    // We can find out whether or not debugging was turned on
    if matches.is_present("version") {
        println!("Installed version: {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let mut store = KvStore::open(Path::new("."))?;

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let value = store.get(key.to_owned())?;
            match value {
                Some(value) => {
                    println!("{}", value);
                    exit(0);
                },
                None => {
                    println!("Key not found");
                    exit(0);
                }
            }
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            let value = matches.value_of("value").unwrap();
            store.set(key.to_owned(), value.to_owned())?;
            exit(0);
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").unwrap();
            match store.remove(key.to_owned()) {
                Ok(_) => exit(0),
                Err(_) => {
                    println!("Key not found");
                    exit(-1);
                }
            }
            
        }
        _ => unreachable!(),
    }
}
