use anyhow::Result;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mrk", about = "MURS Ruleset Keeper")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    Ok(())
}
