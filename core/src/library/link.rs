use std::str::FromStr;

use crate::utils::hash::hash_str;
use anyhow::{anyhow, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LibraryLink {
    Git(LibraryLinkGit),
    Local(LibraryLinkLocal),
}
impl LibraryLink {
    pub fn hash(&self) -> String {
        hash_str(self.to_string()).to_hex().to_string()
    }
    pub fn to_string(&self) -> String {
        match self {
            LibraryLink::Git(link) => format!("git+{}", link.url),
            LibraryLink::Local(link) => format!("local+{}", link.path),
        }
    }
}
impl FromStr for LibraryLink {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        if s.starts_with("git+") {
            let url = s[4..].to_string();
            Ok(LibraryLink::Git(LibraryLinkGit {
                url,
                branch: None,
                commit: None,
            }))
        } else if s.starts_with("local+") {
            let path = s[6..].to_string();
            Ok(LibraryLink::Local(LibraryLinkLocal { path }))
        } else {
            Err(anyhow!("Invalid library link: {}", s))
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryLinkGit {
    pub url: String,

    // main/master HEAD is missing
    pub branch: Option<String>,
    pub commit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryLinkLocal {
    pub path: String,
}
