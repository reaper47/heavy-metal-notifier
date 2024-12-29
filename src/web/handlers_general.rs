use super::templates::main::*;
use crate::model::EntitiesBmc;
use axum::{response::Redirect, routing::get, Router};

#[derive(Clone)]
pub struct AppState {
    pub bands: Vec<String>,
    pub genres: [String; 46],
}

impl AppState {
    fn new() -> Self {
        Self {
            bands: EntitiesBmc::bands(),
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
