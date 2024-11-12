mod handler;
mod dto;
mod utils;
mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::root::routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
