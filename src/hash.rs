use std::path::PathBuf;
use img_hash::{HasherConfig, Hasher, ImageHash};
use walkdir::{WalkDir,DirEntry};
use globset::GlobMatcher;
use rayon::prelude::*;

type HashSize = [u8; 8];

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
      eprintln!("Error during file matching: {:?}", e);
      None
    }
  }
}

fn hash_image(hasher: &mut Hasher<HashSize>, file: PathBuf) -> Option<HashOf> {
  match image::open(&file) {
    Ok(img) => {
      let hash = hasher.hash_image(&img);
      println!("{:?} -> {}", file, hash.to_base64());

      Some(HashOf {
        hash,
        file: String::from(file.to_string_lossy())
      })
    },
    Err(e) => {
      eprintln!("Error decoding image {:?}: {:?}", file, e);
      None
    }
  }
}

pub fn calculate_hashes(tree: WalkDir, matcher: GlobMatcher) -> Vec<HashOf> {
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

pub fn same_pairs(vec: &Vec<HashOf>, threshold: u32) -> Vec<(&String,&String)> {
  let mut pairs = Vec::new();

  for (i, a) in vec.iter().enumerate() {
    for b in vec.iter().skip(i + 1) {
      let dist = a.hash.dist(&b.hash);

      if dist <= threshold {
        pairs.push((&a.file, &b.file));
      }
    }
  }

  return pairs;
}
