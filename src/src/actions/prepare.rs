use std::path::{Path, PathBuf};

use anyhow::Result;
use tokio::fs::create_dir;

use super::parse::{parse, ParsedModule};

#[derive(Debug)]
pub struct Workspace {
    pub module: ParsedModule,
    pub work_dir: PathBuf,
    pub git_dir: PathBuf,
}

pub async fn prepare(path: impl AsRef<Path>) -> Result<Workspace> {
    let module = parse(&path).await?;
    let work_dir = module.path.join("work");
    if work_dir.exists() == false {
        create_dir(&work_dir).await?;
    }

    let git_dir = work_dir.join("git");
    if git_dir.exists() == false {
        create_dir(&git_dir).await?;
    }

    Ok(Workspace {
        module,
        work_dir,
        git_dir,
    })
}
