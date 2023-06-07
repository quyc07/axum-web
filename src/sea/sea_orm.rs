use axum::extract::FromRef;
use std::sync::{Arc, Mutex};

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, DbErr, EntityTrait, NotSet,
    QueryFilter,
};
use tokio::task::JoinHandle;

use crate::async_db::AsyncDb;
use crate::err::SchoolErr;
use crate::school::{Class, Gender, Student, Teacher};
use crate::sea;
use crate::sea::{class, student, teacher};

#[derive(Clone)]
pub struct SeaOrm {
    db: DatabaseConnection,
}

impl SeaOrm {
    pub async fn new() -> Result<SeaOrm, DbErr> {
        let db: DatabaseConnection =
            Database::connect("mysql://root:abc123@127.0.0.1/mydb").await?;
        Ok(SeaOrm { db })
    }
}

async fn class_model_2_class(class: class::Model, db: &DatabaseConnection) -> Arc<Mutex<Class>> {
    let teacher = teacher::Entity::find()
        .filter(teacher::Column::Name.eq(class.teacher.unwrap()))
        .one(db)
        .await
        .unwrap()
        .unwrap();
    let students_name = class.students.unwrap();
    // let students_name = students_name.split(",").collect::<Vec<&str>>();
    let students = student::Entity::find()
        .filter(student::Column::Name.is_in(students_name.split(",")))
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|x| Arc::new(Mutex::new(x.into())))
        .collect();
    Arc::new(Mutex::new(Class::new(
        class.name.to_string(),
        Arc::new(Mutex::new(teacher.into())),
        students,
    )))
}

#[tonic::async_trait]
impl AsyncDb for SeaOrm {
    async fn insert_teacher(&mut self, teacher: Teacher) -> Result<Teacher, SchoolErr> {
        let teacher: teacher::Model = teacher::Model::from(teacher);
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
        let db = Arc::new(DatabaseConnection::from_ref(&self.db));
        // 查询所有班级
        let class_vec = class::Entity::find().all(&*db).await.unwrap();
        // 拷贝出与班级数相同的链接
        let db_vec = (0..class_vec.len())
            .into_iter()
            .map(|_| Arc::clone(&db))
            .collect::<Vec<Arc<DatabaseConnection>>>();
        // 构建 class - 链接 的映射
        let class_2_db = class_vec.into_iter().zip(db_vec.into_iter());
        // 便利并查询每个班级的老师和学生
        let handles = class_2_db
            .into_iter()
            .map(|x| tokio::task::spawn(async move { class_model_2_class(x.0, &x.1).await }))
            .collect::<Vec<JoinHandle<Arc<Mutex<Class>>>>>();
        let mut classes = vec![];
        for x in handles {
            classes.push(x.await.unwrap());
        }
        Ok(classes)
    }

    async fn insert_class(&mut self, class: Class) -> Result<(), SchoolErr> {
        let class = class::Model::from(class);
        let class_act = class::ActiveModel {
            id: NotSet,
            name: Set(class.name),
            teacher: Set(class.teacher),
            students: Set(class.students),
        };
        class_act.insert(&self.db).await?;
        Ok(())
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

impl From<Class> for class::Model {
    fn from(value: Class) -> Self {
        class::Model {
            id: Default::default(),
            name: value.name().to_string(),
            teacher: Some(value.teacher().lock().unwrap().name().to_string()),
            students: Some(
                value
                    .students()
                    .iter()
                    .map(|x| x.lock().unwrap().name().to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
        }
    }
}
