use std::collections::HashMap;

use semver::{Version, VersionReq};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub version: Version,
    /// Defines if this module shows up as a ruleset.
    #[serde(default)]
    pub ruleset: bool,
    /// Name of the module this module is supposed to replace.
    /// Used during trying to resolve some package with a specific name.
    pub replaces: Option<String>,
    /// Modules this module depends on to be able to reference them in the source.
    pub dependencies: Option<HashMap<String, ModuleDependency>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleDependency {
    /// URL of the repository to check out.
    pub git: Option<String>,
    /// Local path to the module.
    pub path: Option<String>,
    /// Version of the module to require.
    /// This will search through all the tags formatted like `<module-name>-<version>` (like `hello-world-1.0.0`).
    pub version: Option<VersionReq>,
    pub branch: Option<String>,
}
