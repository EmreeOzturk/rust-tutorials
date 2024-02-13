pub fn ex9() {
    let x: i32 = 4;
    let y: i32 = 3;

    let coordinates: (i32, i32) = create_coordinates(x, y);
    let res: &str = printval(coordinates);
    println!("{}", res)
}

fn printval(coordinates: (i32, i32)) -> &'static str {
    let res: &str = if coordinates.1 > 5 {
        "greater 5"
    } else if coordinates.1 == 5 {
        "equals 5"
    } else {
        "less than 5"
    };

    res
}

fn create_coordinates(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}
