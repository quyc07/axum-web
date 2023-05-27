use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use redis::Commands;

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
    fn insert_teacher(&mut self, teacher: Teacher) {
        self.teachers.entry(teacher.name().to_string()).or_insert(Arc::new(Mutex::new(teacher.clone())));
    }

    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        self.teachers.get(name)
    }

    fn get_all_teachers(&self) -> Vec<&Arc<Mutex<Teacher>>> {
        self.teachers.values().collect()
    }

    fn contains_teacher(&self, name: &str) -> bool {
        self.teachers.contains_key(name)
    }

    fn insert_student(&mut self, student: Student) {
        self.students.entry(student.name().to_string()).or_insert(Arc::new(Mutex::new(student.clone())));
    }

    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        self.students.get(name)
    }

    fn get_all_students(&self) -> Vec<&Arc<Mutex<Student>>> {
        self.students.values().collect()
    }

    fn contains_student(&self, name: &str) -> bool {
        self.students.contains_key(name)
    }

    fn get_all_classes(&self) -> Vec<&Arc<Mutex<Class>>> {
        self.classes.values().collect()
    }
}

pub trait Db {
    fn insert_teacher(&mut self, teacher: Teacher);
    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>>;
    fn get_all_teachers(&self) -> Vec<&Arc<Mutex<Teacher>>>;
    fn contains_teacher(&self, name: &str) -> bool;
    fn insert_student(&mut self, student: Student);
    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>>;
    fn get_all_students(&self) -> Vec<&Arc<Mutex<Student>>>;
    fn contains_student(&self, name: &str) -> bool;
    fn get_all_classes(&self) -> Vec<&Arc<Mutex<Class>>>;
}


pub struct RedisDb {
    client: Client,
}

impl Default for RedisDb {
    fn default() -> Self {
        RedisDb {
            client: Client::new()
        }
    }
}

impl Db for RedisDb {
    fn insert_teacher(&mut self, teacher: Teacher) {
        todo!()
    }

    fn get_teacher_by_name(&self, name: &str) -> Option<&Arc<Mutex<Teacher>>> {
        todo!()
    }

    fn get_all_teachers(&self) -> Vec<&Arc<Mutex<Teacher>>> {
        todo!()
    }

    fn contains_teacher(&self, name: &str) -> bool {
        todo!()
    }

    fn insert_student(&mut self, student: Student) {
        todo!()
    }

    fn get_student_by_name(&self, name: &str) -> Option<&Arc<Mutex<Student>>> {
        todo!()
    }

    fn get_all_students(&self) -> Vec<&Arc<Mutex<Student>>> {
        todo!()
    }

    fn contains_student(&self, name: &str) -> bool {
        todo!()
    }

    fn get_all_classes(&self) -> Vec<&Arc<Mutex<Class>>> {
        todo!()
    }
}

pub struct Client {
    client: redis::Client,
}

impl Client {
    fn new() -> Client {
        // connect to redis
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        Client { client }
    }
}