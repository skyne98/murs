use std::path::PathBuf;

use crate::{
    models::{module::Module, unit::Unit},
    utils::fs::clear_dir,
};
use anyhow::Result;
use log::info;
use mdbook::{config::BookConfig, Config, MDBook};
use tempfile::tempdir;

pub async fn package_module(module: &Module, units: &Vec<Unit>) -> Result<PathBuf> {
    // Setup the book
    let book_directory = tempdir()?.into_path();
    let mut config: Config = Default::default();
    let mut book_config: BookConfig = Default::default();
    book_config.title = Some(format!("{} {}", module.title.clone(), module.version));
    config.book = book_config;
    let mdbook = MDBook::init(&book_directory)
        .with_config(config.clone())
        .build()?;
    let build_dir = mdbook.build_dir_for("HTML");
    let src_dir = mdbook.source_dir();
    info!("Initialized a book at {}", book_directory.display());

    // Clean the directory
    clear_dir(&src_dir).await?;
    clear_dir(&build_dir).await?;

    // Populate the source directory
    // - Populate the SUMMARY.md
    let summary_path = src_dir.join("SUMMARY.md");
    let summary_contents = summary_contents(module, units);
    tokio::fs::write(summary_path, summary_contents).await?;

    // - Populate units
    for unit in units {
        let unit_path = src_dir.join(format!("{}.md", unit.name));
        tokio::fs::write(unit_path, &unit.contents).await?;
    }

    // Make a build
    let mdbook = MDBook::load(book_directory)?;
    mdbook.build()?;

    Ok(build_dir)
}

pub fn summary_contents(module: &Module, units: &Vec<Unit>) -> String {
    let header = format!("#Summary");
    let mut unit_links = vec![];
    for unit in units {
        unit_links.push(format!("- [{}]({}.md)", unit.title, unit.name));
    }
    let unit_links_joined = unit_links.join("\n");
    format!("{}\n{}", header, unit_links_joined)
}
