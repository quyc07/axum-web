use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::{Json, Router};
use axum::extract::{DefaultBodyLimit, Path, State};
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::routing::{get, post, post_service};
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::school::{Gender, Teacher};

mod school;
mod db;

type DbState = Arc<RwLock<Db>>;

#[tokio::main]
async fn main() {
    let shared_db: HashMap<String, Teacher> = HashMap::new();
    // init(&shared_db);

    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
        .route("/teachers", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn init(shared_state: &DbState) {
    school::init(shared_state).await;
}

// async fn teacher(Path(name): Path<String>, State(db_state): State<DbState>) ->(StatusCode,Json<Teacher>){
//     let db = db_state.read().unwrap();
//     let teacher = db.get_teacher_by_name(name.as_str());
//     (StatusCode::FOUND,Json(Teacher{
//         name:teacher.name().to_string(),
//         gender: teacher.gender().clone(),
//         age: teacher.age(),
//     }))
// }

async fn create_teacher(Json(payload): Json<Teacher>) -> (StatusCode, Json<Teacher>) {
    let teacher = Teacher::new(payload.name().to_string(), Gender::FEMALE, payload.age());
    (StatusCode::CREATED, Json(teacher))
}


async fn index() -> &'static str {
    "hello world!"
}

async fn create_user1(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserResponse>) {
    let user_response = UserResponse {
        id: 1,
        name: payload.name.to_string(),
        age: payload.age,
    };
    (StatusCode::CREATED, Json(user_response))
}async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserResponse>) {
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