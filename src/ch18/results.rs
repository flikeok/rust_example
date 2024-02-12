use std::num::ParseIntError;

type ParseIntResult<T> = Result<T, ParseIntError>;
fn two_plus(l: String, r: String) -> ParseIntResult<i32> {
    let li = l.parse::<i32>()?;
    let ri = r.parse::<i32>()?;
    Ok(li + ri)
}

#[test]
fn test_two_plus() {
    let r = two_plus("1".to_owned(), "100".to_owned());
    match r {
        Ok(r) => println!("result:{}", r),
        Err(e) => println!("Err:{:?}", e),
    }

    let r = two_plus("1tt".to_owned(), "100".to_owned());
    match r {
        Ok(r) => println!("result:{}", r),
        Err(e) => println!("Err:{:?}", e),
    }
}

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
}

#[test]
fn test_double_first() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first(numbers));

    println!("The first doubled is {:?}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {:?}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}


