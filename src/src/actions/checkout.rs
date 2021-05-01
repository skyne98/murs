use std::path::PathBuf;

use crate::models::{Module, ModuleDependency};
use anyhow::{Context, Result};
use git2::Repository;
use seahash::hash;

use super::prepare::Workspace;

pub struct GitRepository {
    pub repository: Repository,
    pub path: PathBuf,
}

pub async fn checkout_dependency(
    workspace: &Workspace,
    dep: &ModuleDependency,
) -> Result<GitRepository> {
    let git_hash = hash(
        dep.git
            .as_ref()
            .context("Reading the \"git\" field of the dependency")?
            .as_bytes(),
    );
    let git_hash_bytes = git_hash.to_be_bytes();
    let git_hash_str = base64::encode(git_hash_bytes);
    let repo_path = workspace.git_dir.join(git_hash_str);

    // Check out if necessary
    let repo = if repo_path.exists() == false {
        let url = dep
            .git
            .as_ref()
            .context("Reading the \"git\" field of the dependency")?;
        Repository::clone(url, &repo_path)?
    } else {
        Repository::open(&repo_path)?
    };

    Ok(GitRepository {
        repository: repo,
        path: repo_path.to_path_buf(),
    })
}
