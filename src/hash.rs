use std::path::PathBuf;
use img_hash::{HasherConfig, Hasher, ImageHash};

type HashSize = [u8; 8];

pub struct HashOf {
  hash: ImageHash<HashSize>,
  file: String
}

pub fn calculate_hashes(files: impl Iterator<Item = PathBuf>) -> Vec<HashOf> {
  let hasher = HasherConfig::with_bytes_type::<HashSize>().to_hasher();

  return files.filter_map(|file| {
      return match image::open(&file) {
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
      };
    })
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
