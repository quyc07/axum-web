use std::sync::{Arc, Mutex};
use std::time::Duration;

use mysql::{Error, FromValueError, params, Pool, Row, Value, OptsBuilder, PoolOpts, Conn, Opts, PoolConstraints};
use mysql::prelude::{FromValue, Queryable};

use crate::db::Db;
use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};

pub struct MysqlDb {
    pool: Pool,
}

impl Default for MysqlDb {
    fn default() -> Self {
        MysqlDb::new().unwrap()
    }
}

impl MysqlDb {
    fn new() -> Result<Self, Error> {
        let builder = OptsBuilder::new()
            .ip_or_hostname(Some("127.0.0.1"))
            .user(Some("root"))
            .pass(Some("abc123"))
            .db_name(Some("mydb"))
            .tcp_connect_timeout(Some(std::time::Duration::from_secs(5)))// 设置连接超时时间为 5 秒
            .read_timeout(Some(Duration::from_secs(10)))// 读取数据超时时间
            .pool_opts(PoolOpts::new().with_constraints(PoolConstraints::new(5, 10).unwrap()))// 连接池设置
            ;
        Ok(MysqlDb { pool: Pool::new(builder)? })
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