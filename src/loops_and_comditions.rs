pub fn loops_and_comditions() {
    for index in 1..11 {
        println!("index = {}", index);
    }

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for element in arr.iter() {
        println!("element = {}", element);
    }

    let mut x = 4;

    while x != 0 {
        println!("x = {}", x);
        x -= 1;
    }

    loop {
        println!("x = {}", x);
        x += 1;
        if x == 10 {
            break;
        }
    }

    //match

    let value: i32 = 22;

    match value {
        86..=100 => println!("Great job!"),
        70..=85 => println!("You did OK"),
        _ => println!("You can do better"),
    }

    // if statemets in match

    match value {
        value if value > 86 => println!("Great job!"),
        value if value > 70 => println!("You did OK"),
        _ => println!("You can do better"),
    }
}
