enum SpredsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("[vector_sample]");
    vector_sample();

    println!("[scan_vectore]");
    scan_vectore();

    println!("[enum vector]")
    enum_vector();
}

fn vector_sample() {
    // Vecの宣言
    // let v: Vec<i32> = Vec::new();

    // マクロを使うこともできる
    // let v = vec![1,2,3];

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // v.push(9);

    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("{:?}", third);
    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);
}

fn scan_vectore() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

fn enum_vector() {
    let row = vec![
        SpredsheetCell::Int(3),
        SpredsheetCell::Text(String::from("blue")),
        SpredsheetCell::Float(10.12),
    ];
}