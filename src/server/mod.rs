use axum::{routing::get_service, Router};
use miette::{IntoDiagnostic, Result};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

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

impl Server {
    pub fn new(options: ServerOpt) -> Self {
        Server { options }
    }

    pub async fn serve(self) -> Result<()> {
        let asset_service = get_service(ServeDir::new("./assets")).handle_error(|err| async move {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", err),
            )
        });

        let markdown_service =
            get_service(ServeDir::new(self.options.src)).handle_error(|err| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", err),
                )
            });

        let app = Router::new()
            .nest_service("/assets", asset_service)
            .nest_service("/", markdown_service);

        let addr = SocketAddr::from(([127, 0, 0, 1], self.options.port));
        let listener = TcpListener::bind(&addr).await.into_diagnostic()?;
        println!("Listening on http://{}", addr);

        axum::serve(listener, app).await.into_diagnostic()
    }
}
