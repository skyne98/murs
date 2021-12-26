use anyhow::{Context, Result};
use semver::Version;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::utils::fs::read_file_str;

use self::dependency::ModuleDependency;

pub mod dependency;
pub mod ruleset;

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    /// Name used inside the system
    pub name: String,
    /// Human-readable version of the module name
    pub title: String,
    pub version: Version,
    /// Defines if this module shows up as a set of rules (ruleset).
    #[serde(default)]
    pub ruleset: bool,
    pub dependencies: Option<HashMap<String, ModuleDependency>>,
}

impl Module {
    pub async fn from_path<P: AsRef<Path>>(root: P) -> Result<(PathBuf, Module)> {
        let root = root.as_ref();
        let manifest_path = root.join("module.toml");
        let contents = read_file_str(&manifest_path)
            .await
            .context(format!("Cannot find module.toml at {:?}", manifest_path))?;
        let pkg: Module = toml::from_str(&contents)
            .context("Invalid syntax in module.toml, make sure to follow https://toml.io/en/")?;

        Ok((manifest_path, pkg))
    }
}
