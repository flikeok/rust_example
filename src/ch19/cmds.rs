use std::process::Command;

#[test]
fn test_cmd() {
    let output = Command::new("echo").arg("hello").output().unwrap();
    let h = output.stdout;
    println!("h:{}", String::from_utf8(h).unwrap());
}
