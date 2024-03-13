use axum::{
  routing::get,
  Router,
  response::Json
};
use serde_json::{
  Value,
  json
};

pub mod config;
pub mod routers;


#[tokio::main]
async fn main() {
  let app = Router::new()
    .route("/", get(root))
    .nest("/admisnistration", routers::administration::apis::router())
    .nest("/master_data", routers::administration::apis::router());

  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config::center::PORT))
    .await
    .unwrap();

  axum::serve(listener, app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> Json<Value> {
  Json(json!({ "text": "Hello, Welcome to Axum-Daim APIs." }))
}
