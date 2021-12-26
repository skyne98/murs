use anyhow::Result;
use colored::*;
use core::{cache::Cache, module::Module, unit::Unit};
use indicatif::ProgressBar;
use log::info;
use std::{env::current_dir, path::PathBuf};
use structopt::StructOpt;
use tools::parse_module_and_units;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mpk",
    about = "The only tool you need to manage your MURS modules"
)]
enum Opt {
    #[structopt(
        name = "package",
        alias = "pkg",
        about = "Package the contents of a module into a book."
    )]
    Package {
        #[structopt(parse(from_os_str))]
        dir: Option<PathBuf>,
    },
    #[structopt(
        name = "serve",
        about = "Package the contents of a module into a book and serve it."
    )]
    Serve {
        #[structopt(parse(from_os_str))]
        dir: Option<PathBuf>,
    },
    #[structopt(name = "cache", about = "Commands used for operating the MURS cache.")]
    Cache(CacheOpt),
    #[structopt(name = "debug", about = "Commands used for debugging purposes.")]
    Debug(DebugOpt),
}
#[derive(Debug, StructOpt)]
enum CacheOpt {
    #[structopt(name = "clean", about = "Completely empty the MURS cache.")]
    Clean {},
    #[structopt(
        name = "git",
        about = "Commands used for operating git repositories in the cache."
    )]
    Git(CacheGitOpt),
}
#[derive(Debug, StructOpt)]
enum CacheGitOpt {
    #[structopt(name = "ensure", about = "Clone or pull the repository.")]
    Ensure { url: String, branch: String },
}

#[derive(Debug, StructOpt)]
enum DebugOpt {
    #[structopt(name = "parse", about = "Parse the module and units.")]
    Parse {
        #[structopt(parse(from_os_str))]
        dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    match opt {
        Opt::Package { dir } => {
            let module_and_units = parse_module_and_units(dir).await?;
            let module = module_and_units.module;
            let units = module_and_units.units;

            // Build the book
            let book_build_path = module.package(&units).await?;
            info!("Book built at {}", book_build_path.display());
        }
        Opt::Serve { dir } => {
            let module_and_units = parse_module_and_units(dir).await?;
            let module = module_and_units.module;
            let units = module_and_units.units;

            // Build the book
            let book_build_path = module.package(&units).await?;
            info!("Book built at {}", book_build_path.display());

            // Serve the files
            println!("Serving the book at {}", "127.0.0.1:3030".yellow());
            warp::serve(warp::fs::dir(book_build_path))
                .run(([127, 0, 0, 1], 3030))
                .await;
        }
        Opt::Cache(CacheOpt::Clean {}) => {
            let cache = Cache::new().await?;
            info!("Emptying the cache...");
            cache.clean().await?;
        }
        Opt::Cache(CacheOpt::Git(CacheGitOpt::Ensure { url, branch })) => {
            let cache = Cache::new().await?;
            info!("Updating repository {}...", url.yellow());
            cache.git(&url, &branch).await?;
        }
        Opt::Debug(DebugOpt::Parse { dir }) => {
            let module_and_units = parse_module_and_units(dir).await?;
            println!("{:#?}", module_and_units);
        }
    }

    Ok(())
}
