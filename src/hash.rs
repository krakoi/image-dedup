use img_hash::{HasherConfig, Hasher, ImageHash};
use std::path::Path;
use std::fs::read_dir;
use std::io::Result;

type HashSize = [u8; 8];

pub struct HashOf {
  hash: ImageHash<HashSize>,
  file: String
}

fn hash_file(file: &Path, hasher: &Hasher<HashSize>, hashes: &mut Vec<HashOf>) {
  match image::open(file) {
    Ok(img) => {
      let hash = hasher.hash_image(&img);
      println!("{:?} -> {}", file, hash.to_base64());
      hashes.push(HashOf {
        hash,
        file: String::from(file.to_string_lossy())
      });
    },
    Err(e) => eprintln!("Error decoding image {:?}: {:?}", file, e),
  }
}

fn hash_helper(path: &Path, hasher: &Hasher<HashSize>, hashes: &mut Vec<HashOf>) -> Result<()> {
  if path.is_dir() {
    for entry in read_dir(path)? {
      hash_helper(&entry?.path(), hasher, hashes)?;
    }
  } else {
    hash_file(path, hasher, hashes);
  }

  return Ok(());
}

pub fn hash(dir: &Path) -> Result<Vec<HashOf>> {
  let hasher = HasherConfig::with_bytes_type::<HashSize>().to_hasher();
  let mut hashes = Vec::<HashOf>::new();

  hash_helper(dir, &hasher, &mut hashes)?;

  return Ok(hashes);
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
