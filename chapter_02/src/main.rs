// APIs
use rand::Rng;
use std::io;
use std::{cmp::Ordering, u32};

fn main() {
    println!("Guess the number!");

    // generating a random secret number
    // thread_rng => same thread as the main program
    // seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // constantly asks the user for guesses
    loop {
        let mut guess = String::new();

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // transform guess from string to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(yo) => yo,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // comparing the numbers
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("perfect!");
                break;
            }
        }

        println!("--------------");
    }
}
