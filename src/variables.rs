pub fn learn_variables() {
    //unsigned integer
    //u8, u16, u32, u64, u128

    let unsigned_integer: u8 = 255;
    let unsigned_integer: u16 = 65535;
    println!("unsigned_integer = {}", unsigned_integer);

    //signed integer
    //i8, i16, i32, i64, i128

    let signed_integer: i8 = -128;
    let signed_integer: i16 = -32768;
    println!("signed_integer = {}", signed_integer);

    //floating point
    //f32, f64

    let floating_point: f32 = 3.14;
    let floating_point: f64 = 3.141592653589793238;
    println!("floating_point = {}", floating_point);

    //boolean
    let boolean: bool = true;
    println!("boolean = {}", boolean);

    //character
    let character: char = 'a';
    println!("character = {}", character);

    //arrays
    let colors_of_rainbow: [&str; 7] = [
        "Red", "Orange", "Yellow", "Green", "Blue", "Indigo", "Violet",
    ];

    let first_color = colors_of_rainbow[0];
    let last_color = colors_of_rainbow[colors_of_rainbow.len() - 1];
    println!("first_color = {}", first_color);
    println!("last_color = {}", last_color);

    let num_array: [i32; 5] = [1, 2, 3, 4, 5];
    let other_num_array: [u8; 3] = [6, 7, 8];
    
    println!("num_array = {:?}", num_array);
    println!("other_num_array = {:?}", other_num_array);

    //tuples
    //like arrays but can have holds different types
    //tuples have a fixed length: once declared, they cannot grow or shrink in size
    //you can destructuring tuples to create bindings

    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    let tuple2 : (&str, i32, f64, u8) = ("Emre", 500, 6.4, 1);
    println!("tuple2 = {:?}", tuple2);
    println!("tuple2.0 = {}", tuple2.0);


    //mutable variables
    //variables are immutable by default
    //you can make them mutable by using the mut keyword
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);


    //slice
    //slices are immutable
    //slices like arrays, but their size is not known at compile time
    //slices have a length, can be used to borrow a section of an array
    //in solana a lot of the data that we pass in from the client is in the form of slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);



}
