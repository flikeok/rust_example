use std::convert::From;

#[derive(Debug)]
pub struct Stu {
    pub name: String,
}

impl From<String> for Stu {
    fn from(n: String) -> Stu {
        return Stu { name: n };
    }
}

#[test]
fn from_test() {
    let s: String = "hello".to_string();
    let stu = Stu::from(s);
    println!("stu:{:?}", stu);
}

#[test]
fn into_test() {
    let s: String = "hello".to_string();
    let stu: Stu = s.into();
    println!("stu:{:?}", stu);
}
