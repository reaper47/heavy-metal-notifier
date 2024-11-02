use axum::{routing::get, Router};

use super::templates::main::*;

pub fn routes_general() -> Router {
    Router::new()
        .route("/", get(index()))
        .route("/about", get(about()))
        .route("/contact", get(contact(false)).post(contact(true)))
        .route("/privacy", get(privacy()))
        .route("/tos", get(tos()))
}
