use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    // i32のようなCopyトレイトを実装する型について、値は
    // ハッシュマップにコピーされる。
    // Stringのような所有権のある値なら、値は
    // moveされ、ハッシュマップがそれらの値の所有者になる。
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}", field_name); // <= 所有権がhashmapにあるためコンパイルエラー

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score is {:?}", score);

    for (key, value) in &scores {
        // 順番は任意
        println!("{}: {}", key, value);
    }

    // キーに値がなかったときのみ値を挿入する
    // hashMapにはentryと呼ばれる特別なAPIがあり、
    // 引数としてチェックしたいキーを取る。戻り値はEntryと
    // 呼ばれるenumであり、存在したりしなかったりする可能性のある値を表す。
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // 古い値に基づいて値を更新する
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // or_insert関数は、キーに対する値への可変参照（&mut V）を返す
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
