use std::sync::{Arc, Mutex};

use mysql::{Error, FromValueError, params, Pool, Row, Value};
use mysql::prelude::{FromValue, Queryable};

use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};

pub struct MysqlDb {
    pool: Pool,
}

impl Default for MysqlDb {
    fn default() -> Self {
        let url = "mysql://root:abc123@mysql:3306/mydb";
        MysqlDb {
            pool: Pool::new(url).unwrap()
        }
    }
}

impl Db for MysqlDb {
    fn insert_teacher(&mut self, teacher: Teacher) -> Result<(), SchoolErr> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop("insert into teacher(name,gender,age) values(:name,:gender,:age)", params! {
            "name"=>teacher.name(),
            "gender"=>teacher.gender().name(),
            "age"=>teacher.age()
        })?;
        Ok(())
    }

    fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        let mut conn = self.pool.get_conn()?;
        let x: Row = conn.exec_first("select * from teacher where name = :name", params! {
            "name"=>name
        })?.unwrap();
        Ok(Arc::new(Mutex::new(Teacher::new(x.get(0).unwrap(), x.get(1).unwrap(), x.get(2).unwrap()))))
    }

    fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        let mut conn = self.pool.get_conn().unwrap();
        let result: Result<Vec<Row>, Error> = conn.query("select * from teacher");
        if let Ok(vec) = result {
            return Ok(vec.iter().map(|x| Arc::new(Mutex::new(Teacher::new(x.get(0).unwrap(), x.get(1).unwrap(), x.get(2).unwrap()))))
                .collect());
        }
        Err(SchoolErr::NotFound)
    }

    fn contains_teacher(&self, name: &str) -> bool {
        let mut conn = self.pool.get_conn().unwrap();
        let result: Result<Option<Row>, Error> = conn.exec_first("select 1 from teacher where name = :name", params! {"name"=>name});
        if let Ok(Some(r)) = result {
            return r.len() >= 1;
        }
        false
    }

    fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop("insert into student(name,gender,age) values(:name,:gender,:age)", params! {
            "name"=>student.name(),
            "gender"=>student.gender().name(),
            "age"=>student.age()
        })?;
        Ok(())
    }

    fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        let mut conn = self.pool.get_conn()?;
        let x: Row = conn.exec_first("select * from student where name = :name", params! {
            "name"=>name
        })?.unwrap();
        Ok(Arc::new(Mutex::new(Student::new(x.get(0).unwrap(), x.get(1).unwrap(), x.get(2).unwrap()))))
    }

    fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        let mut conn = self.pool.get_conn().unwrap();
        let result: Result<Vec<Row>, Error> = conn.query("select * from student");
        if let Ok(vec) = result {
            return Ok(vec.iter().map(|x| Arc::new(Mutex::new(Student::new(x.get(0).unwrap(), x.get(1).unwrap(), x.get(2).unwrap()))))
                .collect());
        }
        Err(SchoolErr::NotFound)
    }

    fn contains_student(&self, name: &str) -> bool {
        let mut conn = self.pool.get_conn().unwrap();
        let result: Result<Option<Row>, Error> = conn.exec_first("select 1 from student where name = :name", params! {"name"=>name});
        if let Ok(Some(r)) = result {
            return r.len() >= 1;
        }
        false
    }

    fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        todo!()
    }

    fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop("insert into class(name,teacher,students) values(:name,:teacher,:students)", params! {
            "name"=>class.name(),
            "teacher"=>class.teacher().lock().unwrap().name(),
            "students"=>class.students().iter().map(|x| x.lock().unwrap().name().to_string()).collect::<Vec<String>>().join(","),
        })?;
        Ok(())
    }
}

impl FromValue for Gender {
    type Intermediate = String;

    fn from_value(v: Value) -> Self {
        match v {
            Value::Bytes(vec) => String::from_utf8(vec).unwrap().into(),
            _ => panic!("fail to from value to gender"),
        }
    }

    fn from_value_opt(v: Value) -> Result<Self, FromValueError> {
        match v {
            Value::Bytes(vec) => Ok(String::from_utf8(vec).unwrap().into()),
            _ => Err(FromValueError(v))
        }
    }

    fn get_intermediate(v: Value) -> Result<Self::Intermediate, FromValueError> {
        match v {
            Value::Bytes(vec) => Ok(String::from_utf8(vec).unwrap()),
            _ => Err(FromValueError(v)),
        }
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::MALE,
            "Female" => Gender::FEMALE,
            _ => panic!("fail to from String to Gender")
        }
    }
}