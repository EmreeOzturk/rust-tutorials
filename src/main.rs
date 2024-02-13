use crate::guess_game::guess_game;


mod conditions_and_control_flow;
mod enums;
mod ex1;
mod ex4;
mod ex7;
mod ex8;
mod ex9;
mod functions;
mod loops_and_comditions;
mod strings;
mod structs_and_traits;
mod variables;
mod ex11;
mod ex12;
mod guess_game;
fn main() {
    println!("Hello, world!");

    let _x: i32 = 42;
    let _pi: f64 = 3.14159;
    let _is_rust_cool: bool = true;
    let _random_letter: char = 'z';

    fn _add_one(x: i32) -> i32 {
        return x + 1;

        // or x+1
    }

    fn _sum_two_numbers(x: i32, y: i32) -> i32 {
        return x + y;

        // or x+y
    }

    let x = 4;

    if x == 4 {
        //   println!("x is 4!");
    } else {
        println!("x is not 4 :(");
    }

    // println!("x = {}", x); // x = 4

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
    //  slices are immutable
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

    variables::learn_variables();
    functions::functions();
    strings::learn_strings();
    loops_and_comditions::loops_and_comditions();
    structs_and_traits::structs();
    enums::enums();
    conditions_and_control_flow::conditions_and_control();
    ex1::ex1();
    ex4::ex4();
    ex7::ex7();
    ex8::ex8();
    ex9::ex9();
    ex11::ex11();
    ex12::ex12();
    guess_game::guess_game();
}
