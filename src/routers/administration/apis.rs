use axum::{
  routing::get,
  routing::post,
  Router,
  Json
};
use serde_json::{
  Value, json
};

pub fn router() -> Router {
  Router::new()
    .route("/get_admin", get(get_admin_api))
    .route("/post_admin", post(post_admin_api))
}

async fn get_admin_api() -> Json<Value> {
  /* Get master data api */ 
  Json(json!({ "text": "get_admin" }))
}

async fn post_admin_api() -> Json<Value> {
  /* Post master data api */ 
  Json(json!({ "text": "post_admin" }))
}