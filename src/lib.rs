extern crate log;
extern crate image;
extern crate img_hash;
extern crate globset;
extern crate walkdir;
extern crate rayon;

use log::{debug,error};
use std::path::{Path,PathBuf};
use img_hash::{HasherConfig, Hasher, ImageHash};
use walkdir::{WalkDir,DirEntry};
use globset::{GlobMatcher, GlobBuilder};
use rayon::prelude::*;

type HashSize = [u8; 8];

#[macro_export]
macro_rules! DEFAULT_FILTER {
  () => ( "*.{jpg,jpeg,png,gif,webp,bmp,tiff}" )
}

pub struct HashOf {
  hash: ImageHash<HashSize>,
  file: String
}

fn filter_glob_errors<E: std::fmt::Debug>(file: Result<DirEntry,E>) -> Option<PathBuf> {
  match file {
    Ok(file) => {
      if file.file_type().is_dir() {
        None
      } else {
        Some(file.into_path())
      }
    }
    Err(e) => {
      error!("Error during file matching: {:?}", e);
      None
    }
  }
}

fn hash_image(hasher: &mut Hasher<HashSize>, file: PathBuf) -> Option<HashOf> {
  match image::open(&file) {
    Ok(img) => {
      let hash = hasher.hash_image(&img);
      debug!("{:?} -> {}", file, hash.to_base64());

      Some(HashOf {
        hash,
        file: String::from(file.to_string_lossy())
      })
    },
    Err(e) => {
      error!("Error decoding image {:?}: {:?}", file, e);
      None
    }
  }
}

pub fn calculate_hashes_sequential(tree: WalkDir, matcher: GlobMatcher) -> Vec<HashOf> {
  let mut hasher = HasherConfig::with_bytes_type::<HashSize>().to_hasher();

  return tree.into_iter()
    .filter_map(filter_glob_errors)
    .filter(|file| matcher.is_match(file))
    .filter_map(|file| hash_image(&mut hasher, file))
  .collect();
}

pub fn calculate_hashes_paralell(tree: WalkDir, matcher: GlobMatcher) -> Vec<HashOf> {
  return tree.into_iter()
    .filter_map(filter_glob_errors)
    .filter(|file| matcher.is_match(file))
    .par_bridge()
    .map_init(|| HasherConfig::with_bytes_type::<HashSize>().to_hasher(), hash_image)
    .filter_map(|hash| hash)
  .collect();
}

pub fn calculate_hashes(root: &Path, paralell: bool, custom_filter: Option<&str>) -> Vec<HashOf> {
  let images = WalkDir::new(root);

  let glob = GlobBuilder::new(custom_filter.unwrap_or(DEFAULT_FILTER!()))
      .case_insensitive(true)
      .build().expect("Failed to parse glob pattern")
  .compile_matcher();

  if paralell {
    return calculate_hashes_paralell(images, glob)
  } else {
    return calculate_hashes_sequential(images, glob);
  }
}

pub fn same_pairs(vec: &Vec<HashOf>, threshold: u32) -> Vec<(&str,&str)> {
  let mut pairs = Vec::new();

  for (i, a) in vec.iter().enumerate() {
    for b in vec.iter().skip(i + 1) {
      let dist = a.hash.dist(&b.hash);

      if dist <= threshold {
        pairs.push((a.file.as_str(), b.file.as_str()));
      }
    }
  }

  return pairs;
}
