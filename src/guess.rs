use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guess() {
    println!("Guess a number");
    println!("Please input your guess");
    let secret_numb: i32 = rand::thread_rng().gen_range(1, 101);
    println!("your sec {}", secret_numb);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("An error occured");

        let int_guess: i32 = guess.trim().parse().expect("qwe");
        println!("Your guess {}", guess);

        match int_guess.cmp(&secret_numb) {
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}
