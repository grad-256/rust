// pub mod sub_a;
// pub mod sub_b;

// 変数はスタックに格納されている
// constは好きなところで定義できるが、letはできない
// constは静的領域にバイナリで格納されている 0x....
const MAX_POINTS: u32 = 100_000;
// let dummyNumber = 2

pub fn run() {
    println!("Here is vars modules");
    // sub_a::func_a_run();
    // sub_b::func_b_run();
    let mut x = 5; // mut 可変にできる
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _i = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS); // システムサイズを表示
    println!("Memory address of const is: {:p}", &MAX_POINTS); // 格納されているアドレスをポインタ形式で表示

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2); // 格納されているアドレスをポインタ形式で表示
    println!("Stack address of i3 is: {:p}", &i3); // 格納されているアドレスをポインタ形式で表示

    let y = 5;
    println!("Stack address of y is: {:p}", &y); // 格納されているアドレスをポインタ形式で表示
    let y = y + 1; // 同じ変数名でletが使える はシャドーイングによって前の変数が隠れる
    println!("Stack address of y is: {:p}", &y); // 格納されているアドレスをポインタ形式で表示
    let y = y * 2; // 同じ変数名はシャドーイングによって前の変数が隠れる
    println!("Stack address of y is: {:p}", &y); // 格納されているアドレスをポインタ形式で表示
    println!("The value of y is :{}", y);
    {
        let y = 0;
        println!("The value of y is :{}", y);
    }
    println!("The value of y is :{}", y);

    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of x, y, z is :{} {} {}", x, y, z);
    println!("The value of t1 is :{} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2; // 各要素の値だけではなくポインタも取得する場合は、refをつける
    *x1_ptr = 5; // 参照はずし * アスタリスクつける
    *y1_ptr = -5; // 参照はずし * アスタリスクつける
    println!("{:?}", t2); // {} カーリーブラケット {:?} 複雑なデータ型も表示できる

    let a1 = [1, 2, 3, 4, 5]; // スタック
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライスは静的領域に格納される ポインタとレングスがある
    // 参照を意識する
    // なぜ所有にならないのか？
    // 静的領域に格納されるとそもそも解放する必要がない
    // ストリング型か作った場合、所有権はString型から移動しない
    // 所有権を移動させずに参照する権利だけ貸し出す。（借用）
    let s1 = "hello Rust"; // 文字列スライス
    println!("Stack address of y is: {:p}", &s1); // 格納されているアドレスをポインタ形式で表示
    println!("Stack memory address of y is: {:?}", s1.as_ptr()); // 静的格納領域に紐付いたアドレスの値を取得
    println!("Len of s1 is: {}", s1.len()); // スタック内の値を取得

    // ストリングはヒープメモリに格納されている
    // ヒープとは動的に確保と解放ができるメモリ領域のこと データの仮置き場や、臨時の作業台
    // データ所有 = メモリ解放(drop)をする責任を負う
    // 所有を意識する
    // ポインタとレングスとキャパシティーがある
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1); //
    println!("Stack address of s2 is: {:p}", &s2); //
    println!("Stack memory address of y is: {:?}", s1.as_ptr());
    println!("Stack memory address of y is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1"); // 文字追加
    s2.push_str("_new2"); // 文字追加
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    println!("{} {}", s1, s2);

    // 所有権を導入する理由
    // 二重開放エラー回避のために導入されている
    // どうゆう時？
    // メモリを二重に開放してしまうことを言います。
    // 所有者だけがヒープメモリ確保されたメモリを解放できる
}
