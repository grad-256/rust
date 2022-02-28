struct Point<T> {
    x: T,
    y: T,
}

struct PointAnother<T, U> {
    x: T,
    y: U,
}

struct PointTaple<T, U, G> {
    x: T,
    y: U,
    z: G,
}

impl<T, U, G> PointTaple<T, U, G> {
    fn mixup<V, W, J>(self, other: PointTaple<V, W, J>) -> PointTaple<T, W, J> {
        PointTaple {
            x: self.x,
            y: other.y,
            z: other.z,
        }
    }
}

// メソッドを追加していく impl
// 中にメソッドをつくていく
// PointAnotherを使って構造体を作る
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    // どうゆう時に役立つのか
    // 動的な割り当てをしたい時に役立つ

    let number_list = vec![34, 50, 25, 100, 65];
    // println!("{}", largest_i32(number_list));

    let char_list = vec!["a", "b", "c", "d", "e"];
    println!("{}", largest(number_list));
    println!("{}", largest(char_list));
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 1.0, y: 2.0 };
    let p4 = PointAnother { x: "Rust", y: "a" };

    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);

    let g = PointTaple {
        x: "Rust",
        y: "a",
        z: 4,
    };
    println!("{} {} {}", g.x, g.y, g.z);
    let g2 = PointTaple {
        x: "Rust",
        y: 2,
        z: 4,
    };
    let g1 = g.mixup(g2);
    println!("{} {} {}", g1.x, g1.y, g1.z);
}

// fn largest_i32(list: Vec<i32>) -> i32 {
//     let mut largest = list[0];
//     for number in list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     largest
// }

// トレイト境界
// 共通の振る舞いを抽象的に定義でき、トレイト境界を使用するとあるジェネリックが特定の振る舞いをもつあらゆる型になり得ることを指定する
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
