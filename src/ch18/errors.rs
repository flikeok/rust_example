use std::str::FromStr;

#[test]
pub fn error_handle_test() {
    let s = test_error(100);
    if let Some(i) = s {
        println!("i:{}", i);
    } else {
        println!("i is None");
    }

    let s2 = test_error(2);
    assert_eq!(s2.expect("i large than 10"), 2);
}

pub fn test_error(i: i32) -> Option<i32> {
    if i < 10 {
        return Some(i);
    }
    return None;
}

pub fn option_string(p: Option<String>) -> Option<String> {
    let s = p?;
    println!("s:{}", s);
    Some(s)
}

#[test]
fn option_test() {
    let s = option_string(Some(String::from("Hello,world")));
    println!("s:{}", s.unwrap());
}
