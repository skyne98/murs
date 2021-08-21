use std::collections::HashMap;

use semver::{Version, VersionReq};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub title: String,
    pub version: Version,
    /// Defines if this module shows up as a set of rules (ruleset).
    #[serde(default)]
    pub ruleset: bool,
    pub dependencies: Option<HashMap<String, ModuleDependency>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleDependency {
    pub volume: Option<VolumeReference>,
    /// Version of the module to require.
    /// This will search through all the tags formatted like `<module-name>-<version>` (like `hello-world-1.0.0`).
    pub version: VersionReq,
}

pub enum VolumeReferenceType {
    Directory,
    Git,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VolumeReference {
    /// Path to a directory containing a volume on the local file system.
    pub path: Option<String>,
}

impl VolumeReference {
    pub fn reference_type(&self) -> Option<VolumeReferenceType> {
        if let Some(_) = self.path {
            return Some(VolumeReferenceType::Directory);
        }

        None
    }
}
