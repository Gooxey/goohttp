use goohttp::axum::{
    extract::Path,
    response::IntoResponse
};

pub async fn remaining(Path(remaining): Path<String>) -> impl IntoResponse {
    format!("called remaining with the route `{remaining}`").into_response()
}