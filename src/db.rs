use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::school::{Class, Student, Teacher};
use crate::teacher;

#[derive(Default)]
pub struct Db {
    pub classes: HashMap<String, Arc<Mutex<Class>>>,
    pub teachers: HashMap<String, Arc<Mutex<Teacher>>>,
    pub students: HashMap<String, Arc<Mutex<Student>>>,
}

impl Db {

    pub fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        self.teachers.get(name)
        // let mut teachers: Vec<&Arc<Mutex<Teacher>>> = self.teachers.iter()
        //     .filter(|x| x.lock().unwrap().name().eq(name.as_str()))
        //     .take(1)
        //     .collect();
        // teachers.pop()
    }
    pub fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        self.students.get(name)
    }
}


