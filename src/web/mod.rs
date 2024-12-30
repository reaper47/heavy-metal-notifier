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
                "Atmospheric Black Metal".to_string(),
                "Avantgarde Metal".to_string(),
                "Black Metal".to_string(),
                "Blackened Thrash Metal".to_string(),
                "Brutal Death Metal".to_string(),
                "Crossover".to_string(),
                "Death Metal".to_string(),
                "Deathcore".to_string(),
                "Deathgrind".to_string(),
                "Djent".to_string(),
                "Doom Metal".to_string(),
                "Drone".to_string(),
                "Folk Metal".to_string(),
                "Funeral Doom Metal".to_string(),
                "Glam Metal".to_string(),
                "Goregrind".to_string(),
                "Groove Metal".to_string(),
                "Grindcore".to_string(),
                "Hair Metal".to_string(),
                "Hardcore".to_string(),
                "Heavy Metal".to_string(),
                "Industrial Metal".to_string(),
                "Math Metal".to_string(),
                "Mathcore".to_string(),
                "Melodic Death Metal".to_string(),
                "Metalcore".to_string(),
                "Neoclassical Metal".to_string(),
                "Nu Metal".to_string(),
                "Pagan Metal".to_string(),
                "Pirate Metal".to_string(),
                "Post-Black Metal".to_string(),
                "Post-Hardcore".to_string(),
                "Post-Metal".to_string(),
                "Power Metal".to_string(),
                "Powerviolence".to_string(),
                "Progressive Metal".to_string(),
                "Sludge Metal".to_string(),
                "Slam Death Metal".to_string(),
                "Speed Metal".to_string(),
                "Stoner Doom Metal".to_string(),
                "Symphonic Black Metal".to_string(),
                "Symphonic Metal".to_string(),
                "Technical Death Metal".to_string(),
                "Thrash Metal".to_string(),
                "True Norwegian Black Metal".to_string(),
                "Viking Metal".to_string(),
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
