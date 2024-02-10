use self::my::{CloseBox, OpenBox};

mod my {
    pub struct OpenBox<T> {
        pub content: T,
    }

    pub struct CloseBox<T> {
        content: T,
    }

    impl<T> CloseBox<T>
    where
        T: std::fmt::Display,
    {
        pub fn new(t: T) -> Self {
            Self { content: t }
        }

        pub fn print(&self) {
            println!("t:{}", self.content);
        }
    }
}

#[test]
pub fn mod_struct_test() {
    let b = OpenBox {
        content: "hello".to_string(),
    };
    println!("content in OpenBox:{}", b.content);

    let cb = CloseBox::new("t");
    cb.print();
}
