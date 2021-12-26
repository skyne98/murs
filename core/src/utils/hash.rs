use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use blake3::Hash;

use super::fs::read_file_bytes;

pub fn hash_str<S: AsRef<str>>(str: S) -> Hash {
    blake3::hash(str.as_ref().as_bytes())
}

pub fn hash_bytes(bytes: &[u8]) -> Hash {
    blake3::hash(bytes)
}

#[async_recursion]
pub async fn hash<U: AsRef<Path> + std::marker::Send>(
    path: U,
    contents_only: bool,
) -> Result<Hash> {
    let path = path.as_ref();
    let hash = if path.is_dir() {
        // Collect the hashes
        let mut hashes: Vec<Hash> = vec![];
        let mut read_dir = tokio::fs::read_dir(path).await?;
        // Collect directory entries
        let mut entries = vec![];
        loop {
            let entry = read_dir
                .next_entry()
                .await
                .context("Problem reading a directory entry")?;
            if let None = entry {
                break;
            }
            let entry = entry.context("Directory entry is missing")?;
            entries.push(entry);
        }
        entries.sort_by(|a, b| a.path().cmp(&b.path()));
        // Dig into each of them
        for entry in entries.iter() {
            let path = entry.path();
            if path.is_dir() {
                hashes.push(hash(path, contents_only).await?);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        // File found
                        let filename = PathBuf::from(filename);
                        let file_name = filename
                            .file_name()
                            .context("Cannot get name of the file")?
                            .to_str()
                            .context("Cannot convert OsStr")?;
                        let contents = read_file_bytes(path).await?;
                        let name_hash = hash_bytes(file_name.as_bytes());
                        let name_hash_bytes: &[u8] = if contents_only {
                            &[]
                        } else {
                            name_hash.as_bytes()
                        };
                        let contents_hash = hash_bytes(&contents);
                        let hash_combo = vec![name_hash_bytes, contents_hash.as_bytes()].concat();
                        let hash_combo_hash = hash_bytes(&hash_combo);
                        hashes.push(hash_combo_hash);
                    }
                    None => {
                        // println!("failed: {:?}", path);
                    }
                }
            }
        }
        // Make a general hash
        let hash_combo: Vec<u8> = hashes
            .iter()
            .map(|hash| hash.as_bytes())
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .map(|byte| *byte)
            .collect();
        hash_bytes(&hash_combo)
    } else {
        let file_name = path
            .file_name()
            .context("Cannot get name of the file")?
            .to_str()
            .context("Cannot convert OsStr")?;
        let contents = read_file_bytes(path).await?;
        let name_hash = hash_bytes(file_name.as_bytes());
        let name_hash_bytes: &[u8] = if contents_only {
            &[]
        } else {
            name_hash.as_bytes()
        };
        let contents_hash = hash_bytes(&contents);
        let hash_combo = vec![name_hash_bytes, contents_hash.as_bytes()].concat();
        let hash_combo_hash = hash_bytes(&hash_combo);
        hash_combo_hash
    };
    Ok(hash)
}
