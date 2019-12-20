extern crate image;
extern crate img_hash;

mod hash;

use std::env::{args, current_dir};
use std::io::{Error, ErrorKind};
use self::hash::{hash, same_pairs};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();

    let mut dir = current_dir()?;
    dir.push(&args[1]);

    if !dir.is_dir() {
        return Err(Error::new(ErrorKind::Other, "The given path is not a directory!"));
    }

    let hashes = hash(&dir)?;
    let pairs = same_pairs(&hashes, 5);

    for (a, b) in pairs {
        println!("{}\t{}", a, b);
    }

    return Ok(());
}
