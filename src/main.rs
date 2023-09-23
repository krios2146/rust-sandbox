use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Number guessing game begins!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Enter the guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = guess.trim().parse()
        .expect("Please type a number");

    println!("Your guess is: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Too big")
    }
}
