use axum::{extract::Path, response::IntoResponse};

pub async fn say_hello_caller_sender(Path((caller, sender)): Path<(String, String)>) -> impl IntoResponse {
    format!("said hello from {caller} to {sender}").into_response()
}