fn main() {
    // ref_and_borw();
    try_change_borrowed_value();
}

fn ref_and_borw() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // sはStringへの参照
    s.len()
} // ここでは、sはスコープ外になるが、参照しているものの所有権を持っているわけではないので何も起こらない。

fn try_change_borrowed_value() {
    // let s = String::from("hello"); // 参照は普遍なため、changeによる変更はコンパイルエラーになる
    // change(&s);
    let mut s = String::from("hello");
    change_mut(&mut s);

}
// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 参照は不変なのでコンパイルエラー
// }
fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}
