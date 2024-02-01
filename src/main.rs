fn main() {
    println!("Hello, world!");

    let x: i32 = 42;
    let pi: f64 = 3.14159;
    let is_rust_cool: bool = true;
    let random_letter: char = 'z';

    fn add_one(x: i32) -> i32 {
        return x + 1;

        // or x+1
    }

    fn sum_two_numbers(x: i32, y: i32) -> i32 {
        return x + y;

        // or x+y
    }

    let x = 4;

    if x == 4 {
        println!("x is 4!");
    } else {
        println!("x is not 4 :(");
    }

    println!("x = {}", x); // x = 4

    let mut x = 4;

    while x != 0 {
        println!("x = {}", x);
        x -= 1;
    }

    let _my_name_is = "Emre";
    let _my_name = String::from("Emre Ozturk");

    let _my_name_is = "Emre";

    let _days_of_week: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
}
