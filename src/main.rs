use std::collections::HashMap;
use std::io::Bytes;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

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
    let db_state = school::init().await;
    let app = Router::new()
        .route("/", get(index))
        .route("/users", post(create_user))
        .route("/teacher/create", post(create_teacher))
        .route("/teacher", post(create_teacher1).with_state(Arc::clone(&db_state)))
        // 共享状态既可以是method_router级别，也可以是Router级别，Router级别所有的method_router都可以共享
        .route("/teacher/:name", get(teacher1).with_state(Arc::clone(&db_state)))
        .with_state(Arc::clone(&shared_db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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

async fn create_teacher1(State(db_state): State<DbState>, Json(teacher): Json<Teacher>) -> (StatusCode, Json<Teacher>) {
    db_state.write().unwrap().teachers.push(Arc::new(Mutex::new(teacher.clone())));
    (StatusCode::CREATED, Json(teacher))
}

async fn teacher1(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Teacher>, StatusCode> {
    match shared_state.read().unwrap().get_teacher_by_name(name.as_str()) {
        None => Err(StatusCode::NOT_FOUND),
        Some(teacher) => Ok(Json(teacher.lock().unwrap().clone()))
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