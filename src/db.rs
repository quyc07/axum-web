use std::collections::HashMap;
use std::string::ToString;
use std::sync::{Arc, Mutex};

use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
use crate::err::SchoolErr;

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

const TEACHER: &str = "teacher";
const STUDENT: &str = "student";
const CLASS: &str = "class";


impl Db for RedisDb {
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        self.client.client.get_connection()?
            .hset(TEACHER, teacher.name(), serde_json::to_string(&teacher)?)?;
        Ok(())
    }

    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        let teacher: String = self.client.client.get_connection()?.hget(TEACHER, name)?;
        Ok(Arc::new(Mutex::new(serde_json::from_str(teacher.as_str()).unwrap())))
    }

    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        let teachers: HashMap<String, String> = self.client.client.get_connection()?.hgetall(TEACHER)?;
        Ok(teachers.iter().map(|(k, v)| Arc::new(Mutex::new(serde_json::from_str(v).unwrap()))).collect())
    }

    fn contains_teacher(&self, name: &str) -> bool {
        let result: RedisResult<String> = self.client.client.get_connection().unwrap().hget(TEACHER, name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        self.client.client.get_connection()?
            .hset(STUDENT, student.name(), serde_json::to_string(&student)?)?;
        Ok(())
    }

    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        let redis_string: String = self.client.client.get_connection()?.hget(STUDENT, name)?;
        Ok(Arc::new(Mutex::new(serde_json::from_str(redis_string.as_str()).unwrap())))
    }

    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        let students: HashMap<String, String> = self.client.client.get_connection()?.hgetall(STUDENT)?;
        Ok(students.iter().map(|(k, v)| Arc::new(Mutex::new(serde_json::from_str(v).unwrap()))).collect())
    }

    fn contains_student(&self, name: &str) -> bool {
        let result: RedisResult<String> = self.client.client.get_connection().unwrap().hget(STUDENT, name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        let classes: HashMap<String, String> = self.client.client.get_connection()?.hgetall(CLASS).unwrap();
        let classes: Vec<ClassRedisPo> = classes.iter().map(|(k, v)| serde_json::from_str(v).unwrap()).collect();
        Ok(classes.iter().map(|x| Arc::new(Mutex::new(self.class_redis_po_2_class(x).unwrap()))).collect())
    }

    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        let class_redis_po = ClassRedisPo::from(class);
        self.client.client.get_connection()?
            .hset(CLASS, class_redis_po.name(), serde_json::to_string(&class_redis_po)?)?;
        Ok(())
    }
}

impl RedisDb {
    fn class_redis_po_2_class(&self, x: &ClassRedisPo) -> Result<Class, SchoolErr> {
        Ok(Class {
            name: x.name().to_string(),
            teacher: self.get_teacher_by_name(x.teacher_name.as_str())?,
            students: x.students_name.iter().map(|s| self.get_student_by_name(s).unwrap()).collect(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct ClassRedisPo {
    name: String,
    teacher_name: String,
    students_name: Vec<String>,
}

impl ClassRedisPo {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl From<Class> for ClassRedisPo {
    fn from(class: Class) -> Self {
        ClassRedisPo {
            name: class.name().to_string(),
            teacher_name: class.teacher().lock().unwrap().name().to_string(),
            students_name: class.students().iter().map(|x| x.lock().unwrap().name().to_string()).collect(),
        }
    }
}

pub struct Client {
    client: redis::Client,
}

impl Client {
    fn new() -> Client {
        // connect to redis
        let client = redis::Client::open(("redis", 6379)).unwrap();
        Client { client }
    }
}