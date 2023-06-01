use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::{Class, Student, Teacher};
use redis::{Commands, RedisResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct RedisDb {
    client: Client,
}

impl Default for RedisDb {
    fn default() -> Self {
        RedisDb {
            client: Client::new(),
        }
    }
}

const TEACHER: &str = "teacher";
const STUDENT: &str = "student";
const CLASS: &str = "class";

impl Db for RedisDb {
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        self.client.client.get_connection()?.hset(
            TEACHER,
            teacher.name(),
            serde_json::to_string(&teacher)?,
        )?;
        Ok(())
    }

    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        let teacher: String = self.client.client.get_connection()?.hget(TEACHER, name)?;
        Ok(Arc::new(Mutex::new(
            serde_json::from_str(teacher.as_str()).unwrap(),
        )))
    }

    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        let teachers: HashMap<String, String> =
            self.client.client.get_connection()?.hgetall(TEACHER)?;
        Ok(teachers
            .iter()
            .map(|(k, v)| Arc::new(Mutex::new(serde_json::from_str(v).unwrap())))
            .collect())
    }

    fn contains_teacher(&self, name: &str) -> bool {
        let result: RedisResult<String> = self
            .client
            .client
            .get_connection()
            .unwrap()
            .hget(TEACHER, name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        self.client.client.get_connection()?.hset(
            STUDENT,
            student.name(),
            serde_json::to_string(&student)?,
        )?;
        Ok(())
    }

    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        let redis_string: String = self.client.client.get_connection()?.hget(STUDENT, name)?;
        Ok(Arc::new(Mutex::new(
            serde_json::from_str(redis_string.as_str()).unwrap(),
        )))
    }

    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        let students: HashMap<String, String> =
            self.client.client.get_connection()?.hgetall(STUDENT)?;
        Ok(students
            .iter()
            .map(|(k, v)| Arc::new(Mutex::new(serde_json::from_str(v).unwrap())))
            .collect())
    }

    fn contains_student(&self, name: &str) -> bool {
        let result: RedisResult<String> = self
            .client
            .client
            .get_connection()
            .unwrap()
            .hget(STUDENT, name);
        if let Ok(_) = result {
            return true;
        }
        false
    }

    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        let classes: HashMap<String, String> =
            self.client.client.get_connection()?.hgetall(CLASS).unwrap();
        let classes: Vec<ClassRedisPo> = classes
            .iter()
            .map(|(k, v)| serde_json::from_str(v).unwrap())
            .collect();
        Ok(classes
            .iter()
            .map(|x| Arc::new(Mutex::new(self.class_redis_po_2_class(x).unwrap())))
            .collect())
    }

    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        let class_redis_po = ClassRedisPo::from(class);
        self.client.client.get_connection()?.hset(
            CLASS,
            class_redis_po.name(),
            serde_json::to_string(&class_redis_po)?,
        )?;
        Ok(())
    }
}

impl RedisDb {
    fn class_redis_po_2_class(&self, x: &ClassRedisPo) -> Result<Class, SchoolErr> {
        Ok(Class::new(
            x.name().to_string(),
            self.get_teacher_by_name(x.teacher_name.as_str())?,
            x.students_name
                .iter()
                .map(|s| self.get_student_by_name(s).unwrap())
                .collect(),
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ClassRedisPo {
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
            students_name: class
                .students()
                .iter()
                .map(|x| x.lock().unwrap().name().to_string())
                .collect(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::school::{Class, Gender, Student, Teacher};
    use redis::Commands;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_class_redis_po_from_class() {
        let teacher = Arc::new(Mutex::new(Teacher::new(
            "John".to_string(),
            Gender::MALE,
            30,
        )));

        let student1 = Arc::new(Mutex::new(Student::new(
            "Alice".to_string(),
            Gender::FEMALE,
            18,
        )));
        let student2 = Arc::new(Mutex::new(Student::new(
            "Bob".to_string(),
            Gender::MALE,
            19,
        )));
        let class = Class::new(
            "Math".to_owned(),
            teacher.clone(),
            vec![student1.clone(), student2.clone()],
        );
        let class_redis_po = ClassRedisPo::from(class);
        assert_eq!(class_redis_po.name, "Math");
        assert_eq!(class_redis_po.teacher_name, "John");
        assert_eq!(class_redis_po.students_name, vec!["Alice", "Bob"]);
    }
}
