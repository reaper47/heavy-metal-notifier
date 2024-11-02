//! The `web` module exposes the handlers for the web server.

mod handlers_calendar;
mod handlers_general;
mod templates;

use axum::{http::Uri, response::IntoResponse, routing::get, Router};
use reqwest::{header, StatusCode};
use rust_embed::Embed;

use crate::error::Result;
use handlers_calendar::routes_calendar;
use handlers_general::routes_general;

/// Creates the Router for the web server.
pub async fn routes() -> Result<Router> {
    let router = Router::new()
        .merge(routes_general())
        .nest("/calendar", routes_calendar())
        .route("/public/*file", get(static_handler));

    Ok(router)
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().strip_prefix("/public/").unwrap_or(uri.path()).to_string();

    StaticFile(path)   
}

#[derive(Embed)]
#[folder = "web/static/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where  
    T: Into<String>
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            },
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
    }
}