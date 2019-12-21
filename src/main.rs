extern crate image;
extern crate img_hash;
extern crate glob;
extern crate rayon;

mod hash;

use std::env::{args, current_dir};
use std::io::{Error, ErrorKind};
use glob::glob;
use self::hash::{calculate_hashes, calculate_hashes_paralell, same_pairs};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();

    let mut dir = current_dir()?;
    dir.push(&args[1]);

    if !dir.is_dir() {
        return Err(Error::new(ErrorKind::Other, "The given path is not a directory!"));
    }

    let images = glob(&(dir.to_string_lossy() + "/**/*.jpg"))
        .expect("Failed to read glob pattern");
    
    // let hashes = calculate_hashes(images);
    let hashes = calculate_hashes_paralell(images);
    let pairs = same_pairs(&hashes, 5);

    for (a, b) in pairs {
        println!("{}\t{}", a, b);
    }

    return Ok(());
}
