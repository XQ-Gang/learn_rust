fn main() {
    let some_number = Some(5);
    println!("some_number = {:?}", some_number);
    let some_char = Some('e');
    println!("some_char = {:?}", some_char);

    let absent_number: Option<i32> = None;
    println!("absent_number = {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    println!("sum = {sum}");
}
