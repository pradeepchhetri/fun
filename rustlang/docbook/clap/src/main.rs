extern crate clap;

use clap::App;
use clap::Arg;
use clap::SubCommand;

fn main() {

    let matches = App::new("FirstClapApp")
                        .version("0.0.1")
                        .author("Pradeep Chhetri")
                        .about("Learn Rust")
                        .arg(Arg::with_name("config")
                                    .short("c")
                                    .long("config")
                                    .value_name("FILE")
                                    .help("Sets a custom configuration file")
                                    .takes_value(true))
                        .arg(Arg::with_name("output")
                                    .help("Sets an optional output file")
                                    .index(1))
                        .arg(Arg::with_name("debug")
                                    .short("d")
                                    .multiple(true)
                                    .help("Turn on debugging information"))
                        .subcommand(SubCommand::with_name("test")
                                                .about("Testing App")
                                                .arg(Arg::with_name("list")
                                                          .short("l")
                                                          .long("list")
                                                          .help("list test values")))
                        .get_matches();

    // Check for value passed to positional arguments or optional arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value of output file: {}", o);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value of configuration file: {}", c);
    }

    // You can check number to times a particular flag was passed
    match matches.occurrences_of("d") {
        0 => println!("Debug mode is off!"),
        1 => println!("Debug mode is on in mode 1"),
        2 => println!("Debug mode is on in mode 2"),
        3 | _ => println!("Only for development debugging, never on production"),
    }

    // You can check for subcommands existence
    if let Some(ref matches) = matches.subcommand_matches("test") {
        if matches.is_present("list") {
            println!("Print test list.");
        } else {
            println!("Don't print test list.");
        }
    }
}
