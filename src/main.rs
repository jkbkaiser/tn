// TODO:
// - Print error msg when not found https://docs.rs/axum/latest/axum/error_handling/index.html
// - (Support both absolute and relative paths in config)
use clap::Parser;
use miette::IntoDiagnostic;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

use tn::config::Config;
use tn::crawler;
use tn::generator::Generator;
use tn::server::{Server, ServerOpt};

/// Tn is tool that can parse and serve markdown files.
/// Parsed files are served alongside other assets e.g. .css files, images, etc.
/// The file `index.nav` in the root of the project tells tn what to dislpay in the navbar.
/// This file also has a markdown format.
#[derive(Parser, Debug)]
#[command(version, verbatim_doc_comment, about, long_about = None)]
struct Args {
    /// Path to the configuration file
    #[arg(short, long, default_value = "./tn.toml")]
    config: PathBuf,

    /// Port on which to serve content
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Specified whether to reload on changes
    #[arg(short, long, default_value_t = false)]
    watch: bool,
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = Args::parse();

    let cache_dir = tn::get_cache_dir();

    let config = Config::parse(Path::new(&args.config))?;
    let files = crawler::crawl(&config.src)?;

    let mut generator = Generator::new(
        config.src.clone(),
        cache_dir.clone(),
        config.name.clone(),
        args.watch,
    )?;
    generator.generate(&files)?;

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).into_diagnostic()?;
    watcher
        .watch(Path::new(&config.src), RecursiveMode::Recursive)
        .into_diagnostic()?;

    tokio::spawn(async move {
        for event in rx {
            match event {
                Ok(Event {
                    kind: EventKind::Modify(_),
                    paths,
                    attrs,
                }) => {
                    generator
                        .generate(&paths)
                        .expect("Could not generate files");
                }
                Err(e) => println!("watch error: {:?}", e),
                _ => {}
            }
        }
    });

    let root = cache_dir.join(config.name);
    Server::new(ServerOpt::new(args.port, root, config.assets.unwrap()))
        .serve()
        .await
}
