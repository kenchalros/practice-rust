enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    CnahgeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body wold be defined here
        // メソッド本体はここに定義される
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_centes(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let coin1 = Coin::Penny;
    let val1 = value_in_centes(coin1);
    println!("coin1 value is: {}", val1);

    let coin2 = Coin::Quarter(UsState::Alaska);
    let val2 = value_in_centes(coin2);
    println!("coin2 value is: {}", val2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other val"),
    }

    // Some(3)にマッチしたときだけ何かをし、他の値やNoneの時には
    // 何もしたくないという場合でも、match式を満たすためには、列挙式
    // を一つだけ処理した後に _ => () を追加しなければならない。
    // これでは追加すべき定型コードが多すぎる。
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 代わりに、if letを使用して短く書くことができる。
    // 上記のmatchと同じように振る舞う。
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin3 = Coin::Penny;
    let mut count = 0;
    match coin3 {
        Coin::Quarter(state) => println!("State quater from {:?}!", state),
        _ => count += 1
    }
    println!("count: {}", count);

    let coin4 = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin4 {
        println!("State quater from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}
