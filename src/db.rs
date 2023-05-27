use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::school::{Class, Student, Teacher};

#[derive(Default)]
pub struct HashMapDb {
    pub classes: HashMap<String, Arc<Mutex<Class>>>,
    pub teachers: HashMap<String, Arc<Mutex<Teacher>>>,
    pub students: HashMap<String, Arc<Mutex<Student>>>,
}

impl HashMapDb {
    pub fn new() -> HashMapDb {
        HashMapDb {
            classes: Default::default(),
            teachers: Default::default(),
            students: Default::default(),
        }
    }
}

impl Db for HashMapDb {
    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        self.teachers.get(name)
    }
    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        self.students.get(name)
    }
}

pub trait Db {
    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>>;
    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>>;
}


#[derive(Default)]
pub struct RedisDb{

}

impl Db for RedisDb {
    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        todo!()
    }

    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        todo!()
    }
}