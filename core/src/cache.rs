use std::path::PathBuf;

use anyhow::Result;
use colored::*;
use directories::UserDirs;
use git2::Repository;
use log::info;

use crate::{
    library::link::LibraryLinkGit,
    utils::{
        fs::{ensure_dir_exists, remove_dir},
        git::do_pull,
        hash::hash_str,
    },
};

pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn dir() -> PathBuf {
        let user_dirs = UserDirs::new().expect("Cannot find a $HOME directory");
        let home = user_dirs.home_dir();
        home.join(".murs")
    }
    pub async fn new() -> Result<Self> {
        let dir = Self::dir();
        ensure_dir_exists(&dir).await?;
        Ok(Cache { dir: dir })
    }
    pub async fn clean(&self) -> Result<()> {
        let dir = self.dir.clone();
        remove_dir(&dir).await?;
        Ok(())
    }
    pub async fn git_path(&self) -> Result<PathBuf> {
        let path = self.dir.join("git");
        ensure_dir_exists(&path).await?;
        Ok(path)
    }
    pub async fn git(&self, link: &LibraryLinkGit) -> Result<PathBuf> {
        let repository = &link.url;
        let branch = link.branch.clone().unwrap_or_else(|| "main".to_string());

        // Hash the repo name
        let repository_hash = hash_str(repository).to_hex().to_string();
        let repository_path = self.git_path().await?.join(repository_hash);

        if repository_path.exists() {
            // Pull the repository
            {
                info!("Pulling repository {}...", repository.yellow());
                let repository = Repository::open(&repository_path)?;
                let remote_name = "origin";
                let mut remote = repository.find_remote(&remote_name)?;
                do_pull(&repository, &branch, &mut remote)?;
            }

            let repository = Repository::open(&repository_path)?;
            repository
        } else {
            // Clone the repository
            info!("Cloning repository {}...", repository.yellow());
            let url = repository;
            let repository = Repository::clone(url, &repository_path)?;
            repository
        };

        Ok(repository_path)
    }
}
