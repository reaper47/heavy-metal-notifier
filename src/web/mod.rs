//! The `web` module exposes the handlers for the web server.

mod handlers_calendar;
mod handlers_general;
mod templates;

use axum::{http::Uri, response::IntoResponse, routing::get, Router};
use reqwest::{header, StatusCode};
use rust_embed::Embed;
use std::sync::Arc;

use crate::{
    error::Result,
    model::{CalendarRepository, EntitiesRepository, FeedRepository},
};
use handlers_calendar::routes_calendar;
use handlers_general::routes_general;

/// Shared application state for the Axum web server.
#[derive(Clone)]
pub struct AppState {
    pub bands: Vec<String>,
    pub genres: [String; 46],
    pub calendar_repo: Arc<dyn CalendarRepository + Send + Sync>,
    pub feed_repo: Arc<dyn FeedRepository + Send + Sync>,
}

impl AppState {
    pub fn new(
        calendar_repo: Arc<dyn CalendarRepository + Send + Sync>,
        entities_repo: Arc<dyn EntitiesRepository + Send + Sync>,
        feed_repo: Arc<dyn FeedRepository + Send + Sync>,
    ) -> Self {
        Self {
            bands: entities_repo.bands(),
            genres: [
                String::from("Atmospheric Black Metal"),
                String::from("Avantgarde Metal"),
                String::from("Black Metal"),
                String::from("Blackened Thrash Metal"),
                String::from("Brutal Death Metal"),
                String::from("Crossover"),
                String::from("Death Metal"),
                String::from("Deathcore"),
                String::from("Deathgrind"),
                String::from("Djent"),
                String::from("Doom Metal"),
                String::from("Drone"),
                String::from("Folk Metal"),
                String::from("Funeral Doom Metal"),
                String::from("Glam Metal"),
                String::from("Goregrind"),
                String::from("Groove Metal"),
                String::from("Grindcore"),
                String::from("Hair Metal"),
                String::from("Hardcore"),
                String::from("Heavy Metal"),
                String::from("Industrial Metal"),
                String::from("Math Metal"),
                String::from("Mathcore"),
                String::from("Melodic Death Metal"),
                String::from("Metalcore"),
                String::from("Neoclassical Metal"),
                String::from("Nu Metal"),
                String::from("Pagan Metal"),
                String::from("Pirate Metal"),
                String::from("Post-Black Metal"),
                String::from("Post-Hardcore"),
                String::from("Post-Metal"),
                String::from("Power Metal"),
                String::from("Powerviolence"),
                String::from("Progressive Metal"),
                String::from("Sludge Metal"),
                String::from("Slam Death Metal"),
                String::from("Speed Metal"),
                String::from("Stoner Doom Metal"),
                String::from("Symphonic Black Metal"),
                String::from("Symphonic Metal"),
                String::from("Technical Death Metal"),
                String::from("Thrash Metal"),
                String::from("True Norwegian Black Metal"),
                String::from("Viking Metal"),
            ],
            calendar_repo,
            feed_repo,
        }
    }
}

/// Creates the Router for the web server.
pub async fn routes() -> Result<Router<AppState>> {
    let router = Router::new()
        .merge(routes_general())
        .nest("/calendar", routes_calendar())
        .route("/public/*file", get(static_handler));

    Ok(router)
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri
        .path()
        .strip_prefix("/public/")
        .unwrap_or(uri.path())
        .to_string();

    StaticFile(path)
}

#[derive(Embed)]
#[folder = "web/static/"]
struct Asset;

/// Wrapper type for serving static files in the web application.
pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> axum::response::Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
