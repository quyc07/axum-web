use std::sync::{Arc, Mutex};

use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status};

use crate::db::hashmap_db::HashMapDb;
use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::Teacher;
use crate::school_proto::school_service_server::SchoolService;
use crate::school_proto::{TeacherByNameRequest, TeacherResponse};

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
}

fn main() {}
