use std::path::PathBuf;
use img_hash::{HasherConfig, Hasher, ImageHash};
use glob::Paths;
use rayon::prelude::*;

type HashSize = [u8; 8];

pub struct HashOf {
  hash: ImageHash<HashSize>,
  file: String
}

fn filter_glob_errors<E: std::fmt::Debug>(file: Result<PathBuf,E>) -> Option<PathBuf> {
  match file {
    Ok(file) => Some(file),
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

pub fn calculate_hashes(files: Paths) -> Vec<HashOf> {
  let mut hasher = HasherConfig::with_bytes_type::<HashSize>().to_hasher();

  return files
    .filter_map(filter_glob_errors)
    .filter_map(|file| hash_image(&mut hasher, file))
  .collect();
}

pub fn calculate_hashes_paralell(files: Paths) -> Vec<HashOf> {
  return files
    .filter_map(filter_glob_errors)
    .par_bridge()
    .map_init(|| HasherConfig::with_bytes_type::<HashSize>().to_hasher(), hash_image)
    .filter_map(|file| file)
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
