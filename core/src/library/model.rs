use std::path::PathBuf;

use crate::{module::model::ModuleModel, utils::fs::path_is_file};

use super::{link::LibraryLink, Library, LibraryLinks};
use anyhow::Result;
use colored::*;
use log::warn;

#[derive(Debug, Clone)]
pub struct LibraryModel {
    pub link: LibraryLink,
    pub path: PathBuf,
    pub library: Library,
}
impl LibraryModel {
    pub async fn from_dir(path: PathBuf, link: LibraryLink) -> Result<Self> {
        let library_toml_path = path.join("library.toml");
        let library = if path_is_file(&library_toml_path).await? {
            Library::from_toml(&library_toml_path).await?
        } else {
            warn!(
                "No library.toml found in {}, looking for module.toml",
                format!("{:?}", path).yellow()
            );
            Library {
                modules: vec![".".into()],
                links: Some(LibraryLinks {
                    git: None,
                    local: None,
                }),
            }
        };

        Ok(LibraryModel {
            link,
            path,
            library,
        })
    }
    pub async fn from_toml(path: PathBuf, link: LibraryLink) -> Result<Self> {
        let library = Library::from_toml(&path).await?;
        Ok(LibraryModel {
            link,
            path,
            library,
        })
    }
    pub async fn modules(&self) -> Result<Vec<ModuleModel>> {
        let mut modules = vec![];
        for module_path in self.library.modules.iter() {
            let module_path = self.path.join(module_path);
            let module = ModuleModel::from_dir(module_path).await?;
            modules.push(module);
        }
        Ok(modules)
    }
    pub async fn lookup_module(&self, name: &str) -> Result<Vec<ModuleModel>> {
        let name = name.trim();
        let modules = self.modules().await?;
        let modules = modules
            .into_iter()
            .filter(|module| module.module.name == name)
            .collect::<Vec<_>>();
        Ok(modules)
    }
}
