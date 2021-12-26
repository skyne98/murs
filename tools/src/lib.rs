use anyhow::Result;
use murs_core::{module::model::ModuleModel, unit::Unit};
use std::{env::current_dir, path::Path};

#[derive(Debug)]
pub struct ParsedModuleAndUnits {
    pub module: ModuleModel,
    pub units: Vec<Unit>,
}

pub async fn parse_module_and_units<P: AsRef<Path>>(
    dir: Option<P>,
) -> Result<ParsedModuleAndUnits> {
    // Read/parse the module and units
    let dir = if let Some(dir) = dir {
        dir.as_ref().to_path_buf()
    } else {
        let current_dir = current_dir()?;
        current_dir
    };
    let module = ModuleModel::from_dir(dir).await?;
    let manifest_path = module.path.join("module.toml");

    let unit_dir = manifest_path.parent().unwrap().join("units");
    let units = Unit::from_path(unit_dir).await?;

    Ok(ParsedModuleAndUnits { module, units })
}
