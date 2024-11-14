use axum::Router;
use mongodb::Database;
use crate::routes::user::routes as user_routes;
use crate::state;
use crate::state::app_state::AppState;
use crate::utils::db_connect::init_db;

pub fn routes() -> Router {
     // let state = state::app_state::AppState::new(db);
     Router::new().merge(user_routes())
}