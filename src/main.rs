extern crate image;
extern crate img_hash;
extern crate globset;
extern crate walkdir;
extern crate rayon;

mod hash;

use std::env::{args, current_dir};
use std::io::{Error, ErrorKind};
use std::path::Path;
use walkdir::WalkDir;
use globset::GlobBuilder;
use self::hash::{calculate_hashes, calculate_hashes_paralell, same_pairs};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = args().collect();
    let dir = Path::new(&args[1]);

    if !dir.is_dir() {
        panic!("The given path is not a directory!");
    }

    let images = WalkDir::new(dir);

    let glob = GlobBuilder::new("*.{jpg,jpeg,png,gif,webp,bmp,tiff}")
        .case_insensitive(true)
        .build().expect("Failed to parse glob pattern")
    .compile_matcher();
    
    // let hashes = calculate_hashes(images);
    let hashes = calculate_hashes_paralell(images, glob);
    let pairs = same_pairs(&hashes, 5);

    for (a, b) in pairs {
        println!("{}\t{}", a, b);
    }

    return Ok(());
}
