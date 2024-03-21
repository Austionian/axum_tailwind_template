mod configuration;
mod routes;

use axum::{routing::get, Router};
use lazy_static::lazy_static;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub use configuration::get_configuration;

lazy_static! {
    pub static ref TEMPLATES: tera::Tera = {
        match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        }
    };
}

#[derive(Clone)]
pub struct AppState {
    title: String,
}

pub fn startup() -> Result<Router, String> {
    // Create an AppState is shared across the app.
    let state = AppState {
        title: String::from("Axum Tailwind Template"),
    };

    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::root))
        .fallback(routes::handle_404)
        .layer(ServiceBuilder::new().layer(CompressionLayer::new()))
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state))
        .route("/health_check", get(routes::health_check)))
}
