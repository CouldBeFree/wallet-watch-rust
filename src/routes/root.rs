use axum::Router;
use crate::routes::user::routes as user_routes;

pub fn routes() -> Router {
    let general_routes = Router::new()
        .merge(user_routes());
    Router::new().nest("/api", general_routes)
}