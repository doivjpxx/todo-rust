use axum::{response::IntoResponse, Json};

pub mod router;

pub async fn health_check() -> impl IntoResponse {
    const MESSAGE: &str = "Good!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE,
    });

    Json(json_response)
}
