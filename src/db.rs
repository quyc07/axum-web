use std::sync::{Arc, Mutex};

use crate::school::{Class, Student, Teacher};
use crate::teacher;

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

    pub fn get_teacher_by_name(&self, name: String) -> Option<&Arc<Mutex<Teacher>>> {
        let mut teachers: Vec<&Arc<Mutex<Teacher>>> = self.teachers.iter()
            .filter(|x| x.lock().unwrap().name().eq(name.as_str()))
            .take(1)
            .collect();
        teachers.pop()
        // if let Ok(i) = self.teachers.binary_search_by_key(&name, |x| x.lock().unwrap().name().to_string()) {
        //     return self.teachers.get(i);
        // };
        // None
    }
    pub fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        if let Ok(i) = self.students.binary_search_by_key(&name.to_string(), |x| x.lock().unwrap().name().to_string()) {
            return self.students.get(i);
        };
        None
    }
}


