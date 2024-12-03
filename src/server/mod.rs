use axum::{extract::Path, response::Html, routing::get, Router};
use miette::{IntoDiagnostic, Result};
use std::{fs, path::PathBuf};
use tokio::net::TcpListener;

pub struct ServerOpt {
    port: u16,
    src: PathBuf,
}

impl Default for ServerOpt {
    fn default() -> Self {
        Self {
            port: 8080,
            src: PathBuf::new(),
        }
    }
}

impl ServerOpt {
    pub fn new(port: u16, src: PathBuf) -> Self {
        Self { port, src }
    }
}

#[derive(Default)]
pub struct Server {
    options: ServerOpt,
}

async fn index(src: PathBuf) -> String {
    println!("index {src:?}");
    "Hello, World".to_string()
}

async fn handle(src: PathBuf, Path(path): Path<String>) -> Html<String> {
    let file_path = src.join(path);
    let html = fs::read_to_string(file_path).unwrap();
    Html(html)
}

impl Server {
    pub fn new(options: ServerOpt) -> Self {
        Server { options }
    }

    pub async fn serve(self) -> Result<()> {
        let src = self.options.src.clone();
        println!("src: {src:?}");

        let app = Router::new()
            .route("/", get(move || index(src)))
            .route("/*key", get(move |a| handle(self.options.src, a)));

        let addr = format!("127.0.0.1:{}", self.options.port);
        let listener = TcpListener::bind(&addr).await.into_diagnostic()?;
        println!("Listing on http://{addr}");

        axum::serve(listener, app).await.into_diagnostic()
    }
}
