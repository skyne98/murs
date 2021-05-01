use std::path::PathBuf;

use anyhow::Result;
use murs::actions::{clean::clean, parse::parse, prepare::prepare};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mrk", about = "MURS Ruleset Keeper")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Find and parse the module definition")]
    Parse {
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },
    #[structopt(about = "Prepare the workspace")]
    Prepare {
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },
    #[structopt(about = "Clean the workspace")]
    Clean {
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    // Process the command and display the result
    match opt.command {
        Command::Parse { path } => {
            let path = path.unwrap_or(std::env::current_dir()?);
            let result = parse(&path).await?;
            println!("Parse: {:#?}", result);
        }
        Command::Prepare { path } => {
            let path = path.unwrap_or(std::env::current_dir()?);
            let result = prepare(&path).await?;
            println!("Prepare: {:#?}", result);
        }
        Command::Clean { path } => {
            let path = path.unwrap_or(std::env::current_dir()?);
            clean(&path).await?;
            println!("Cleaned successfuly");
        }
    }

    Ok(())
}
