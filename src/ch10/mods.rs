mod my_mod {
    fn hello() {
        println!("Hello,world!");
    }
    pub fn pub_hello() {
        println!("Hello,world!");
    }

    pub fn indirect_access() {
        println!("call private hello in indirect_access");
        hello();
    }

    pub mod nested {
        fn hello() {
            println!("Hello,world!");
        }
        pub(in crate::ch10::mods::my_mod) fn pub_hello() {
            println!("Hello,world in nested::pub_hello!");
        }

        pub(self) fn self_hello() {
            println!("Hello,world in nested::self_hello!");
        }

        pub(super) fn super_hello() {
            println!("Hello,world in nested::super_hello!");
        }

        pub fn call_super_in_nested() {
            println!("call_super_in_nested");
            super_hello();
        }
        pub(crate) fn call_crate_hello() {
            println!("call_crate_hello");
        }
    }
}

#[test]
fn test_mods() {
    my_mod::pub_hello();
    my_mod::nested::call_super_in_nested();
    my_mod::nested::call_crate_hello();
}
