use axum::{Router, response::Redirect, routing::get};

use super::templates::main::*;
use crate::web::AppState;

/// Defines the routes for general endpoints of the web application.
pub fn routes_general() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/about", get(about_handler))
        .route("/contact", get(contact_handler).post(contact_post_handler))
        .route("/sitemap", get(sitemap_handler))
        .route("/tos", get(tos))
}

async fn sitemap_handler() -> Redirect {
    Redirect::to("/public/sitemap.xml")
}
