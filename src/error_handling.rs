pub fn run() {
    let res1 = division_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Not allowed !!"),
    }

    let res2 = division_result(5.0, 1.0);
    match res2 {
        Ok(x) => println!("Result: {:.3}", x),
        Err(e) => println!("{}", e),
    }

    // 途中でもリターンで返される
    let a = [0, 1];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("Result: {:.3}", x),
        None => println!("Out of index !!"),
    }
}

// 標準で用意されている
// Optionは想定していない値に対して例外処理を書くことができる。
// Resultはエラーが発生する可能性のある値に対するエラー処理を書くことができる

// Optionを使ったエラーハンドリング
// pub enum Option<T> {
//     None,
//     Some(T),
// }

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

// Resultを使ったエラーハンドリング
// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not allowed !!"))
    } else {
        Ok(x / y)
    }
}

//以外のエラーハンドリング
fn sum(a: &[i32]) -> Option<i32> {
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
