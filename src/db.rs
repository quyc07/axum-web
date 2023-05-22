use std::sync::{Arc, RwLock};
use crate::school::{Class, Student, Teacher};

#[derive(Default)]
pub struct Db<'a> {
    classes: Vec<&'a Class<'a>>,
    teachers: Vec<&'a Teacher>,
    students: Vec<&'a Student>,
}

impl Db<'_> {
    fn new() -> Db<'static> {
        Db {
            classes: vec![],
            teachers: vec![],
            students: vec![],
        }
    }

    pub fn add_class(&mut self, class: &Class) {
        self.classes.push(class);
    }
    pub fn add_teacher(&mut self, teacher: &Teacher) {
        self.teachers.push(teacher);
    }
    pub fn add_student(&mut self, student: &Student) {
        self.students.push(student);
    }
    pub fn next_class_id(&self) -> u8 {
        (self.classes.len() + 1) as u8
    }

    pub fn next_teacher_id(&self) -> u8 {
        (self.teachers.len() + 1) as u8
    }

    pub fn next_student_id(&self) -> u8 {
        (self.students.len() + 1) as u8
    }
}


