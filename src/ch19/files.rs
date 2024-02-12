use std::{
    fs::{remove_file, File},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::FileExt,
    path::Path,
    str::from_utf8,
};

#[test]
fn file_rw_test() {
    let p = Path::new("/tmp/hello.txt");
    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .append(true)
        .open(p)
        .expect("file open fail");

    //write string
    let r = file.write("hello,world".as_bytes()).unwrap();
    println!("write {} bytes in file", r);

    //read
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("read:{},size:{}", s, s.len());

    //delete file
    remove_file(p).unwrap();
}
