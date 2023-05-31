use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use askama::Template;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

use crate::db::Db;
use crate::db::hashmap_db::HashMapDb;
use crate::db::mysql_db::MysqlDb;
use crate::db::redis_db::RedisDb;
use crate::school::{Class, Student, Teacher};
use crate::templates::askama_template::HelloTemplate;

mod school;
mod err;
mod db;
mod templates;

#[derive(Default)]
struct AppState<T> {
    db: T,
}

type DbState = Arc<RwLock<AppState<MysqlDb>>>;

#[tokio::main]
async fn main() {
    // 注意，env_logger 必须尽可能早的初始化
    env_logger::init();
    let db_state = Arc::new(RwLock::new(AppState::<MysqlDb>::default()));
    db_state.write().unwrap().db.init();
    let student_route = Router::new()
        .route("/student", post(create_student))
        .route("/student/:name", get(student))
        .route("/students", get(students))
        .route("/student/update", post(update_student))
        .route("/students/template", get(students_template));
    let teacher_router = Router::new()
        .route("/teacher", post(create_teacher))
        .route("/teacher/:name", get(teacher))
        .route("/teachers", get(teachers));
    let app = Router::new()
        .merge(student_route)
        .merge(teacher_router)
        .route("/", get(index))
        .route("/classes", get(classes))
        .route("/common", get(common))
        // 共享状态既可以是method_router级别，也可以是Router级别，Router级别所有的method_router都可以共享
        .with_state(Arc::clone(&db_state));


    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// 最后一个参数才是body
async fn create_teacher(State(db_state): State<DbState>, Json(teacher): Json<Teacher>) -> Result<Json<Teacher>, StatusCode> {
    let mut guard = db_state.write().unwrap();
    if guard.db.contains_teacher(teacher.name()) {
        return Err(StatusCode::CONFLICT);
    }
    guard.db.insert_teacher(teacher.clone())?;
    Ok(Json(teacher))
}

async fn teacher(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Teacher>, StatusCode> {
    Ok(Json(shared_state.read().unwrap().db.get_teacher_by_name(name.as_str())?.lock().unwrap().clone()))
}


async fn index() -> &'static str {
    "Welcome to 八七小学!"
}

async fn create_student(State(db_state): State<DbState>, Json(student): Json<Student>) -> Result<Json<Student>, StatusCode> {
    let mut guard = db_state.write().unwrap();
    if guard.db.contains_student(student.name()) {
        return Err(StatusCode::CONFLICT);
    }
    guard.db.insert_student(student.clone())?;
    Ok(Json(student))
}

async fn student(Path(name): Path<String>, State(shared_state): State<DbState>) -> Result<Json<Student>, StatusCode> {
    Ok(Json(shared_state.read().unwrap().db.get_student_by_name(name.as_str())?.lock().unwrap().clone()))
}

async fn teachers(State(db_state): State<DbState>) -> Result<Json<Vec<Teacher>>, StatusCode> {
    Ok(Json(db_state.read().unwrap().db.get_all_teachers()?.iter().map(|x| x.lock().unwrap().clone()).collect()))
}

async fn students(State(db_state): State<DbState>) -> Result<Json<Vec<Student>>, StatusCode> {
    Ok(Json(db_state.read().unwrap().db.get_all_students()?.iter().map(|x| x.lock().unwrap().clone()).collect()))
}

async fn update_student(State(db_state): State<DbState>, Json(student): Json<Student>) -> Result<Json<Student>, StatusCode> {
    let mut db_state = db_state.write().unwrap();
    if db_state.db.contains_student(student.name()) {
        db_state.db.insert_student(student.clone())?;
        return Ok(Json(student));
    }
    Err(StatusCode::NOT_FOUND)
}

async fn classes(State(db_state): State<DbState>) -> Result<Json<Vec<ClassVo>>, StatusCode> {
    Ok(Json(db_state.read().unwrap().db.get_all_classes()?
        .iter().map(|x| ClassVo::from(x.lock().unwrap().to_owned())).collect()))
}

async fn students_template(State(db_state): State<DbState>) -> Result<Html<String>, StatusCode> {
    let students: Vec<Student> = db_state.read().unwrap().db.get_all_students()?
        .iter().map(|x| x.lock().unwrap().clone())
        .collect();
    let template = HelloTemplate { students };
    Ok(Html(template.render().unwrap()))
}

async fn common() -> Html<String> {
    Html(templates::askama_template::CommonTemplate{}.render().unwrap())
}

#[derive(Deserialize, Serialize, Clone)]
struct ClassVo {
    name: String,
    teacher: Teacher,
    students: Vec<Student>,
}

impl From<Class> for ClassVo {
    fn from(class: Class) -> Self {
        ClassVo {
            name: class.name().to_string(),
            teacher: class.teacher().lock().unwrap().clone(),
            students: class.students().iter().map(|x| x.lock().unwrap().clone()).collect(),
        }
    }
}
