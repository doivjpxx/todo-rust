use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    domain::models::todo::NewTodo,
    infrastructure::data::repositories::todo_repository::TodoRepository,
};

pub async fn create_todo_command(
    Json(body): Json<NewTodo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let repository = TodoRepository::new();

    if let Ok(tood) = repository.get_by_title(body.title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error".to_string(),
            "message": format!("Todo with title {} already exists", tood.title),
        });

        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!(json_response)),
        ));
    }

    let todo = NewTodo {
        title: body.title.clone(),
        content: body.content.clone(),
    };

    let todo = repository.create_todo(todo).await.unwrap()[0].to_owned();

    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": todo,
    });

    Ok((StatusCode::CREATED, Json(serde_json::json!(json_response))))
}
