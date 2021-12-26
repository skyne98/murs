use semver::VersionReq;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleDependency {
    /// Version of the module to require.
    /// This will search through all the tags formatted like `<module-name>` (like `hello-world`).
    pub version: VersionReq,
}
