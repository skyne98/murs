pub mod graph;
pub mod link;
pub mod model;

use std::path::{Path, PathBuf};

use self::link::{LibraryLink, LibraryLinkGit, LibraryLinkLocal};
use crate::utils::fs::read_file_str;
use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    modules: Vec<PathBuf>,
    links: Option<LibraryLinks>,
}
impl Library {
    pub async fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let contents = read_file_str(path)
            .await
            .context(format!("Cannot read library.toml at {:?}", path))?;
        let library = toml::from_str(&contents)
            .context("Invalid syntax in library.toml, make sure to follow https://toml.io/en/")?;
        Ok(library)
    }
    pub fn modules(&self) -> &Vec<PathBuf> {
        &self.modules
    }
    pub fn links(&self) -> Vec<LibraryLink> {
        let links = self.links.clone();
        let git = links
            .as_ref()
            .map(|l| l.git.clone())
            .flatten()
            .map_or(vec![], |git| {
                git.iter()
                    .map(|link| LibraryLink::Git(link.clone()))
                    .collect()
            });
        let local = links.map(|l| l.local).flatten().map_or(vec![], |local| {
            local
                .iter()
                .map(|link| LibraryLink::Local(link.clone()))
                .collect()
        });

        git.into_iter().chain(local).collect()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryLinks {
    pub git: Option<Vec<LibraryLinkGit>>,
    pub local: Option<Vec<LibraryLinkLocal>>,
}
