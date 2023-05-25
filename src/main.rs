use std::net::SocketAddr;
use std::sync::{Arc, Mutex, RwLock};

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

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
        .route("/students", get(students))
        .route("/student/update", post(update_student))
        .route("/teacher", post(create_teacher))
        // 共享状态既可以是method_router级别，也可以是Router级别，Router级别所有的method_router都可以共享
        .route("/teacher/:name", get(teacher))
        .route("/teachers", get(teachers))
        .route("/classes", get(classes))
        .with_state(Arc::clone(&db_state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// 最后一个参数才是body
async fn create_teacher(State(db_state): State<DbState>, Json(teacher): Json<Teacher>) -> (StatusCode, Json<Teacher>) {
    let mut guard = db_state.write().unwrap();
    if guard.db.teachers.contains_key(teacher.name()) {
        return (StatusCode::CONFLICT, Json(teacher));
    }
    guard.db.teachers.entry(teacher.name().to_string()).or_insert(Arc::new(Mutex::new(teacher.clone())));
    (StatusCode::CREATED, Json(teacher))
}

async fn teacher(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Teacher>, StatusCode> {
    match shared_state.read().unwrap().db.get_teacher_by_name(name.as_str()) {
        None => Err(StatusCode::NOT_FOUND),
        Some(teacher) => Ok(Json(teacher.lock().unwrap().clone()))
    }
}


async fn index() -> &'static str {
    "Welcome to 八七小学!"
}

async fn create_student(State(db_state): State<DbState>, Json(student): Json<Student>) -> (StatusCode, Json<Student>) {
    let mut guard = db_state.write().unwrap();
    if guard.db.students.contains_key(student.name()) {
        return (StatusCode::CONFLICT, Json(student));
    }
    guard.db.students.entry(student.name().to_string()).or_insert(Arc::new(Mutex::new(student.clone())));
    (StatusCode::CREATED, Json(student))
}

async fn student(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Student>, StatusCode> {
    match shared_state.read().unwrap().db.get_student_by_name(name.as_str()) {
        None => Err(StatusCode::NOT_FOUND),
        Some(student) => Ok(Json(student.lock().unwrap().clone()))
    }
}

async fn teachers(State(db_state): State<DbState>) -> (StatusCode, Json<Vec<Teacher>>) {
    (StatusCode::OK, Json(db_state.read().unwrap().db.teachers.values().map(|x| x.lock().unwrap().clone()).collect()))
}

async fn students(State(db_state): State<DbState>) -> (StatusCode, Json<Vec<Student>>) {
    (StatusCode::OK, Json(db_state.read().unwrap().db.students.values().map(|x| x.lock().unwrap().clone()).collect()))
}

async fn update_student(State(db_state): State<DbState>, Json(student): Json<Student>) -> (StatusCode, Json<Student>) {
    let mut db_state = db_state.write().unwrap();
    return match db_state.db.get_student_by_name(student.name()) {
        Some(_) => {
            db_state.db.students.insert(student.name().to_string(), Arc::new(Mutex::new(student.clone())));
            (StatusCode::OK, Json(student))
        }
        None => (StatusCode::NOT_FOUND, Json(student)),
    };
}

async fn classes(State(db_state): State<DbState>) -> (StatusCode, Json<Vec<ClassVo>>) {
    (StatusCode::OK, Json(db_state.read().unwrap().db.classes.values().map(|x| {
        let class = x.lock().unwrap();
        let class_vo = ClassVo {
            name: class.name().to_string(),
            teacher: class.teacher().lock().unwrap().clone(),
            students: class.students().iter().map(|x| x.lock().unwrap().clone()).collect(),
        };
        class_vo
    }).collect()))
}

#[derive(Deserialize, Serialize, Clone)]
struct ClassVo {
    name: String,
    teacher: Teacher,
    students: Vec<Student>,
}
