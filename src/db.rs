use std::sync::{Arc, Mutex};

use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};

pub mod hashmap_db;
pub mod mysql_db;
pub mod redis_db;

pub trait Db {
    fn init(&mut self) {
        let ming_ming = Arc::new(Mutex::new(Student::new(
            "mingming".to_string(),
            Gender::MALE,
            7,
        )));
        let fang_fang = Arc::new(Mutex::new(Student::new(
            "fangfang".to_string(),
            Gender::FEMALE,
            8,
        )));
        let xiao_hong = Arc::new(Mutex::new(Student::new(
            "xiaohong".to_string(),
            Gender::FEMALE,
            10,
        )));
        let xiao_bai = Arc::new(Mutex::new(Student::new(
            "xiaobai".to_string(),
            Gender::FEMALE,
            8,
        )));
        let wang_hai = Arc::new(Mutex::new(Student::new(
            "wanghai".to_string(),
            Gender::FEMALE,
            10,
        )));
        let ling_ling = Arc::new(Mutex::new(Student::new(
            "lingling".to_string(),
            Gender::FEMALE,
            8,
        )));
        let hui_hui = Arc::new(Mutex::new(Student::new(
            "huihui".to_string(),
            Gender::FEMALE,
            10,
        )));
        let qing_qing = Arc::new(Mutex::new(Student::new(
            "qingqing".to_string(),
            Gender::MALE,
            26,
        )));
        let zhang_san = Arc::new(Mutex::new(Teacher::new(
            "zhangsan".to_string(),
            Gender::MALE,
            26,
        )));
        let li_si = Arc::new(Mutex::new(Teacher::new(
            "lisi".to_string(),
            Gender::MALE,
            29,
        )));
        let wang_wu = Arc::new(Mutex::new(Teacher::new(
            "wangwu".to_string(),
            Gender::MALE,
            30,
        )));
        let class1 = Class::new(
            "1-1".to_string(),
            zhang_san.clone(),
            vec![ming_ming.clone(), fang_fang.clone()],
        );
        let class2 = Class::new(
            "1-2".to_string(),
            li_si.clone(),
            vec![xiao_bai.clone(), xiao_hong.clone()],
        );
        let class3 = Class::new(
            "2-1".to_string(),
            wang_wu.clone(),
            vec![wang_hai.clone(), ling_ling.clone()],
        );
        let class4 = Class::new(
            "2-2".to_string(),
            zhang_san.clone(),
            vec![hui_hui.clone(), qing_qing.clone()],
        );
        self.insert_class(class1);
        self.insert_class(class2);
        self.insert_class(class3);
        self.insert_class(class4);
        self.insert_teacher(zhang_san.lock().unwrap().to_owned());
        self.insert_teacher(wang_wu.lock().unwrap().to_owned());
        self.insert_teacher(li_si.lock().unwrap().to_owned());
        self.insert_student(ming_ming.lock().unwrap().to_owned());
        self.insert_student(fang_fang.lock().unwrap().to_owned());
        self.insert_student(xiao_hong.lock().unwrap().to_owned());
        self.insert_student(xiao_bai.lock().unwrap().to_owned());
        self.insert_student(wang_hai.lock().unwrap().to_owned());
        self.insert_student(ling_ling.lock().unwrap().to_owned());
        self.insert_student(hui_hui.lock().unwrap().to_owned());
        self.insert_student(qing_qing.lock().unwrap().to_owned());
    }
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr>;
    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr>;
    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr>;
    fn contains_teacher(&self, name: &str) -> bool;
    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr>;
    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr>;
    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr>;
    fn contains_student(&self, name: &str) -> bool;
    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr>;
    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr>;
}
