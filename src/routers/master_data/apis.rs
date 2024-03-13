use axum::{
  routing::get,
  Json,
  Router
};
use serde_json::{
  Value, 
  json
};

pub fn router() -> Router {
  Router::new()
    .route("/get_master_data", get(get_master_data_api))
}

async fn get_master_data_api() -> Json<Value> {
  /* Get master data api */ 
  Json(json!({ "text": "get_master_data" }))
}