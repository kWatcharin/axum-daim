pub mod config;
pub mod routers;

use axum::{
  routing::get, Router, response::Json
};
use serde_json::{
  Value, json
};


#[tokio::main]
async fn main() {
  /* Routers */ 
  /* Administration */ 
  let administration;
  administration = routers::administration::apis::router();

  /* Master Data */ 
  let master_data; 
  master_data = routers::administration::apis::router();
  
  /* APIs App */ 
  let app = Router::new()
    .route("/", get(root))
    .nest("/admisnistration", administration)
    .nest("/master_data", master_data);

  let listener;
    listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config::center::PORT))
    .await
    .unwrap();

  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> Json<Value> {
  Json(json!({ "text": "Hello, Welcome to Axum-Daim APIs." }))
}
