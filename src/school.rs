use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};
use crate::db::HashMapDb;

use crate::{AppState, DbState};

#[derive(Clone)]
pub struct Class<> {
    pub(crate) name: String,
    teacher: Arc<Mutex<Teacher>>,
    students: Vec<Arc<Mutex<Student>>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Student {
    pub(crate) name: String,
    gender: Gender,
    age: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Teacher {
    pub(crate) name: String,
    gender: Gender,
    age: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Gender {
    MALE,
    FEMALE,
}

impl Class {
    pub(crate) fn new(name: String, teacher: Arc<Mutex<Teacher>>, students: Vec<Arc<Mutex<Student>>>) -> Class {
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
    pub(crate) fn new(name: String, gender: Gender, age: u8) -> Student {
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

// pub(crate) fn init() -> DbState {
//     let ming_ming = Arc::new(Mutex::new(Student::new("mingming".to_string(), Gender::MALE, 7)));
//     let fang_fang = Arc::new(Mutex::new(Student::new("fangfang".to_string(), Gender::FEMALE, 8)));
//     let xiao_hong = Arc::new(Mutex::new(Student::new("xiaohong".to_string(), Gender::FEMALE, 10)));
//     let xiao_bai = Arc::new(Mutex::new(Student::new("xiaobai".to_string(), Gender::FEMALE, 8)));
//     let wang_hai = Arc::new(Mutex::new(Student::new("wanghai".to_string(), Gender::FEMALE, 10)));
//     let ling_ling = Arc::new(Mutex::new(Student::new("lingling".to_string(), Gender::FEMALE, 8)));
//     let hui_hui = Arc::new(Mutex::new(Student::new("huihui".to_string(), Gender::FEMALE, 10)));
//     let qing_qing = Arc::new(Mutex::new(Student::new("qingqing".to_string(), Gender::MALE, 26)));
//     let zhang_san = Arc::new(Mutex::new(Teacher::new("zhangsan".to_string(), Gender::MALE, 26)));
//     let li_si = Arc::new(Mutex::new(Teacher::new("lisi".to_string(), Gender::MALE, 29)));
//     let wang_wu = Arc::new(Mutex::new(Teacher::new("wangwu".to_string(), Gender::MALE, 30)));
//     let class1 = Arc::new(Mutex::new(Class::new("1-1".to_string(), Arc::clone(&zhang_san), vec![Arc::clone(&ming_ming), Arc::clone(&fang_fang)])));
//     let class2 = Arc::new(Mutex::new(Class::new("1-2".to_string(), Arc::clone(&li_si), vec![Arc::clone(&xiao_bai), Arc::clone(&xiao_hong)])));
//     let class3 = Arc::new(Mutex::new(Class::new("2-1".to_string(), Arc::clone(&wang_wu), vec![Arc::clone(&wang_hai), Arc::clone(&ling_ling)])));
//     let class4 = Arc::new(Mutex::new(Class::new("2-2".to_string(), Arc::clone(&zhang_san), vec![Arc::clone(&hui_hui), Arc::clone(&qing_qing)])));
//     let db_state = Arc::new(RwLock::new(AppState{db:HashMapDb::new()}));
//     let class_name_1 = class1.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.classes.entry(class_name_1).or_insert(class1);
//     let class_name_2 = class2.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.classes.entry(class_name_2).or_insert(class2);
//     let class_name_3 = class3.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.classes.entry(class_name_3).or_insert(class3);
//     let class_name_4 = class4.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.classes.entry(class_name_4).or_insert(class4);
//     let zhang_san_name = zhang_san.lock().unwrap().name.clone();
//     let wang_wu_name = wang_wu.lock().unwrap().name.clone();
//     let li_si_name = li_si.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.teachers.insert(zhang_san_name, Arc::clone(&zhang_san));
//     db_state.write().unwrap().db.teachers.insert(wang_wu_name, Arc::clone(&wang_wu));
//     db_state.write().unwrap().db.teachers.insert(li_si_name, Arc::clone(&li_si));
//     let ming_ming_name = ming_ming.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(ming_ming_name, Arc::clone(&ming_ming));
//     let fang_fang_name = fang_fang.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(fang_fang_name, Arc::clone(&fang_fang));
//     let xiao_hong_name = xiao_hong.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(xiao_hong_name, Arc::clone(&xiao_hong));
//     let xiao_bai_name = xiao_bai.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(xiao_bai_name, Arc::clone(&xiao_bai));
//     let wang_hai_name = wang_hai.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(wang_hai_name, Arc::clone(&wang_hai));
//     let ling_ling_name = ling_ling.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(ling_ling_name, Arc::clone(&ling_ling));
//     let hui_hui_name = hui_hui.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(hui_hui_name, Arc::clone(&hui_hui));
//     let qing_qing_name = qing_qing.lock().unwrap().name.clone();
//     db_state.write().unwrap().db.students.insert(qing_qing_name, Arc::clone(&qing_qing));
//     db_state
// }