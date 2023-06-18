use goohttp::axum::response::IntoResponse;

pub async fn index() -> impl IntoResponse {
    "index".into_response()
}