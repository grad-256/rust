pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    // ライフタイムが違う場合
    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
    // スコープの外でres2を使おうとするとダングリングポインタが発生する
    // 指し示す先に何もない状態になっているからなっている
    // println!("{}", res2); ドリップしている
}

// <'a> 返り値のリファレンスのライフタイムは受け取った値の短い方を適応する &'aで指定する
// -> &strではどちらを受け取ったらいいのかわからずエラーになる
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }

// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

fn dummy3() -> String {
    let s = String::from("demo");
    s
}
