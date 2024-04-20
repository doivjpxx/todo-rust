use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::Local;

use crate::domain::models::todo::Todo;

pub async fn create_todo_command(
    Json(body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let datetime = Local::now();

    let todo = Todo {
        id: Some("1".to_string()),
        title: body.title,
        content: body.content,
        completed: Some(false),
        created_at: Some(datetime),
        updated_at: Some(datetime),
    };

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(serde_json::json!(json_response))))
}
