use std::path::{Path, PathBuf};

use super::Module;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ModuleModel {
    pub path: PathBuf,
    pub module: Module,
}
impl ModuleModel {
    pub async fn from_dir<P: AsRef<Path>>(dir: P) -> Result<Self> {
        let dir = dir.as_ref();
        let module = Module::from_dir(dir).await?;

        Ok(ModuleModel {
            path: dir.to_path_buf(),
            module,
        })
    }
}
