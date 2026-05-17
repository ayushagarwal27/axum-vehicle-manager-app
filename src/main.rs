use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Define routes
    let router = Router::new().route("/vehicle", get(vehicle_get));

    // Define listener
    let address = "0.0.0.0:3001";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // Launch web server
    axum::serve(listener, router).await.unwrap();
}

async fn vehicle_get() -> String {
    "BMW Z4".to_string()
}
