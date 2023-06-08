use std::sync::{Arc, Mutex};

use axum::async_trait;

use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};
use crate::sea::teacher;

#[async_trait]
pub trait AsyncDb {
    fn init(&mut self) {
        let ming_ming = Student::new("mingming".to_string(), Gender::MALE, 7);
        let fang_fang = Student::new("fangfang".to_string(), Gender::FEMALE, 8);
        let xiao_hong = Student::new("xiaohong".to_string(), Gender::FEMALE, 10);
        let xiao_bai = Student::new("xiaobai".to_string(), Gender::FEMALE, 8);
        let wang_hai = Student::new("wanghai".to_string(), Gender::FEMALE, 10);
        let ling_ling = Student::new("lingling".to_string(), Gender::FEMALE, 8);
        let hui_hui = Student::new("huihui".to_string(), Gender::FEMALE, 10);
        let qing_qing = Student::new("qingqing".to_string(), Gender::MALE, 26);
        let zhang_san = Teacher::new("zhangsan".to_string(), Gender::MALE, 26);
        let li_si = Teacher::new("lisi".to_string(), Gender::MALE, 29);
        let wang_wu = Teacher::new("wangwu".to_string(), Gender::MALE, 30);
        let class1 = Class::new(
            "1-1".to_string(),
            Arc::new(Mutex::new(zhang_san.clone())),
            vec![
                Arc::new(Mutex::new(ming_ming.clone())),
                Arc::new(Mutex::new(fang_fang.clone())),
            ],
        );
        let class2 = Class::new(
            "1-2".to_string(),
            Arc::new(Mutex::new(li_si.clone())),
            vec![
                Arc::new(Mutex::new(xiao_bai.clone())),
                Arc::new(Mutex::new(xiao_hong.clone())),
            ],
        );
        let class3 = Class::new(
            "2-1".to_string(),
            Arc::new(Mutex::new(wang_wu.clone())),
            vec![
                Arc::new(Mutex::new(wang_hai.clone())),
                Arc::new(Mutex::new(ling_ling.clone())),
            ],
        );
        let class4 = Class::new(
            "2-2".to_string(),
            Arc::new(Mutex::new(zhang_san.clone())),
            vec![
                Arc::new(Mutex::new(hui_hui.clone())),
                Arc::new(Mutex::new(qing_qing.clone())),
            ],
        );
        self.insert_class(class1);
        self.insert_class(class2);
        self.insert_class(class3);
        self.insert_class(class4);
        self.insert_teacher(zhang_san);
        self.insert_teacher(wang_wu);
        self.insert_teacher(li_si);
        self.insert_student(ming_ming.lock().unwrap().to_owned());
        self.insert_student(fang_fang.lock().unwrap().to_owned());
        self.insert_student(xiao_hong.lock().unwrap().to_owned());
        self.insert_student(xiao_bai.lock().unwrap().to_owned());
        self.insert_student(wang_hai.lock().unwrap().to_owned());
        self.insert_student(ling_ling.lock().unwrap().to_owned());
        self.insert_student(hui_hui.lock().unwrap().to_owned());
        self.insert_student(qing_qing.lock().unwrap().to_owned());
    }
    async fn insert_teacher(&mut self, teacher: Teacher) -> Result<Teacher, SchoolErr>;
    async fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr>;
    async fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr>;
    async fn contains_teacher(&self, name: &str) -> bool;
    async fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr>;
    async fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr>;
    async fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr>;
    async fn contains_student(&self, name: &str) -> bool;
    async fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr>;
    async fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr>;
}
