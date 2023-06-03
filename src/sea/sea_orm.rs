use std::sync::{Arc, Mutex};
use axum_web::err::SchoolErr;
use crate::async_db::AsyncDb;
use crate::sea::prelude::{Class, Student, Teacher};

pub struct SeaOrm{

}

#[tonic::async_trait]
impl AsyncDb for SeaOrm{
    async fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        todo!()
    }

    async fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        todo!()
    }

    async fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        todo!()
    }

    async fn contains_teacher(&self, name: &str) -> bool {
        todo!()
    }

    async fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        todo!()
    }

    async fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        todo!()
    }

    async fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        todo!()
    }

    async fn contains_student(&self, name: &str) -> bool {
        todo!()
    }

    async fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        todo!()
    }

    async fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        todo!()
    }
}