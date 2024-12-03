// TODO:
// - Make as executable everywhere
// - File navigation
// - Extract template, compilation, cache
// - Hot reloading with cache in ~/.tn
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
    #[arg(short, long, default_value = "./config.toml")]
    config: PathBuf,

    /// Port on which to serve content
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let cache_dir = tn::get_cache_dir();

    let config = Config::parse(Path::new(&args.config))?;
    let files = crawler::crawl(&config.src)?;

    let root = Compiler::new(config.src, cache_dir, config.name).compile(files)?;
    Server::new(ServerOpt::new(args.port, root, config.assets.unwrap()))
        .serve()
        .await
}
