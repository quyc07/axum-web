use tonic::{Request, Response, Status};
use crate::db::hashmap_db::HashMapDb;
use crate::db::school_proto::school_service_server::SchoolService;
use crate::db::school_proto::{TeacherByNameRequest, TeacherResponse};

pub struct SchoolServiceHashMapDb {
    db: HashMapDb,
}

impl SchoolService for SchoolServiceHashMapDb{
    async fn get_teacher_by_name(&self, request: Request<TeacherByNameRequest>) -> Result<Response<TeacherResponse>, Status> {

    }
}



fn main() {}
