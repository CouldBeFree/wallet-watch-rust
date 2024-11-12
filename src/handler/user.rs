use axum::{Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::dto::user_dto::User;
use crate::utils::validation_error_to_json::validation_errors_to_json;
use validator::{Validate};

pub async fn register_user(Json(payload): Json<User>) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => {
            "Registration successful".into_response()
        },
        Err(errors) => {
            let error_json = validation_errors_to_json(errors);
            (StatusCode::BAD_REQUEST, Json(error_json)).into_response()
        }
    }
}