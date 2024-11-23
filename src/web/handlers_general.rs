use super::templates::main::*;
use crate::model::EntitiesBmc;
use axum::{response::Redirect, routing::get, Router};

#[derive(Clone)]
pub struct AppState {
    pub bands: Vec<String>,
    pub genres: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        Self {
            bands: EntitiesBmc::bands(),
            genres: EntitiesBmc::genres(),
        }
    }
}

pub fn routes_general() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about_handler))
        .route("/contact", get(contact_handler).post(contact_post_handler))
        .route("/sitemap", get(sitemap_handler))
        .route("/tos", get(tos))
        .with_state(AppState::new())
}

async fn sitemap_handler() -> Redirect {
    Redirect::to("/public/sitemap.xml")
}
