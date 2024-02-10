trait Animal {
    fn noise(&self);
}

struct Sheep {
    name: &'static str,
}

impl Sheep {
    pub fn new_sheep() -> Self {
        Self { name: "hello" }
    }
}

impl Duck {
    pub fn new_duck() -> Self {
        Self { name: "world" }
    }
}

impl Animal for Sheep {
    fn noise(&self) {
        println!("hello,I'm sheep");
    }
}

struct Duck {
    name: &'static str,
}

impl Animal for Duck {
    fn noise(&self) {
        println!("hello,I'm duck");
    }
}
fn random_animal(i: i32) -> Box<dyn Animal> {
    if i < 10 {
        Box::new(Sheep::new_sheep())
    } else {
        Box::new(Duck::new_duck())
    }
}

#[test]
pub fn test_sheep() {
    let s: Sheep = Sheep::new_sheep();
    s.noise();

    let sd = random_animal(1);
    sd.noise();
}
