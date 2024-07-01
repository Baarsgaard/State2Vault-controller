use std::{fmt::Display, str::FromStr};

use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or("info".into()))
        .with(fmt::layer())
        .init();

    let app = Router::new()
        .route("/api/password/:pass_id", get(get_password))
        .route("/api/passwords/:list_id", get(get_passwordlist))
        .route("/api/folder/:parent/:doc_id", get(get_document));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[instrument(level = "info")]
async fn get_password(Path(id): Path<u32>) -> impl IntoResponse {
    format!("{}", id)
}

#[instrument(level = "info")]
async fn get_passwordlist(Path(id): Path<u32>) -> impl IntoResponse {
    format!("{}", id)
}

#[instrument(level = "info")]
async fn get_document(Path((parent, doc_id)): Path<(DocParent, u32)>) -> impl IntoResponse {
    format!("{}/{}", parent, doc_id)
}

#[derive(Debug, Deserialize)]
enum DocParent {
    Password,
    PasswordList,
    Folder,
}
impl FromStr for DocParent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "password" => Ok(Self::Password),
            "passwordlist" => Ok(Self::PasswordList),
            "folder" => Ok(Self::Folder),
            _ => Ok(Self::Password),
        }
    }
}
impl Display for DocParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DocParent::Password => "password",
                DocParent::PasswordList => "passwordlist",
                DocParent::Folder => "folder",
            }
        )
    }
}
