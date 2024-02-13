use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("an error occured");
        let guess: u32 = guess.trim().parse().expect("please type a number");

        println!("guess : {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("equal");
                break;
            }
            Ordering::Greater => println!("greater"),
            Ordering::Less => println!("less"),
        }

        
    }
}
