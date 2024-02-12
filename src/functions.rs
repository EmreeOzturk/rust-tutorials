pub fn functions() {
    fn _is_even(x: i32) -> bool {
        let digit: u8 = (x % 10) as u8;
        if digit % 2 == 0 {
            return true;
        } else {
            return false;
        }
    }

    fn _add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn _no_param_void() {
        println!("it just work {}",2);
    }

    fn _no_param() -> i32 {
        println!("hi world");
        44
    }
}
