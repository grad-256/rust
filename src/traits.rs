// トレイトとは、共通の振る舞いを抽象的な形で定義すること
// トレイト境界を使用すると、あるジェネリックが、特定の振る舞いをもつあらゆる型になり得ることを指定できます。
// 複数の型のタイプに合わせて共通で持たせたい機能を実装したい時に使う
trait Fruits {
    // & 参照の形で受け取る
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    // & 参照の形で受け取る
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    // & 参照の形で受け取る
    fn price(&self) -> u32 {
        5
    }
}

// trait Summay {
//     // & 参照の形で受け取る
//     fn summarize(&self) -> String;
// }
// トレイトにおけるデフォルト実装について
// 具体的な処理を書くとそれがデフォルトになる
trait Summay {
    // & 参照の形で受け取る
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

trait Message {
    // & 参照の形で受け取る
    fn message(&self) -> String {
        String::from("Message")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summay for NewsArticle {
    // & 参照の形で受け取る
    // トレイト内に具体的な実装を書くとオーバーライドされそちらが表示される
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.location, self.author)
    // }
}

impl Message for NewsArticle {}
struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}
impl Summay for Tweet {
    // & 参照の形で受け取る
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    // 実態を渡すことができる
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple); // price is: 10
    get_price(banana); // price is: 5
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of couse, as you probably already know, people"),
        replay: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize()); // 1 new tweet: horse_ebooks: of couse, as you probably already know, people
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };
    println!("{}", article.summarize()); //headline, by location (author)
    notify(&article); // Breaking news! headline, by location (author)
    notify_another(&article);
    // notify_another(&tweet); エラーになる tweetの方ではmessageがない
}

// トレイト境界 <T: Fruits>
// ジェネリクス
fn get_price<T: Fruits>(fruits: T) {
    // fruitsの中にprice関数があることが保証されているので実行できる
    println!("price is: {}", fruits.price())
}
// &impl Summay トレイトもしくは型に実装されている関数を使用できるようにしている
fn notify(item: &impl Summay) {
    println!("Breaking news! {}", item.summarize())
}
// 構造体が2つのトレイトによってimplされていると以下のようにできる
// 2つの関数を呼び出せる
fn notify_another(item: &(impl Summay + Message)) {
    println!("Breaking news! {}", item.summarize());
    println!("Message! {}", item.message());
}
