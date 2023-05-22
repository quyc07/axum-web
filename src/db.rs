use std::sync::RwLock;

use crate::school::{Class, Student, Teacher};

#[derive(Default)]
pub struct Db {
    // classes: Vec<Class<'a>>,
    teachers: Vec<Teacher>,
    students: Vec<Student>,
}

impl Db {
    fn new() -> Db {
        Db {
            // classes: vec![],
            teachers: vec![],
            students: vec![],
        }
    }

    // pub fn add_class(&mut self, class: Class) {
    //     self.classes.push(class);
    // }
    pub fn add_teacher(&mut self, teacher: Teacher) {
        self.teachers.push(teacher);
    }
    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }
    // pub fn get_teacher_by_name(&self, name: &str) -> &Teacher {
    //     let teacher = self.teachers.iter().filter(|&x| &x.name() == name).collect();
    //     teacher
    // }
}


