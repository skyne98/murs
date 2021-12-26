use std::path::Path;

use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn read_file_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();
    let mut file = tokio::fs::File::open(path).await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;
    Ok(contents)
}
pub async fn read_file_str<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytes = read_file_bytes(path).await?;
    Ok(String::from_utf8(bytes)?)
}
pub async fn path_exists<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    Ok(tokio::fs::metadata(path).await.is_ok())
}
pub async fn path_is_dir<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    Ok(tokio::fs::metadata(path).await?.is_dir())
}
pub async fn path_is_file<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    Ok(tokio::fs::metadata(path).await?.is_file())
}
pub async fn ensure_dir_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path_exists(path).await? {
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}
pub async fn ensure_file_exists<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if !path_exists(path).await? {
        tokio::fs::File::create(path).await?;
    }
    Ok(())
}
pub async fn write_file_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> Result<()> {
    let path = path.as_ref();
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(bytes).await?;
    Ok(())
}
pub async fn write_file_str<P: AsRef<Path>>(path: P, str: &str) -> Result<()> {
    let bytes = str.as_bytes();
    write_file_bytes(path, bytes).await
}
pub async fn remove_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    tokio::fs::remove_dir_all(path).await?;
    Ok(())
}
pub async fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    tokio::fs::remove_file(path).await?;
    Ok(())
}
pub async fn remove_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    if path_is_dir(path).await? {
        remove_dir(path).await?;
    } else {
        remove_file(path).await?;
    }
    Ok(())
}
pub async fn clear_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    let mut entries = tokio::fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        remove_path(path).await?;
    }
    Ok(())
}
