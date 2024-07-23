use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, get_service},
    Json, Router,
};

use serde_json::{json};

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use axum::extract::Path;

use tower_http::services::ServeFile;

use std::{io};



#[derive(Deserialize)] 
struct CreateUser { 
    username: String, 
}

#[derive(Debug, Serialize,Deserialize, Clone, Eq, Hash, PartialEq)] 
struct User { 
    id: u64, 
    username: String, 
} 

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(create_user))
        .route("/hello/:name", get(json_hello))
        .route("/hello/mul/:number", get(multiplied_number))
        .route("/static", get_service(ServeFile::new("static/hello.html"))
        .handle_error(|error: io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }));



    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse {
    let greeting = name.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({"message": hello + greeting})))
}

async fn multiplied_number(Path(number): Path<i32>) -> impl IntoResponse {
    let multiplied_number = number * 3;

    (StatusCode::OK, Json(json!({"message": multiplied_number})))
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse { 
    let user = User { 
            id: 1337, 
            username: payload.username 

    };

(StatusCode::CREATED, Json(user)) 
}