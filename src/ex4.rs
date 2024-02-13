pub fn ex4() {
    let value: bool = false;

    match value {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let my_int: i32 = 2;

    match my_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => print!("other"),
    }
}
