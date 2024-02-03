static MY_WORLD:&'static str = "hello";
const NUM:i32 = 10;

#[test]
fn const_test(){
    println!("{},{}",MY_WORLD,NUM);
}