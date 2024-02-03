use core::fmt;


fn reverse(left:i32,right:bool)->(bool,i32){
    (right,left)
}

#[test]
fn print_tuple(){
    let a = (10,false);
    let b = reverse(a.0,a.1);
    println!("a:{:?}",a);
    println!("b:{:?}",b);

    println!("single tuple:{:?}",(5,));
    println!("single int:{:?}",(5));

    let (c,d) = a;
    println!("c:{},d:{}",c,d);

}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f,"({},{})",self.0,self.1)?;
        writeln!(f,"({},{})",self.2,self.3)
    }
}

impl Matrix {
    //在Rust中，Self通常用于实现trait方法，
    //以及在结构体或枚举的实现中。这样可以确保方法能够正确地引用当前实例，而不必担心实例的具体名称
    fn trans(&self)->Self{
        Matrix(self.0,self.2,self.1,self.3)
    }
}


#[test]
fn translate(){
    let mut m = Matrix(1.1,1.2,2.1,2.2);
    println!("befor translate m:\n{}",m);
    m = m.trans();
    println!("after translate m:\n{}",m);
}