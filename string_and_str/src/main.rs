fn main() {
    // 新しい空のStringを生成する
    let mut s = String::new();
    println!("{}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    let s = "initial contents".to_string();
    println!("{}", s);

    // to_stringを使うものと同じ
    let s = String::from("initial contents");
    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // push_strは所有権を奪わない
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // 所有権取られてエラーになりそうだが、そうはならない

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はmoveされ、使用できなくなることに注意
    println!("s3 is '{}'", s3);

    // 複数の文字列連結では + は扱いにくい
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is '{}'", s);

    // format! マクロを使用すると簡単。（所有権も奪わない）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is '{}'", s);
}
