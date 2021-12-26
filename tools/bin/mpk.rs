use anyhow::Result;
use colored::*;
use log::info;
use murs_core::{
    cache::Cache,
    library::{
        graph::LibraryResolutionGraph,
        link::{LibraryLink, LibraryLinkGit},
    },
};
use murs_tools::parse_module_and_units;
use std::path::PathBuf;
use structopt::StructOpt;

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
    #[structopt(name = "lookup", about = "Lookup a module in a given library tree.")]
    Lookup { lib: String, module: String },
    #[structopt(
        name = "require",
        about = "Lookup a module which satisfies the version requirement in a given library tree."
    )]
    Require {
        lib: String,
        module: String,
        version: String,
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
            let book_build_path = module.module.package(&units).await?;
            info!("Book built at {}", book_build_path.display());
        }
        Opt::Serve { dir } => {
            let module_and_units = parse_module_and_units(dir).await?;
            let module = module_and_units.module;
            let units = module_and_units.units;

            // Build the book
            let book_build_path = module.module.package(&units).await?;
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
            let link = LibraryLinkGit {
                url,
                branch: Some(branch),
                commit: None,
            };
            cache.library_link(&LibraryLink::Git(link)).await?;
        }
        Opt::Debug(DebugOpt::Parse { dir }) => {
            let module_and_units = parse_module_and_units(dir).await?;
            println!("{:#?}", module_and_units);
        }
        Opt::Debug(DebugOpt::Lookup { lib, module }) => {
            let cache = Cache::new().await?;
            let link = lib.parse()?;
            let lib = cache.library_link(&link).await?;
            let library_graph = LibraryResolutionGraph::from_roots(vec![lib]).await?;
            let modules = library_graph.lookup_module(&module).await?;
            println!("Found: {:#?}", modules);
        }
        Opt::Debug(DebugOpt::Require {
            lib,
            module,
            version,
        }) => {
            let cache = Cache::new().await?;
            let link = lib.parse()?;
            let lib = cache.library_link(&link).await?;
            let library_graph = LibraryResolutionGraph::from_roots(vec![lib]).await?;
            let version_req = version.parse()?;
            let modules = library_graph.best_module(&module, &version_req).await?;
            println!("Found: {:#?}", modules);
        }
    }

    Ok(())
}
