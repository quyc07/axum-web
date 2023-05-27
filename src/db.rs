use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use redis::{Commands, RedisResult};

use crate::school::{Class, Gender, Student, Teacher};

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

    fn get_teacher_by_name(&self, name: &str) -> Option<Arc<Mutex<Teacher>>> {
        Some(Arc::clone(self.teachers.get(name)?))
    }

    fn get_all_teachers(&self) -> Vec<Arc<Mutex<Teacher>>> {
        self.teachers.values().map(|x| Arc::clone(x)).collect()
    }

    fn contains_teacher(&self, name: &str) -> bool {
        self.teachers.contains_key(name)
    }

    fn insert_student(&mut self, student: Student) {
        self.students.entry(student.name().to_string()).or_insert(Arc::new(Mutex::new(student.clone())));
    }

    fn get_student_by_name(&self, name: &str) -> Option<Arc<Mutex<Student>>> {
        Some(Arc::clone(self.students.get(name)?))
    }

    fn get_all_students(&self) -> Vec<Arc<Mutex<Student>>> {
        self.students.values().map(|x| Arc::clone(x)).collect()
    }

    fn contains_student(&self, name: &str) -> bool {
        self.students.contains_key(name)
    }

    fn get_all_classes(&self) -> Vec<Arc<Mutex<Class>>> {
        self.classes.values().map(|x| Arc::clone(x)).collect()
    }

    fn insert_class(&mut self, class: Class) {
        self.classes.entry(class.name().to_string()).or_insert(Arc::new(Mutex::new(class.clone())));
    }
}

pub trait Db {
    fn init(&mut self) {
        let ming_ming = Arc::new(Mutex::new(Student::new("mingming".to_string(), Gender::MALE, 7)));
        let fang_fang = Arc::new(Mutex::new(Student::new("fangfang".to_string(), Gender::FEMALE, 8)));
        let xiao_hong = Arc::new(Mutex::new(Student::new("xiaohong".to_string(), Gender::FEMALE, 10)));
        let xiao_bai = Arc::new(Mutex::new(Student::new("xiaobai".to_string(), Gender::FEMALE, 8)));
        let wang_hai = Arc::new(Mutex::new(Student::new("wanghai".to_string(), Gender::FEMALE, 10)));
        let ling_ling = Arc::new(Mutex::new(Student::new("lingling".to_string(), Gender::FEMALE, 8)));
        let hui_hui = Arc::new(Mutex::new(Student::new("huihui".to_string(), Gender::FEMALE, 10)));
        let qing_qing = Arc::new(Mutex::new(Student::new("qingqing".to_string(), Gender::MALE, 26)));
        let zhang_san = Arc::new(Mutex::new(Teacher::new("zhangsan".to_string(), Gender::MALE, 26)));
        let li_si = Arc::new(Mutex::new(Teacher::new("lisi".to_string(), Gender::MALE, 29)));
        let wang_wu = Arc::new(Mutex::new(Teacher::new("wangwu".to_string(), Gender::MALE, 30)));
        let class1 = Class::new("1-1".to_string(), zhang_san.clone(), vec![ming_ming.clone(), fang_fang.clone()]);
        let class2 = Class::new("1-2".to_string(), li_si.clone(), vec![xiao_bai.clone(), xiao_hong.clone()]);
        let class3 = Class::new("2-1".to_string(), wang_wu.clone(), vec![wang_hai.clone(), ling_ling.clone()]);
        let class4 = Class::new("2-2".to_string(), zhang_san.clone(), vec![hui_hui.clone(), qing_qing.clone()]);
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
    fn insert_teacher(&mut self, teacher: Teacher);
    fn get_teacher_by_name(&self, name: &str) -> Option<Arc<Mutex<Teacher>>>;
    fn get_all_teachers(&self) -> Vec<Arc<Mutex<Teacher>>>;
    fn contains_teacher(&self, name: &str) -> bool;
    fn insert_student(&mut self, student: Student);
    fn get_student_by_name(&self, name: &str) -> Option<Arc<Mutex<Student>>>;
    fn get_all_students(&self) -> Vec<Arc<Mutex<Student>>>;
    fn contains_student(&self, name: &str) -> bool;
    fn get_all_classes(&self) -> Vec<Arc<Mutex<Class>>>;
    fn insert_class(&mut self, class: Class);
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
        let _: RedisResult<()> = self.client.client.get_connection().unwrap()
            .set(teacher.name(), serde_json::to_string(&teacher).unwrap());
    }

    fn get_teacher_by_name(&self, name: &str) -> Option<Arc<Mutex<Teacher>>> {
        let teacher: String = self.client.client.get_connection().unwrap().get(name).unwrap();
        let teacher: Arc<Mutex<Teacher>> = Arc::new(Mutex::new(serde_json::from_str(teacher.as_str()).unwrap()));
        let teacher = Arc::clone(&teacher);
        Some(teacher)
    }

    fn get_all_teachers(&self) -> Vec<Arc<Mutex<Teacher>>> {
        todo!()
    }

    fn contains_teacher(&self, name: &str) -> bool {
        let result: RedisResult<String> = self.client.client.get_connection().unwrap().get(name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn insert_student(&mut self, student: Student) {
        let _: RedisResult<()> = self.client.client.get_connection().unwrap()
            .set(student.name(), serde_json::to_string(&student).unwrap());
    }

    fn get_student_by_name(&self, name: &str) -> Option<Arc<Mutex<Student>>> {
        let student: String = self.client.client.get_connection().unwrap().get(name).unwrap();
        let student: Arc<Mutex<Student>> = Arc::new(Mutex::new(serde_json::from_str(student.as_str()).unwrap()));
        let student = Arc::clone(&student);
        Some(student)
    }

    fn get_all_students(&self) -> Vec<Arc<Mutex<Student>>> {
        todo!()
    }

    fn contains_student(&self, name: &str) -> bool {
        let result: RedisResult<String> = self.client.client.get_connection().unwrap().get(name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn get_all_classes(&self) -> Vec<Arc<Mutex<Class>>> {
        todo!()
    }

    fn insert_class(&mut self, class: Class) {
        // TODO 待实现
        // let _: RedisResult<()> = self.client.client.get_connection().unwrap()
        //     .set(class.name(), serde_json::to_string(&class).unwrap());
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