use goohttp::axum::{response::IntoResponse, extract::Path};

pub async fn say_hello(Path(caller): Path<String>) -> impl IntoResponse {
    format!("said hello from {caller}").into_response()
}