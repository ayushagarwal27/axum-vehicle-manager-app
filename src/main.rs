use axum::{Router, routing::get};

mod vehicle;
use vehicle::{vehicle_get, vehicle_post};

#[tokio::main]
async fn main() {
    // Define routes
    let router = Router::new().route("/vehicle", get(vehicle_get).post(vehicle_post));

    // Define listener
    let address = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Launch web server
    axum::serve(listener, router).await.unwrap();
}
