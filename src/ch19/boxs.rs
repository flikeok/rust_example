use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

#[derive(Debug)]
struct Stu<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> Stu<'a> {
    pub fn new(s: &'a str, i: i32) -> Self {
        Self { name: s, age: i }
    }
}

#[test]
fn box_stu_test() {
    let s = Box::new(Stu::new("hello", 32));
    println!("name:{},age:{}", s.name, s.age);
}

#[test]
fn vector_test() {
    let mut v: Vec<i32> = (0..10).collect();
    v.push(23);
    for i in &v {
        println!("i:{}", i);
    }

    println!("v len:{}", v.len());

    for (i, v) in v.iter().enumerate() {
        println!("i:{},v:{}", i, v);
    }

    for i in v.iter_mut() {
        *i = *i + 5;
    }
    for i in &v {
        println!("i:{}", i);
    }
}

#[test]
fn test_hash_map() {
    let mut m = HashMap::new();
    m.insert("k", "v");
    m.insert("k2", "v2");
    m.insert("k3", "v3");
    m.insert("k4", "v4");

    for v in m.iter() {
        println!("i:{},v:{}", *v.0, *v.1);
    }
}

#[test]
fn hash_set_test() {
    let s1: HashSet<i32> = vec![1, 2, 2, 3, 4].iter().map(|&x| x + 1).collect();
    for i in s1.iter() {
        println!("i:{}", i);
    }
}

#[test]
fn rc_test() {
    let s = Rc::new("hello".to_owned());
    println!("s:{}", s);
    println!("s.count:{}", Rc::strong_count(&s));
}

#[test]
fn arc_test() {
    let s = Arc::new("world".to_owned());
    let mut i = 0;
    let mut jv = Vec::new();
    loop {
        let a = s.clone();
        let t = thread::spawn(move || {
            println!("a:{}", a);
        });
        jv.push(t);
        i = i + 1;
        if i > 10 {
            break;
        }
    }

    for i in jv {
        i.join().unwrap();
    }
}

fn a_plus_b(i: i32, j: i32) -> i32 {
    i + j
}

#[test]
fn test_spawn() {
    for i in 0..10 {
        thread::spawn(move || {
            let s = a_plus_b(i, i + 10);
            println!("s:{}", s);
        });
    }
}
