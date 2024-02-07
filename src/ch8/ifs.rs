#[test]
fn statement() {
    let mut a = 3;
    if a > 3 {
        println!("a is large than 3");
    } else {
        println!("a is little than 3");
    }

    let b = loop {
        a = a + 1;
        if a > 10 {
            break a + 10;
        }
    };
    println!("a:{},b:{}", a, b);
}

#[test]
fn while_test() {
    let mut a = 10;
    while a < 20 {
        a = a + 1;
    }
    println!("a:{}", a);

    for i in 0..a {
        println!("i:{}", i)
    }

    for i in 0..=a {
        println!("i:{}", i)
    }

    let v = vec![3, 4, 9, 8];
    for item in v.iter() {
        println!("item:{}", item);
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Yellow,
}

#[test]
fn match_test() {
    let a = 13;
    match a {
        1..=10 => println!("a < 10"),
        b => println!("b:{}", b),
    }

    let t = (10, 'a');
    match t {
        (11, a) => println!("a:{}", a),
        (10, a) => println!("t:{}", a),
        (c, d) => println!("c:{},d:{}", c, d),
    }

    let mut r = 5;
    match r {
        ref mut a => *a = *a + 1,
    }
    println!("r:{}", r);

    let ref mut cc = 10;
    let dd = match cc {
        value => {
            *value = 100;
            *value
        }
        _ => 0,
    };
    println!("dd:{}", dd);
}

pub struct Stu {
    name: String,
    age: i32,
}
#[test]
fn struct_test() {
    let s = Stu {
        name: "hello".to_string(),
        age: 10,
    };
    let Stu { name, .. } = s;

    println!("name:{}", name);
}

#[test]
fn if_guard_test() {
    let s = (1, 3);
    match s {
        (x, y) if x < y => println!("x:{},y:{}", x, y),
        (x, y) if x + y == 4 => println!("x+y={}", x + y),
        _ => println!("don't care"),
    }
}

#[test]
fn if_let_test() {
    let s = Some(7);
    if let Some(i) = s {
        println!("i is {}", i);
    }

    let y: Option<i32> = None;
    if let Some(j) = y {
        println!("j is {}", j);
    } else {
        println!("j is None");
    }
}

#[test]
fn test_while_let() {
    let mut s = Some(1);
    //一定要Some在前
    while let Some(mut i) = s {
        println!("i is {}", i);
        i = i + 1;
        if i > 10 {
            s = None;
        } else {
            s = Some(i);
        }
    }
}
