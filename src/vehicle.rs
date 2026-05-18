use axum::Json;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    id: Option<String>,
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
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

pub async fn vehicle_post(Json(mut vehicle): Json<Vehicle>) -> Json<Vehicle> {
    vehicle.id = Some(uuid::Uuid::new_v4().to_string());
    Json(vehicle)
}
