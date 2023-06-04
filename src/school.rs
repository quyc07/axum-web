use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Class {
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

impl Gender {
    pub fn name(&self) -> String {
        match self {
            Gender::MALE => "Male".to_string(),
            Gender::FEMALE => "Female".to_string(),
        }
    }
}

impl Class {
    pub fn new(
        name: String,
        teacher: Arc<Mutex<Teacher>>,
        students: Vec<Arc<Mutex<Student>>>,
    ) -> Class {
        Class {
            name,
            teacher,
            students,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn teacher(&self) -> &Arc<Mutex<Teacher>> {
        &self.teacher
    }
    pub fn students(&self) -> &Vec<Arc<Mutex<Student>>> {
        &self.students
    }
}

impl Teacher {
    pub fn new(name: String, gender: Gender, age: u8) -> Teacher {
        Teacher { name, gender, age }
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
    pub fn new(name: String, gender: Gender, age: u8) -> Student {
        Student { name, gender, age }
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
