use axum::{response::Redirect, routing::get, Router};

use super::templates::main::*;

pub fn routes_general() -> Router {
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