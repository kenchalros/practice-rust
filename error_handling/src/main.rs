use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // let res = read_username_from_file();
    let res = read_username_from_file_with_shortcut();
    println!("{:?}", res);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_with_shortcut() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // 下記のようにも書ける
    // let mut s = STring::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
}