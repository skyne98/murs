use std::collections::HashMap;

use semver::{Version, VersionReq};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub name: String,
    pub version: Version,
    pub dependencies: Option<HashMap<String, VersionReq>>,
}
