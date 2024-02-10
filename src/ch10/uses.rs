mod my {
    pub fn hello() {
        println!("hello,world");
    }
    pub fn world() {
        println!("hw");
    }

    mod his {
        pub fn hello() {
            println!("hello in his");
        }
        pub fn call_hello() {
            self::hello();
            super::hello();
        }
    }
}

use my::hello;
#[test]
pub fn test_use() {
    hello();
}

#[test]
pub fn test_world() {
    use my::world;
    world();
}

#[allow(dead_code)]
pub fn hello_dead() {
    println!("hello,world");
}
