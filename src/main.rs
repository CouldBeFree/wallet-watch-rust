mod handler;
mod dto;
mod utils;
mod routes;
mod state;
mod entity;

use axum::Router;
use crate::utils::db_connect::init_db;

#[tokio::main]
async fn main() {
    if let Ok(db) = init_db().await {
        // let state = state::app_state::AppState::new(db);
        let router = Router::new().nest("/api", routes::root::routes());
        // let router = Router::new().with_state(state).nest("/api", routes);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, router).await.unwrap();
    } else {
        panic!("Db connection error")
    }
}
