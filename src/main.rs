// TODO:
// - Make as executable everywhere
// - File navigation
// - Hot reloading with cache in ~/.tn
// - Add favicon
// - Extract template
// - Print error msg when not found https://docs.rs/axum/latest/axum/error_handling/index.html
// - (Support both absolute and relative paths in config)
use clap::Parser;
use std::path::{Path, PathBuf};

use tn::compiler::Compiler;
use tn::config::Config;
use tn::crawler;
use tn::server::{Server, ServerOpt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long)]
    #[arg(short, long, default_value = "./example/config.toml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let config = Config::parse(Path::new(&args.config))?;
    let files = crawler::crawl(&config.src)?;

    Compiler::new(config.src, config.dst.clone()).compile(files)?;
    Server::new(ServerOpt::new(8080, config.dst)).serve().await
}
