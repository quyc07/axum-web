use crate::DbState;

pub struct Class<'a> {
    name: String,
    teacher: &'a Teacher,
    students: Vec<&'a Student>,
}

pub struct Student {
    name: String,
    gender: Gender,
    age: u8,
}

pub struct Teacher {
    name: String,
    gender: Gender,
    age: u8,
}

pub enum Gender {
    MALE,
    FEMALE,
}

impl<'a> Class<'a> {
    fn new(name: String, teacher: &'a Teacher) -> Class<'a> {
        Class {
            name,
            teacher,
            students: Vec::new(),
        }
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
    fn new(name: String, gender: Gender, age: u8) -> Student {
        Student {
            name,
            gender,
            age,
        }
    }
}

pub(crate) async fn init(shared_state: &DbState) {
    let ming_ming = Teacher::new("mingming".to_string(), Gender::MALE, 23);
    let fang_fang = Teacher::new("fangfang".to_string(), Gender::FEMALE, 22);
    let xiao_hong = Teacher::new("xiaohong".to_string(), Gender::FEMALE, 26);
    // let class1 = Class::new("1-1".to_string(), &ming_ming);
    // let class2 = Class::new("1-2".to_string(), &fang_fang);
    // let class3 = Class::new("2-1".to_string(), &xiao_hong);
    // let class4 = Class::new("2-2".to_string(), &ming_ming);
    // shared_state.write().unwrap().add_class(class1);
    // shared_state.write().unwrap().add_class(class2);
    // shared_state.write().unwrap().add_class(class3);
    // shared_state.write().unwrap().add_class(class4);
    shared_state.write().unwrap().add_teacher(ming_ming);
    shared_state.write().unwrap().add_teacher(fang_fang);
    shared_state.write().unwrap().add_teacher(xiao_hong);
}