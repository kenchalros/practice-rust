fn main() {
    slice_fn_example();
    str_signature();
}

fn slice_fn_example() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // コンパイルエラーになる
    // println!("word: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if  item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn str_signature() {
    let my_string = String::from("hello world");

    // first_word_ver2は`String`のスライスに対して機能する
    let word = first_word_ver2(&my_string[..]);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word_ver2(&my_string_literal[..]);

    // 文字列リテラルは、すでに文字列スライスであるため、
    // スライス記法なしでも機能する
    let word = first_word_ver2(my_string_literal);
}

fn first_word_ver2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if  item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}