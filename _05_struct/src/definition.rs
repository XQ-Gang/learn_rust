#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("user1 = {:?}", user1);

    // 使用结构体更新语法从其他实例创建实例
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 = {:?}", user2);

    // 使用没有命名字段的元组结构体来创建不同的类型
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("black = {:?}", black);
    let origin = Point(0, 0, 0);
    println!("origin = {:?}", origin);

    // 没有任何字段的类单元结构体（unit-like structs）
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    println!("subject = {:?}", subject);
}

// 字段初始化简写语法（field init shorthand）
// username 和 email 参数与结构体字段同名，则只需编写 email 而不是 email: email
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
