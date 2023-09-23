use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Number guessing game begins!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter the guess:");

        let mut guess = String::new();

        if io::stdin().read_line(&mut guess).is_err() {
            println!("Failed to read entered guess");
            continue;
        }

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter the number");
                continue;
            }
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
