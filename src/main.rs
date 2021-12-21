use async_fs;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Json, Response},
    routing::get,
    Router,
};
use serde_json::json;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/second", get(second));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

enum AppError {
    Root(RootError),
    Io(std::io::Error),
}

#[derive(Debug)]
enum RootError {
    #[allow(dead_code)]
    NotFound(String),
    #[allow(dead_code)]
    InvalidUsername(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Root(RootError::NotFound(error)) => (StatusCode::NOT_FOUND, error),
            AppError::Root(RootError::InvalidUsername(error)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, error)
            }
            AppError::Io(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(inner: std::io::Error) -> Self {
        AppError::Io(inner)
    }
}

async fn get_file() -> Result<String, AppError> {
    let file = async_fs::read_to_string("./test.txt").await?;

    Ok(file)
}

async fn root() -> Result<impl IntoResponse, AppError> {
    let file = get_file().await?;
    Ok((StatusCode::CREATED, Html(file)))
}

async fn get_file_app_error() -> Result<String, AppError> {
    let file = async_fs::read_to_string("./test.txt").await;
    match file {
        Ok(file) => Ok(file),
        Err(_) => Err(AppError::Root(RootError::NotFound(
            "this is a custom error".to_string(),
        ))),
    }
}

async fn second() -> Result<impl IntoResponse, AppError> {
    let file = get_file_app_error().await?;
    Ok((StatusCode::CREATED, Html("second route!")))
}
