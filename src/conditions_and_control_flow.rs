pub fn conditions_and_control() {
    // < > <= >= == !=  && || !

    let cond: bool = 2 < 3;
    let cond2: bool = (2 as f32) > 3.2;
    let cond3: bool = true && cond2;
    let _cond4: bool = !cond3;
    let _cond5: bool = cond3 || cond2;

    if cond {
        println!("cond is true");
    } else {
        println!("cond is false");
    }

    if cond2 {
        println!("cond2 is true");
    } else if cond3 {
        println!("cond3 is true");
    } else {
        println!("cond2 and cond3 are false");
    }

    let x = String::from("hello");
    let y = &x;
    println!("cond = {:?}", cond);
    println!("cond2 = {:?}", cond2);
    println!("cond3 = {}", cond3);

    println!("x = {}", x);
    println!("y = {}", y);
}
