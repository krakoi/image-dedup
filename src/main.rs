extern crate image;
extern crate img_hash;
extern crate glob;

mod hash;

use std::env::{args, current_dir};
use std::io::{Error, ErrorKind};
use glob::glob;
use self::hash::{calculate_hashes, same_pairs};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();

    let mut dir = current_dir()?;
    dir.push(&args[1]);

    if !dir.is_dir() {
        return Err(Error::new(ErrorKind::Other, "The given path is not a directory!"));
    }

    let images = glob(&(dir.to_string_lossy() + "/**/*.jpg"))
        .expect("Failed to read glob pattern")
        .filter_map(|f|  match f {
            Ok(file) => Some(file),
            Err(_) => None
        });

    // let images = img_collect(&dir)?;
    let hashes = calculate_hashes(images);
    let pairs = same_pairs(&hashes, 5);

    for (a, b) in pairs {
        println!("{}\t{}", a, b);
    }

    return Ok(());
}
