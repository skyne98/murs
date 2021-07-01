use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tokio::fs::{remove_dir_all, remove_file};
use tokio_stream::StreamExt;

use crate::models::{module::Module, unit::Unit};

pub async fn read_package_manifest<P: AsRef<Path>>(root: P) -> Result<(PathBuf, Module)> {
    let root = root.as_ref();
    let manifest_path = root.join("module.toml");
    let contents = tokio::fs::read_to_string(&manifest_path).await?;
    let pkg: Module = toml::from_str(&contents)?;

    Ok((manifest_path, pkg))
}

pub async fn read_units<P: AsRef<Path>>(root: P) -> Result<Vec<Unit>> {
    let root = root.as_ref();
    let read_dir = tokio::fs::read_dir(root).await?;
    let mut read_dir_stream = tokio_stream::wrappers::ReadDirStream::new(read_dir);
    let mut files = vec![];
    while let Some(entry) = read_dir_stream.next().await {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    let unit_str = std::fs::read_to_string(entry.path())?;
                    let name = entry
                        .path()
                        .file_stem()
                        .context("Cannot get file name")?
                        .to_str()
                        .context("Cannot convert OsStr to &str")?
                        .to_string();
                    let unit = Unit::from_str(name, unit_str).await?;
                    files.push(unit);
                }
            }
        }
    }

    Ok(files)
}

pub async fn clear_dir<P: AsRef<Path>>(dir: P) -> Result<()> {
    let root = dir.as_ref();
    let read_dir = tokio::fs::read_dir(root).await?;
    let mut read_dir_stream = tokio_stream::wrappers::ReadDirStream::new(read_dir);
    while let Some(entry) = read_dir_stream.next().await {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    remove_file(entry.path()).await?;
                } else {
                    remove_dir_all(entry.path()).await?;
                }
            }
        }
    }

    Ok(())
}
