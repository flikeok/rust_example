use std::convert::TryFrom;

#[derive(Debug)]
pub struct Stu {
    pub name: String,
}

impl TryFrom<String> for Stu {
    type Error = ();
    fn try_from(n: String) -> std::result::Result<Self, Self::Error> {
        if n.len() < 5 {
            Ok(Self { name: n })
        } else {
            Err(())
        }
    }
}

#[test]
fn from_test() {
    let s: String = "hello".to_string();
    let stu = Stu::try_from(s);
    println!("stu:{:?}", stu);
}

#[test]
fn into_test() {
    let s: String = "hello,world".to_string();
    let stu: std::result::Result<Stu, ()> = s.try_into();
    match stu {
        Ok(s) => println!("stu:{:?}", s),
        Err(e) => println!("err:{:?}", e),
    }
}
