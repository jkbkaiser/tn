// TODO:
// - Recompile every 10 sec
// - File navigation
// - Link fixed stylesheet
// - Resolve references
use clap::Parser;
use std::path::Path;

use tn::compiler::Compiler;
use tn::config::Config;
use tn::crawler;
use tn::server::{Server, ServerOpt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();
    let config = Config::parse(Path::new(&args.config))?;
    let files = crawler::crawl(&config.src)?;

    Compiler::new(config.src, config.dst.clone()).compile(files)?;
    Server::new(ServerOpt::new(8080, config.dst)).serve().await
}
