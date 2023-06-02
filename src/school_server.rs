use tonic::{Request, Response, Status};
use tonic::transport::Server;

use axum_web::db::Db;
use axum_web::db::hashmap_db::HashMapDb;

use crate::school_proto::{
    StudentByNameRequest, StudentResponse, TeacherByNameRequest, TeacherResponse,
};
use crate::school_proto::school_service_server::{SchoolService, SchoolServiceServer};

pub mod school_proto {
    tonic::include_proto!("school_proto");
}

pub struct SchoolServiceHashMapDb {
    db: HashMapDb,
}

impl Default for SchoolServiceHashMapDb {
    fn default() -> Self {
        let mut hash_map_db = HashMapDb::default();
        hash_map_db.init();
        Self { db: hash_map_db }
    }
}

#[tonic::async_trait]
impl SchoolService for SchoolServiceHashMapDb {
    async fn get_teacher_by_name(
        &self,
        request: Request<TeacherByNameRequest>,
    ) -> Result<Response<TeacherResponse>, Status> {
        let request = request.into_inner();
        match self.db.get_teacher_by_name(&request.name) {
            Ok(teacher) => {
                let teacher = teacher.lock().unwrap();
                Ok(Response::new(TeacherResponse {
                    name: teacher.name().to_string(),
                    gender: teacher.gender().name(),
                    age: teacher.age() as i32,
                }))
            }
            Err(_) => return Err(Status::not_found("Teacher not found")),
        }
    }

    async fn get_student_by_name(
        &self,
        request: Request<StudentByNameRequest>,
    ) -> Result<Response<StudentResponse>, Status> {
        let student = request.into_inner();
        match self.db.get_student_by_name(&student.name) {
            Ok(student) => {
                let student = student.lock().unwrap();
                Ok(Response::new(StudentResponse {
                    name: student.name().to_string(),
                    gender: student.gender().name(),
                    age: student.age() as i32,
                }))
            }
            Err(_) => Err(Status::not_found("Student not found")),
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:10000".parse().unwrap();
    let school_service = SchoolServiceHashMapDb::default();

    Server::builder()
        .add_service(SchoolServiceServer::new(school_service))
        .serve(addr)
        .await
        .unwrap();
}
