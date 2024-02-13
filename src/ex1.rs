pub fn ex1() {
    print_name("emre");
    print_surname("ozturk");
}

fn print_name(name: &str) {
    println!("my name is {}", name);
}

fn print_surname(surname: &str) {
    println!("and my surname is {}", surname);
}
