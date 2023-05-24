use std::sync::{Arc, Mutex};

use crate::school::{Class, Student, Teacher};

#[derive(Default)]
pub struct Db {
    pub classes: Vec<Arc<Mutex<Class>>>,
    pub teachers: Vec<Arc<Mutex<Teacher>>>,
    pub students: Vec<Arc<Mutex<Student>>>,
}

impl Db {
    fn new() -> Db {
        Db {
            classes: vec![],
            teachers: vec![],
            students: vec![],
        }
    }

    pub fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        if let Ok(i) = self.teachers.binary_search_by_key(&name.to_string().to_string(),|x| x.lock().unwrap().name().to_string()) {
            return self.teachers.get(i);
        };
        None
    }
}


