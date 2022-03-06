#[derive(Debug)]
struct User {
    email: String,
    username: String,
    sign_in_count: u64, //整数型
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    let user1 = User {
        email: String::from("someone@g.com"),
        username: String::from("somone123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        email: String::from("someone@g.com"),
        username: String::from("somone123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone123@g.com");

    println!("{:#?}", user1);
    let user2 = build_user(String::from("user2@wwww.www"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}
