use std::sync::{Arc, Mutex};
use sqlx::{ConnectOptions, Error, MySql, Pool};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use tokio::fs::read_to_string;
use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::{Class, Student, Teacher};

pub struct MysqlDb {
    pool: Pool<MySql>,
}

impl MysqlDb {
    pub(crate) async fn new() -> Result<MysqlDb, Error> {
        Ok(MysqlDb {
            pool: MySqlPoolOptions::new().max_connections(5).connect_with(MySqlConnectOptions::new()
                .host("127.0.0.1").username("root").password("abc123").database("mydb")).await?
        })
    }
}

impl Db for MysqlDb {
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        todo!()
    }

    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        todo!()
    }

    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        todo!()
    }

    fn contains_teacher(&self, name: &str) -> bool {
        todo!()
    }

    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        todo!()
    }

    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        todo!()
    }

    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        todo!()
    }

    fn contains_student(&self, name: &str) -> bool {
        todo!()
    }

    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        todo!()
    }

    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        todo!()
    }
}