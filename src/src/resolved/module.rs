use std::collections::HashMap;

use semver::Version;
use serde_derive::{Deserialize, Serialize};

use super::Unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub version: Version,
    #[serde(default)]
    pub ruleset: bool,
    pub dependencies: HashMap<String, Module>,
    pub units: Vec<Unit>,
}
