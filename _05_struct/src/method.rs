#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 所有在 impl 块中定义的函数被称为 关联函数（associated functions），
// 因为它们与 impl 后面命名的类型相关。
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 每个结构体都允许拥有多个 impl 块。
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Rust 并没有一个与 C/C++ -> 等效的运算符；
// 相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
// 当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。
// 也就是说，这些代码是等价的：
//      p1.distance(&p2);
//      (&p1).distance(&p2);
// 在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。
fn main() {
    let rect1 = Rectangle::square(40);
    let rect2 = Rectangle::square(30);
    let rect3 = Rectangle::square(50);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
