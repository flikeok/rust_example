#![allow(dead_code)]

pub enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i32,y:i32},
}

fn inpect(e:WebEvent){
    match e {
        WebEvent::PageLoad=> println!("Page Load"),
        WebEvent::PageUnload=> println!("Page Unload"),
        WebEvent::KeyPress(c)=>println!("key press,char:{}",c),
        WebEvent::Paste(s)=>println!("paste string:{}",s),
        WebEvent::Click { x, y }=>println!("click x:{},y:{}",x,y)
    }
}

#[test]
fn enum_test(){
    inpect(WebEvent::PageLoad);
    inpect(WebEvent::PageUnload);
    inpect(WebEvent::KeyPress('H'));
    inpect(WebEvent::Paste(String::from("Hello")));
    inpect(WebEvent::Click {x: 15, y: 40 });
}

#[test]
fn enum_test2(){
    use WebEvent::*;
    inpect(PageLoad);
    inpect(PageUnload);
    inpect(KeyPress('H'));
    inpect(Paste(String::from("Hello")));
    inpect(Click {x: 15, y: 40 });
}

#[derive(Debug)]
enum Color{
    Red = 125,
    Yellow = 12,
    Green = 56,
}

#[test]
fn test_color(){
    let a = Color::Red;
    println!("a:{:?}",a);
}