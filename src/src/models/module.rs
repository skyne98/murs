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
    /// Version of the module to require.
    /// This will search through all the tags formatted like `<module-name>` (like `hello-world`).
    pub version: VersionReq,
}
