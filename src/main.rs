extern crate log;
extern crate stderrlog;
extern crate clap;
extern crate globset;
extern crate walkdir;
extern crate image_dedup;

use stderrlog::{Timestamp};
use std::path::Path;
use image_dedup::{calculate_hashes, same_pairs, DEFAULT_FILTER};
use clap::{clap_app, crate_version, crate_authors, crate_name, crate_description};

static FILTER_HELP: &str = concat!("Filename pattern filter. Default: ", DEFAULT_FILTER!());

fn main() -> Result<(), std::io::Error> {
    let matches = clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg filter: -e --filter FILTER_HELP)
        (@arg sequential: -s --seq "Turn off paralell execution - execution will be slower but somewhat less IO-intensive")
        (@arg verbose: -v --verbose "Print file hashes")
        (@arg DIR: +required "Sets the root directory, where images will be searched")
    ).get_matches();

    let dir = Path::new(matches.value_of("DIR").unwrap());

    if !dir.is_dir() {
        panic!("The given path is not a directory!");
    }

    stderrlog::new()
        .module(module_path!())
        .quiet(!matches.is_present("verbose"))
        .verbosity(3)
        .timestamp(Timestamp::Off)
        .init()
        .unwrap();

    let hashes = calculate_hashes(dir, ! matches.is_present("sequential"), matches.value_of("filter"));
    let pairs = same_pairs(&hashes, 5);

    for (a, b) in pairs {
        println!("{}\t{}", a, b);
    }

    return Ok(());
}
