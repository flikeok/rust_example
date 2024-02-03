
#[derive(Debug)]
pub struct Point{
    pub a:i32,
    pub b:i32,
    pub c:String,
}

#[derive(Debug)]
pub struct Stu(i32,String);

pub struct Unit;

#[test]
fn struct_print(){
    let p = Point{a:12,b:15,c:"world".to_string()};
    println!("p:{:?}",p);
    
    let s =Stu(13,"hello".to_string());
    println!("s:{:?}",s);

    let d = Point{a:20,..p};
    println!("d:{:?}",d);
    println!("a:{},b:{},c:{}",d.a,d.b,d.c);

    let Stu(m,n) = s;
    println!("m:{},n:{}",m,n);
}