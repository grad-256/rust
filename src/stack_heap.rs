enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // 実際にスタッオーバーフローを起こす スタックにはあまり大きなデータは入れられない！！
    // fatal runtime error: Stack Overflow
    // 配列において、サイズが決定している必要がある
    let a1: [u8; 9000000] = [1; 9000000];

    // 実行時に変更していきたい時に使う
    // Vector型 24bytes構成
    // ポインターとレングスとキャパシティーがあり、レングスとキャパシティーに要素数を持っている
    // Heapに格納されるときに要素数が格納される
    // Heapには1レングス４バイトの要素が入っている
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    println!("Heap memory address of v1: {:?}", v1.as_ptr());
    println!("Len of v1 is: {}", v1.len());
    println!("Capacity of v1 is: {}", v1.capacity());
    // 挿入
    v1.insert(1, 10);
    println!("{:?}", v1);
    // 削除
    v1.remove(0);
    println!("{:?}", v1);
    // 連結
    v1.append(&mut v3);
    println!("{:?}", v1); // [1,2,3,4,9,10]
    println!("{:?}", v3); // []

    // Box pointer
    // スタックに存在するデータをヒープに移動し移動させたデータをbox pointerとして所有する
    // 使い道はサイズが決まらないデータをヒープ入れておいて、そのデータのポインターだけスタック所有し、コンパイが通るようになる
    let t1: (i64, String) = (10, String::from("hello")); // タプル
    println!("Stack address of tuple data is: {:p}", &t1);
    println!("Heap memory address of t1.1: {:?}", t1.1.as_ptr());
    println!("Len of t1 is: {}", t1.1.len());
    println!("Capacity of t1 is: {}", t1.1.capacity());

    // Box Pointerを作る
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
    // &を使ってb1が格納されているアドレスを確認する
    println!("Stack address of box pointer is: {:p}", &b1);
    println!("Heap address of tuple data is: {:p}", &b1);
}
