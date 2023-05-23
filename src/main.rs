use std::collections::HashMap;
use std::io::Bytes;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::school::{Gender, Teacher};

mod school;
mod db;

#[derive(Default)]
struct AppState {
    db: HashMap<String, String>,
}

type DbState = Arc<RwLock<Db>>;

type SharedState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let shared_db: Arc<RwLock<AppState>> = SharedState::default();

    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
        .route("/teacher/create", post(create_teacher)).with_state(Arc::clone(&shared_db))
        .route("/teacher", post(create_teacher1).with_state(Arc::clone(&shared_db)))
        .route("/teacher/:name", get(teacher).with_state(Arc::clone(&shared_db)));

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

/// 最后一个参数才是body
async fn create_teacher(State(shared_state): State<SharedState>, Json(payload): Json<Teacher>) -> (StatusCode, Json<Teacher>) {
    let teacher = Teacher::new(payload.name().to_string(), Gender::FEMALE, payload.age());
    shared_state.write().unwrap().db.insert(payload.name().to_string(), payload.name().to_string());
    (StatusCode::CREATED, Json(teacher))
}

async fn create_teacher1(State(shared_state): State<SharedState>) -> (StatusCode, Json<Teacher>) {
    let teacher = Teacher::new("xiaohong".to_string(), Gender::FEMALE, 18);
    shared_state.write().unwrap().db.insert("xiaohong".to_string(), "xiaohong".to_string());
    (StatusCode::CREATED, Json(teacher))
}

async fn teacher(Path(name): Path<String>, State(shared_state): State<SharedState>) -> Result<Json<Teacher>, StatusCode> {
    match shared_state.read().unwrap().db.get(name.as_str()) {
        None => Err(StatusCode::NOT_FOUND),
        Some(teacher_name) => Ok(Json(Teacher::new(teacher_name.to_string(), Gender::FEMALE, 18))),
    }
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