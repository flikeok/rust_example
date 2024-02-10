fn map_string(s: Option<String>) -> Option<usize> {
    s.map(|s| s.len())
}
#[test]
fn test_option_map() {
    let l = map_string(Some(String::from("world")));
    match l {
        Some(i) => println!("len:{}", i),
        None => println!("None"),
    }

    let l2 = map_string(None);
    match l2 {
        Some(i) => println!("len:{}", i),
        None => println!("None"),
    }
}
