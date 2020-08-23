fn main() {
    // ownership_and_fn();
    ret_val_and_scope();
}

fn ownership_and_fn() {
    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にmoveされる
                        // ここではもう有効ではない

    let x = 5;          // xがスコープに入る

    makes_copy(x);      // xも関数にムーブされるが、i32はCopyなのでこの後にxを使ってもOK

} // ここでxがスコープを抜け、sもスコープを抜ける。
  // ただし、sの値はmoveされているので、何も特別なことはない。

fn takes_ownership(some_string: String) { // some_stringがスコープに入る
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが開放される。

fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn ret_val_and_scope() {
    let s1 = gives_ownership(); //gives_ownershipは、戻り値をs1にmoveする

    let s2 = String::from("hello"); // s2がスコープに入る

    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backにmoveされ
                                       // 戻り値もs3にmoveされる
} // ここでs3はスコープを抜け、dropされる。s2もスコープを抜けるが、moveされているため
  // 何も起きない。s1もスコープを抜け、dropされる。

fn gives_ownership() -> String {                // gives_ownershipは、戻り値を
                                                // 呼び出した関数にmoveする

    let some_string = String::from("hello");    // some_stringがスコープに入る
    some_string                                 // some_stringが返され、呼び出し元関数にmoveされる
}

fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る
    a_string // a_strngが返され、呼び出し元関数にmoveされる
}
