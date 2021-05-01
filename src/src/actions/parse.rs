use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use tokio::fs::{canonicalize, read_to_string};

use crate::models::Module;

#[derive(Debug)]
pub struct ParsedModule {
    pub path: PathBuf,
    pub module: Module,
}

pub async fn parse(path: impl AsRef<Path>) -> Result<ParsedModule> {
    let path = path.as_ref();

    if path.is_dir() == false {
        return Err(anyhow!("Path provided for parsing is not a directory"));
    }

    let mut current_path = path.to_path_buf();
    let mut module: Option<Module> = None;
    loop {
        let module_toml_path = current_path.join("module.toml");
        if module_toml_path.exists() {
            // We found the module, parse it
            let module_str = read_to_string(module_toml_path).await?;
            module = Some(toml::from_str::<Module>(&module_str)?);
            break;
        } else {
            // Look further up
            let parent_path = current_path.parent();
            if let Some(parent_path) = parent_path {
                current_path = parent_path.to_path_buf();
            } else {
                // There is no further parent upwards
                break;
            }
        }
    }

    if let Some(module) = module {
        let parsed_module = ParsedModule {
            path: canonicalize(current_path.clone()).await?,
            module,
        };
        Ok(parsed_module)
    } else {
        Err(anyhow!("Couldn't find the module.toml file"))
    }
}
