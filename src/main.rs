extern crate clap;

use std::fs;
use clap::{Arg, App, SubCommand, ArgMatches};

fn is_positive_num<T: std::str::FromStr>(v: String) -> Result<(), String> {
    match v.parse::<T>() {
        Ok(_) => Ok(()),
        Err(_) => Err("Must be a number".into()),
    }
}

fn count_subdirs( args: &ArgMatches ) {
    let inputs: Vec<&str> = args.values_of("input").unwrap().collect();
    for i in inputs {
        println!("Got input: {}", i);
    }
}

fn main() {
    let matches = App::new("File tree counter")
        .version("1.0")
        .author("Jon Davis (khalen@gmail.com)")
        .about("Recursively walk directories on a filesystem, performing useful operations.")
        .subcommand(SubCommand::with_name("count")
                    .about("Count files in the given directories")
                    .arg(Arg::with_name("min")
                         .short("m")
                         .long("minfiles")
                        .validator(is_positive_num::<u32>)
                         .help("Don't display subdirectories with fewer than this many files below them"))
                    .arg(Arg::with_name("max")
                         .short("x")
                         .long("maxfiles")
                         .validator(is_positive_num::<u32>)
                         .help("Don't display subdirectories with more than this many files below them"))
                    .arg(Arg::with_name("dirs")
                         .short("d")
                         .long("count_dirs")
                         .help("Include directories in counts"))
                    .arg(Arg::with_name("input")
                         .default_value(".")))
        .get_matches();

    if let Some(count_args) = matches.subcommand_matches("count") {
        count_subdirs(count_args);
    }
}
