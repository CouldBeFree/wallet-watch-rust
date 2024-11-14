use axum::Router;
use axum::routing::{post};
use crate::handler::user::register_user;
use crate::state::app_state::AppState;

pub fn routes() -> Router {
    Router::new().route("/auth/register", post(register_user))
}