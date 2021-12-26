use crate::utils::hash::hash_str;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryLinkGit {
    url: String,

    // main/master HEAD is missing
    branch: Option<String>,
    commit: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LibraryLinkLocal {
    path: String,
}
