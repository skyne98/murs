use anyhow::Result;
use std::path::Path;
use tokio::fs::remove_dir_all;

use super::prepare::prepare;

pub async fn clean(path: impl AsRef<Path>) -> Result<()> {
    let workspace = prepare(path).await?;
    remove_dir_all(workspace.work_dir).await?;
    Ok(())
}
