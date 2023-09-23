use std::io;

fn main() {
    println!("Number guessing game begins!");

    println!("Enter the guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess is: {guess}")
}
