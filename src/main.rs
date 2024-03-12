pub mod config;
use config::main_config::PORT;

use::axum::{
	routing::get,
	routing::post,
	Router,
	response::Json
};
use serde_json::{Value, json};


#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/", get(root))
		.route("/my_post", post(my_post));

	let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", PORT))
		.await
		.unwrap();

	axum::serve(listener, app.into_make_service())
		.await
		.unwrap();
}

async fn root() -> &'static str {
	"Hello, World!"
}

async fn my_post() -> Json<Value> {
	Json(
		json!(
			{ 
				"data": 42,
				"name": "John"
			}
		)
	)
}