fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
