use anyhow::Result;
use colored::*;
use indicatif::ProgressBar;
use log::info;
use mpk::{
    actions::package::package_module,
    utils::fs::{read_package_manifest, read_units},
};
use std::{env::current_dir, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mpk",
    about = "Module and package keeper for all of your MURS needs."
)]
enum Opt {
    #[structopt(
        name = "parse",
        about = "Parse the current environment for debug purposes."
    )]
    Parse {
        #[structopt(parse(from_os_str), short, long)]
        dir: Option<PathBuf>,
    },
    #[structopt(
        name = "package",
        alias = "pkg",
        about = "Package the contents of a module into a book."
    )]
    Package {
        #[structopt(parse(from_os_str), short, long)]
        dir: Option<PathBuf>,
    },
    #[structopt(
        name = "serve",
        about = "Package the contents of a module into a book and serve it."
    )]
    Serve {
        #[structopt(parse(from_os_str), short, long)]
        dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    match opt {
        Opt::Parse { dir } => {
            // Read/parse the module and units
            let dir = if let Some(dir) = dir {
                dir
            } else {
                let current_dir = current_dir()?;
                current_dir
            };
            let (manifest_path, module) = read_package_manifest(dir).await?;
            println!("{:#?}", module);

            let unit_dir = manifest_path.parent().unwrap().join("units");
            let units = read_units(unit_dir).await?;
            println!("{:#?}", units);
        }
        Opt::Package { dir } => {
            // Read/parse the module and units
            let dir = if let Some(dir) = dir {
                dir
            } else {
                let current_dir = current_dir()?;
                current_dir
            };
            let (manifest_path, module) = read_package_manifest(dir).await?;

            let unit_dir = manifest_path.parent().unwrap().join("units");
            let units = read_units(unit_dir).await?;

            // Build the book
            let book_build_path = package_module(&module, &units).await?;
            info!("Book built at {}", book_build_path.display());
        }
        Opt::Serve { dir } => {
            // Read/parse the module and units
            let dir = if let Some(dir) = dir {
                dir
            } else {
                let current_dir = current_dir()?;
                current_dir
            };
            let (manifest_path, module) = read_package_manifest(dir).await?;

            let unit_dir = manifest_path.parent().unwrap().join("units");
            let units = read_units(unit_dir).await?;

            // Build the book
            let book_build_path = package_module(&module, &units).await?;
            info!("Book built at {}", book_build_path.display());

            // Serve the files
            println!("Serving the book at {}", "127.0.0.1:3030".yellow());
            warp::serve(warp::fs::dir(book_build_path))
                .run(([127, 0, 0, 1], 3030))
                .await;
        }
    }

    Ok(())
}
