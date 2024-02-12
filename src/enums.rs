pub fn enums() {
    enum my_enums {
        A(i32),
        B(bool),
        C(String),
        D{ a: i32, b: bool },
        E,
    }

    let _a = my_enums::A(10);
    let _b = my_enums::B(true);
    let _c: my_enums = my_enums::C(String::from("Hello"));
    let _d = my_enums::D { a: 10, b: true };
    let _e = my_enums::E;

    match _a {
        my_enums::A(10) => println!("A is 10"),
        my_enums::A(_) => println!("A is not 10"),
        _ => println!("Not A"),
    }

    match _b {
        my_enums::B(true) => println!("B is true"),
        my_enums::B(false) => println!("B is false"),
        _ => println!("Not B"),
    }

    match _c {
        my_enums::C(_) => println!("C is a string"),
        _ => println!("Not C"),
    }

    match _d {
        my_enums::D { a: 10, b: true } => println!("D is 10 and true"),
        my_enums::D { a: 10, b: false } => println!("D is 10 and false"),
        my_enums::D { a: _, b: _ } => println!("D is something else"),
        _ => println!("Not D"),
    }

    match _e {
        my_enums::E => println!("E"),
        _ => println!("Not E"),
    }

    // Option enum
    let _x: Option<i32> = Some(10);
    let _y: Option<i32> = None;

    match _x {
        Some(i) => println!("x is {}", i),
        None => println!("x is None"),
    }

    match _y {
        Some(i) => println!("y is {}", i),
        None => println!("y is None"),
    }

    // Result enum
    let _result: Result<i32, &str> = Ok(5);
    let _error: Result<i32, &str> = Err("errorr");

    match _result {
        Ok(i) => println!("result is {}", i),
        Err(e) => println!("error is {}", e),
    }

    match _error {
        Ok(i) => println!("result is {}", i),
        Err(e) => println!("error isss {}", e),
    }

    // if let
    if let Some(i) = _x {
        println!("x is {}", i);
    } else {
        println!("x is None");
    }

    if let Some(i) = _y {
        println!("y is {}", i);
    } else {
        println!("y is None");
    }

    if let Ok(i) = _result {
        println!("result is {}", i);
    } else {
        println!("result is error");
    }

    if let Err(e) = _error {
        println!("error is {}", e);
    } else {
        println!("error is result");
    }

    // unwrap
    let _x = _x.unwrap();
    let _y = _y.unwrap_or(0);
    let _result = _result.unwrap();
    let _error = _error.unwrap_err();

    println!("x is {}", _x);
    println!("y is {}", _y);
    println!("result is {}", _result);
    println!("error is {}", _error);
}
