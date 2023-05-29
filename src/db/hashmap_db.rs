use std::collections::HashMap;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use crate::db::Db;
use crate::err::SchoolErr;
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
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        self.teachers.entry(teacher.name().to_string()).or_insert(Arc::new(Mutex::new(teacher.clone())));
        Ok(())
    }

    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        if let Some(teacher) = self.teachers.get(name) {
            return Ok(teacher.clone());
        }
        Err(SchoolErr::NotFound)
    }

    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        Ok(self.teachers.values().map(|x| Arc::clone(x)).collect())
    }

    fn contains_teacher(&self, name: &str) -> bool {
        self.teachers.contains_key(name)
    }

    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        self.students.entry(student.name().to_string()).or_insert(Arc::new(Mutex::new(student.clone())));
        Ok(())
    }

    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        if let Some(student) = self.students.get(name) {
            return Ok(student.clone());
        }
        Err(SchoolErr::NotFound)
    }

    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        Ok(self.students.values().map(|x| Arc::clone(x)).collect())
    }

    fn contains_student(&self, name: &str) -> bool {
        self.students.contains_key(name)
    }

    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        Ok(self.classes.values().map(|x| Arc::clone(x)).collect())
    }

    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        self.classes.entry(class.name().to_string()).or_insert(Arc::new(Mutex::new(class.clone())));
        Ok(())
    }
}



