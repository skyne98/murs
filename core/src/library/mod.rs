pub mod graph;
pub mod link;

use std::path::PathBuf;

use self::link::{LibraryLink, LibraryLinkGit, LibraryLinkLocal};

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    modules: Vec<PathBuf>,
    links: LibraryLinks,
}
impl Library {
    pub fn modules(&self) -> &Vec<PathBuf> {
        &self.modules
    }
    pub fn links(&self) -> Vec<LibraryLink> {
        let links = self.links.clone();
        let git = links.git.map_or(vec![], |git| {
            git.iter()
                .map(|link| LibraryLink::Git(link.clone()))
                .collect()
        });
        let local = links.local.map_or(vec![], |local| {
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
