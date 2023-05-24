use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::DbState;

#[derive(Clone)]
pub struct Class<> {
    name: String,
    teacher: Arc<Mutex<Teacher>>,
    students: Vec<Arc<Mutex<Student>>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Student {
    name: String,
    gender: Gender,
    age: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Teacher {
    name: String,
    gender: Gender,
    age: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Gender {
    MALE,
    FEMALE,
}

impl Class {
    fn new(name: String, teacher: Arc<Mutex<Teacher>>) -> Class {
        Class {
            name,
            teacher,
            students: Vec::new(),
        }
    }
}

impl Teacher {
    pub fn new(name: String, gender: Gender, age: u8) -> Teacher {
        Teacher {
            name,
            gender,
            age,
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn age(&self) -> u8 {
        self.age
    }
    pub fn gender(&self) -> &Gender {
        &self.gender
    }
}

impl Student {
    fn new(name: String, gender: Gender, age: u8) -> Student {
        Student {
            name,
            gender,
            age,
        }
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn age(&self) -> u8 {
        self.age
    }
    pub fn gender(&self) -> &Gender {
        &self.gender
    }
}

pub(crate) async fn init() -> DbState {
    let ming_ming = Arc::new(Mutex::new(Teacher::new("mingming".to_string(), Gender::MALE, 23)));
    let fang_fang = Arc::new(Mutex::new(Teacher::new("fangfang".to_string(), Gender::FEMALE, 22)));
    let xiao_hong = Arc::new(Mutex::new(Teacher::new("xiaohong".to_string(), Gender::FEMALE, 26)));
    let class1 = Arc::new(Mutex::new(Class::new("1-1".to_string(), ming_ming.clone())));
    let class2 = Arc::new(Mutex::new(Class::new("1-2".to_string(), fang_fang.clone())));
    let class3 = Arc::new(Mutex::new(Class::new("2-1".to_string(), xiao_hong.clone())));
    let class4 = Arc::new(Mutex::new(Class::new("2-2".to_string(), ming_ming.clone())));
    let db_state = DbState::default();
    db_state.write().unwrap().db.classes.push(class1);
    db_state.write().unwrap().db.classes.push(class2);
    db_state.write().unwrap().db.classes.push(class3);
    db_state.write().unwrap().db.classes.push(class4);
    db_state.write().unwrap().db.teachers.push(ming_ming.clone());
    db_state.write().unwrap().db.teachers.push(fang_fang.clone());
    db_state.write().unwrap().db.teachers.push(xiao_hong.clone());
    db_state
}