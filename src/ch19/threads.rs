use std::{ffi::OsStr, path::Path, sync::mpsc, thread};

#[test]
fn thread_test() {
    let (sx, rx) = mpsc::channel::<i32>();
    for i in 0..10 {
        let local_sx = sx.clone();
        thread::spawn(move || {
            local_sx.send(i * 10).unwrap();
        });
    }

    let mut ids = Vec::new();
    for j in 0..10 {
        if let Ok(v) = rx.recv() {
            ids.push(v);
        }
    }
    println!("ids:{:?}", ids);
}

#[test]
fn path_test() {
    let p = Path::new("/usr/local/flike/a.txt");
    let dir = p.parent();
    assert_eq!(dir, Some(Path::new("/usr/local/flike")));

    let file = p.file_name();
    assert_eq!(Some(OsStr::new("a.txt")), file);

    println!("path:{}", p.display());

    let new_path = dir.expect("dir is none").join("hello");
    println!("new path:{}", new_path.display());

    if let Some(s) = new_path.to_str() {
        println!("s:{}", s);
    }
}
