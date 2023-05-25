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
    let class_name_1 = class1.lock().unwrap().name.clone();
    db_state.write().unwrap().db.classes.entry(class_name_1).or_insert(class1);
    let class_name_2 = class2.lock().unwrap().name.clone();
    db_state.write().unwrap().db.classes.entry(class_name_2).or_insert(class2);
    let class_name_3 = class3.lock().unwrap().name.clone();
    db_state.write().unwrap().db.classes.entry(class_name_3).or_insert(class3);
    let class_name_4 = class4.lock().unwrap().name.clone();
    db_state.write().unwrap().db.classes.entry(class_name_4).or_insert(class4);
    let ming_ming_name = ming_ming.lock().unwrap().name.clone();
    let fang_fang_name = fang_fang.lock().unwrap().name.clone();
    let xiao_hong_name = xiao_hong.lock().unwrap().name.clone();
    db_state.write().unwrap().db.teachers.insert(ming_ming_name, ming_ming.clone());
    db_state.write().unwrap().db.teachers.insert(fang_fang_name,fang_fang.clone());
    db_state.write().unwrap().db.teachers.insert(xiao_hong_name,xiao_hong.clone());
    db_state
}