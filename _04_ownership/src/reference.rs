// 引用的规则：
// 1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 2. 引用必须总是有效的。
fn main() {
    let mut s = String::from("hello");

    change(&mut s); // 可变引用

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），
// 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
// 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
// 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
// 以下代码将发生编译错误：
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
