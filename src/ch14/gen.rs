pub struct Stu<T> {
    pub name: T,
}

impl<T> Stu<T> {
    pub fn new(name: T) -> Self {
        Self { name: name }
    }
}

#[test]
pub fn stu_test() {
    let stu = Stu::new("hello");
    println!("stu.name:{}", stu.name);
}
