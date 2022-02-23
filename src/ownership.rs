// 所有権と参照と借用

pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 value borrowed here after move
    // 所有権はs2に動いているのでアクセスできませんとエラーになる
    // println!("{} {}", s1, s2);
    // 変数、関数の引数や戻り値も所有権が動く
    println!("{}", s2);

    // 参照している コピートレイト 所有権のムーブは起こっていない
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);

    // それぞれのスタックアドレスを表示
    println!("Stack address of i1 is: {:p}", &i1);
    println!("Stack address of i2 is: {:p}", &i2);

    // 文字列スライス
    let sl1 = "literal";
    let sl2 = sl1;
    println!("{} {}", sl1, sl2);

    println!("Stack address of sl1 is: {:p}", &sl1);
    println!("Stack address of sl2 is: {:p}", &sl2);

    // deep copy クローン
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 is: {:p}", &s3);
    println!("Stack address of s4 is: {:p}", &s4);
    println!("Heap memory address of s3: {:?}", s3.as_ptr());
    println!("Heap memory address of s4: {:?}", s4.as_ptr());

    let s5 = String::from("hello");
    // take_ownershipの中と同じ
    println!("Stack address of s5 is: {:p}", &s5);
    println!("Heap memory address of hello: {:?}", s5.as_ptr());
    println!("Len is: {}", s5.len());
    println!("Capacity is: {}", s5.capacity());
    take_ownership(s5);

    // take_ownership関数にs5の所有権が動いているので使えない
    // println!("{}", s5)
    let s6 = String::from("hello");
    println!("Stack address of s6 is: {:p}", &s6);
    println!("Heap memory address of hello: {:?}", s6.as_ptr());
    println!("Len is: {}", s6.len());
    // 最終的にs7が所有権を持つことになります
    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 is: {:p}", &s7);
    println!("Heap memory address of hello: {:?}", s7.as_ptr());
    println!("Len is: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8); // & 参照
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // イミュータブルとミュータブルは共存できない
    // let mut s10 = String::from("hello");
    // let r1 = &s10; // immutable
    // let r2 = &mut s10; // mutable
    //                    // データの生合成を保つために出るエラー
    // println!("{} {}", r1, r2);

    // 有効範囲の例
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    // println!("{}", s11); // ミュータブルなレファレンスが有効な領域だと読みことができない
    // println!("{}", r1);
    println!("{}", r1);
    println!("{}", s11);

    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    // 範囲外だと
    let r3 = &mut s12;
    *r3 = String::from("hello_updated");
    println!("{}", s12);
}

fn take_ownership(s: String) {
    println!("Stack address of s is: {:p}", &s);
    println!("Heap memory address of s: {:?}", s.as_ptr());
    println!("Len is: {}", s.len());
    println!("Capacity is: {}", s.capacity());
    println!("{}", s)
}

// 明示的にreturnは書かない 最後の行のセミコロンが値をretrun値として返される
fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
