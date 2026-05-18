use axum::Json;

#[derive(Debug, serde::Serialize)]
pub struct Vehicle {
    id: String,
    manufacturer: String,
    model: String,
    year: u32,
}

// #[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Caller retrieved a vehicle");
    Json::from(Vehicle {
        manufacturer: "BMW".to_string(),
        model: "X1".to_string(),
        year: 2021,
        id: uuid::Uuid::new_v4().to_string(),
    })
}

pub async fn vehicle_post() -> String {
    "New Vehicle Created".to_string()
}
