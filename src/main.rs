use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Please enter your guess number (between 1 and 100):");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error occurred reading input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Your guess is less than random number"),
            Ordering::Equal => {
                println!("Nice, you guessed it correctly");
                break;
            }
            Ordering::Greater => println!("Your guess is bigger than random number")
        }
    }
}
