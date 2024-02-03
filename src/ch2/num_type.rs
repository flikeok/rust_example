
#[test]
fn ex01(){
    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mut_able = 12; // Mutable `i32`
    mut_able = 21;

    println!("mutable:{},inferred_type:{}",mut_able,inferred_type);
}