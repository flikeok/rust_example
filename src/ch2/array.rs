
#[test]
fn array_fn(){
    let a = [1,2,3];
    println!("a:{:?}",a);
    println!("a len:{}",a.len());

    slice(&a[1..2]);
    slice(&a);
}

fn slice(s:&[i32]){
    println!("s:{:?}",s);
    println!("s last:{}",s.last().unwrap());
    println!("s size:{}",s.len());
}