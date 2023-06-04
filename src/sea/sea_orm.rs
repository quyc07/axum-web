use std::sync::{Arc, Mutex};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, NotSet,
    QueryFilter,
};
use sea_orm::ActiveValue::Set;

use crate::async_db::AsyncDb;
use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};
use crate::sea;
use crate::sea::{student, teacher};

#[derive(Clone)]
pub struct SeaOrm {
    db: DatabaseConnection,
}

impl SeaOrm {
    pub async fn new() -> Result<SeaOrm, DbErr> {
        let db: DatabaseConnection =
            Database::connect("mysql://root:abc123@localhost/mydb").await?;
        Ok(SeaOrm { db })
    }
}

#[tonic::async_trait]
impl AsyncDb for SeaOrm {
    async fn insert_teacher(&mut self, teacher: teacher::Model) -> Result<Teacher, SchoolErr> {
        let teacher_act = teacher::ActiveModel {
            id: NotSet,
            name: Set(teacher.name),
            gender: Set(teacher.gender),
            age: Set(teacher.age),
        };
        let model = teacher_act.insert(&self.db).await?;
        Ok(model.into())
    }

    async fn get_teacher_by_name(&self, name: &str) -> Result<Arc<Mutex<Teacher>>, SchoolErr> {
        Ok(Arc::new(Mutex::new(
            teacher::Entity::find()
                .filter(teacher::Column::Name.eq(name))
                .one(&self.db)
                .await?
                .unwrap()
                .into(),
        )))
    }

    async fn get_all_teachers(&self) -> Result<Vec<Arc<Mutex<Teacher>>>, SchoolErr> {
        Ok(teacher::Entity::find()
            .all(&self.db)
            .await
            .unwrap()
            .iter()
            .map(|x| Arc::new(Mutex::new(x.to_owned().into())))
            .collect())
    }

    async fn contains_teacher(&self, name: &str) -> bool {
        teacher::Entity::find()
            .filter(teacher::Column::Name.eq(name))
            .one(&self.db)
            .await
            .unwrap()
            .is_some()
    }

    async fn insert_student(&mut self, student: Student) -> Result<(), SchoolErr> {
        let student: student::Model = Student::into(student);
        let student_act = student::ActiveModel {
            id: NotSet,
            name: Set(student.name),
            gender: Set(student.gender),
            age: Set(student.age),
        };
        let model = student_act.insert(&self.db).await?;
        Ok(())
    }

    async fn get_student_by_name(&self, name: &str) -> Result<Arc<Mutex<Student>>, SchoolErr> {
        Ok(Arc::new(Mutex::new(
            student::Entity::find()
                .filter(student::Column::Name.eq(name))
                .one(&self.db)
                .await?
                .unwrap()
                .into(),
        )))
    }

    async fn get_all_students(&self) -> Result<Vec<Arc<Mutex<Student>>>, SchoolErr> {
        Ok(student::Entity::find()
            .all(&self.db)
            .await
            .unwrap()
            .iter()
            .map(|x| Arc::new(Mutex::new(x.to_owned().into())))
            .collect())
    }

    async fn contains_student(&self, name: &str) -> bool {
        student::Entity::find()
            .filter(student::Column::Name.eq(name))
            .one(&self.db)
            .await
            .unwrap()
            .is_some()
    }

    async fn get_all_classes(&self) -> Result<Vec<Arc<Mutex<Class>>>, SchoolErr> {
        todo!()
    }

    async fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        todo!()
    }
}

impl From<Teacher> for teacher::Model {
    fn from(value: Teacher) -> Self {
        teacher::Model {
            id: Default::default(),
            name: value.name().to_string(),
            gender: Some(value.gender().name()),
            age: Some(value.age() as i32),
        }
    }
}

impl From<teacher::Model> for Teacher {
    fn from(value: teacher::Model) -> Self {
        Teacher::new(
            value.name,
            Gender::from(value.gender.unwrap()),
            value.age.unwrap() as u8,
        )
    }
}
impl From<Student> for student::Model {
    fn from(value: Student) -> Self {
        student::Model {
            id: Default::default(),
            name: value.name().to_string(),
            gender: Some(value.gender().name()),
            age: Some(value.age() as i32),
        }
    }
}

impl From<student::Model> for Student {
    fn from(value: student::Model) -> Self {
        Student::new(
            value.name,
            Gender::from(value.gender.unwrap()),
            value.age.unwrap() as u8,
        )
    }
}
