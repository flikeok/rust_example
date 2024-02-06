use std::fmt::format;
use std::str::FromStr;

#[derive(Debug)]
pub struct Stu {
    pub name: String,
}

impl ToString for Stu {
    fn to_string(&self) -> String {
        format!("my name is {}", self.name)
    }
}

impl FromStr for Stu {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stu {
            name: s.to_string(),
        })
    }
}

#[test]
fn test_to_string() {
    let s: Stu = Stu {
        name: "hello".to_string(),
    };
    println!("stu:{}", s.to_string());
}

#[test]
fn test_from_string() {
    let s: String = "hello".to_string();
    let stu: Result<Stu, ()> = s.parse::<Stu>();
    match stu {
        Ok(s) => println!("stu:{:?}", s),
        Err(e) => println!("err:{:?}", e),
    }
}
