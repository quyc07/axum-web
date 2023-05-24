use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};

use crate::db::Db;
use crate::school::{Student, Teacher};

mod school;
mod db;

#[derive(Default)]
struct AppState {
    db: Db,
}

type DbState = Arc<RwLock<AppState>>;

#[tokio::main]
async fn main() {
    let db_state = school::init().await;
    let app = Router::new()
        .route("/", get(index))
        .route("/student", post(create_student))
        .route("/student/:name", get(student))
        .route("/teacher", post(create_teacher))
        // 共享状态既可以是method_router级别，也可以是Router级别，Router级别所有的method_router都可以共享
        .route("/teacher/:name", get(teacher))
        .with_state(Arc::clone(&db_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// 最后一个参数才是body
async fn create_teacher(State(db_state): State<DbState>, Json(teacher): Json<Teacher>) -> (StatusCode, Json<Teacher>) {
    db_state.write().unwrap().db.teachers.push(Arc::new(Mutex::new(teacher.clone())));
    (StatusCode::CREATED, Json(teacher))
}

async fn teacher(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Teacher>, StatusCode> {
    match shared_state.read().unwrap().db.get_teacher_by_name(name) {
        None => Err(StatusCode::NOT_FOUND),
        Some(teacher) => Ok(Json(teacher.lock().unwrap().clone()))
    }
}


async fn index() -> &'static str {
    "Welcome to 八七小学!"
}

async fn create_student(State(db_state): State<DbState>, Json(student): Json<Student>) -> (StatusCode, Json<Student>) {
    db_state.write().unwrap().db.students.push(Arc::new(Mutex::new(student.clone())));
    (StatusCode::CREATED, Json(student))
}

async fn student(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Student>, StatusCode> {
    match shared_state.read().unwrap().db.get_student_by_name(name.as_str()) {
        None => Err(StatusCode::NOT_FOUND),
        Some(student) => Ok(Json(student.lock().unwrap().clone()))
    }
}