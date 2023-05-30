use std::collections::HashMap;
use std::ops::Index;
use std::sync::{Arc, Mutex};
use mysql::{Column, FromRowError, Params, params, Pool, Row, Value};
use mysql::prelude::{FromRow, Queryable, ToValue};
use tokio::fs::read_to_string;
use crate::classes;
use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::{Class, Student, Teacher};

pub struct MysqlDb {
    pool: Pool,
}

impl Default for MysqlDb {
    fn default() -> Self {
        let url = "mysql://root:abc123@localhost:3306/mydb";
        MysqlDb {
            pool: Pool::new(url).unwrap()
        }
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
        let mut conn = self.pool.get_conn()?;
        let x1 = class.students().iter().map(|x| x.lock().unwrap().name()).interspace(",").collect::<String>();
        conn.exec_drop(r"insert into class(name,teacher,students) values(:name,:teacher,:students)", params! {
            "name"=>class.name(),
            "teacher"=>class.teacher().lock().unwrap().name(),
            "students"=>class.students().iter().map(|x| x.lock().unwrap().name()).intersperse(",").collect::<String>(),
        })?;
        // println!("{classes}");
        Ok(())
    }
}