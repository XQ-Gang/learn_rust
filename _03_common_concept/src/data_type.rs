fn main() {
    // Scalar Type
    // 1. integer types:
    //   - i8, i16, i32 (default), i64, i128, isize
    //   - u8, u16, u32, u64, u128, usize
    // 2. floating-point types: f64 (default), f32
    // 3. boolean: false, - true
    // 4. character

    // Compound Type
    // 1. tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    let (x, y, z) = tup; // destructure
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    // 2. array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {:?}", arr);
    let arr = [3; 5];
    let first = arr[0];
    let second = arr[1];
    let third = arr[2];
    let fourth = arr[3];
    let fifth = arr[4];
    // let sixth = arr[5]; // panic
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");
    println!("The value of third is: {third}");
    println!("The value of fourth is: {fourth}");
    println!("The value of fifth is: {fifth}");
    println!("The value of arr is: {:?}", arr);
}