mod school;
mod db;

use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use crate::db::Db;

type DbState = Arc<RwLock<Db<'static>>>;

#[tokio::main]
async fn main() {
    let shared_state = DbState::default();
    init(shared_state);

    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn init(shared_state: DbState) {
    school::init(shared_state).await;
}


async fn index() -> &'static str {
    "hello world!"
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserResponse>) {
    let user_response = UserResponse {
        id: 1,
        name: payload.name.to_string(),
        age: payload.age,
    };
    (StatusCode::CREATED, Json(user_response))
}



#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    name: String,
    age: u8,
}