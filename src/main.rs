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

    let _first_element = _days_of_week[0];
    let _last_element = _days_of_week[_days_of_week.len() - 1];

    // slices

    let _two_days = &_days_of_week[1..3];
    let _first_element_of_slice = _two_days[0];

    // tuples

    let _me = ("Emre", 30, true);
    let _name = _me.0;
    let _age = _me.1;
    let _is_cool = _me.2;

    //  unit type
    let _unit_type = ();

    // variables

    let _my_variable = 10;
    // dont mutable this variable so you can't change it _my_variable = 20; 
    // javada final js'te const gibi
    // default olarak immutable 

    let mut _my_mutable_variable = 10;
    _my_mutable_variable = 20;
    //  you can change it because it is mutable variable 

}
