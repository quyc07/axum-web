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

enum Gender {
    MALE,
    FEMALE,
}

impl Class<'_> {
    fn new(name: String, teacher: &Teacher) -> Class {
        Class {
            name,
            teacher,
            students: Vec::new(),
        }
    }
}

impl Teacher {
    fn new(name: String, gender: Gender, age: u8) -> Teacher {
        Teacher {
            name,
            gender,
            age,
        }
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

pub(crate) async fn init(mut shared_state: DbState) {
    let ming_ming = Teacher::new("mingming".to_string(), Gender::MALE, 23);
    let fang_fang = Teacher::new("fangfang".to_string(), Gender::FEMALE, 22);
    let xiao_hong = Teacher::new("xiaohong".to_string(), Gender::FEMALE, 26);
    let class1 = Class::new("1-1".to_string(), &ming_ming);
    let class2 = Class::new("1-2".to_string(), &fang_fang);
    let class3 = Class::new("2-1".to_string(), &xiao_hong);
    let class4 = Class::new("2-2".to_string(), &ming_ming);
    shared_state.write().unwrap().add_teacher(&ming_ming);
    shared_state.write().unwrap().add_teacher(&fang_fang);
    shared_state.write().unwrap().add_teacher(&xiao_hong);
    shared_state.write().unwrap().add_class(&class1);
    shared_state.write().unwrap().add_class(&class2);
    shared_state.write().unwrap().add_class(&class3);
    shared_state.write().unwrap().add_class(&class4);
    println!("{}", shared_state.read().unwrap().next_class_id());
}