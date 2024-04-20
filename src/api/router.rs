use axum::{routing::get, Router};

use super::health_check;

pub fn create_router() -> Router {
    Router::new().route("/api/health", get(health_check))
}
