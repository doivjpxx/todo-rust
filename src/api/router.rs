use axum::{
    routing::{get, post},
    Router,
};

use crate::application::commands::todo::create::create_todo_command;

use super::health_check;

pub fn create_router() -> Router {
    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/todos", post(create_todo_command))
}
