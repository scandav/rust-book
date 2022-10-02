use rand::Rng;
use std::io;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("The random number is {random_number}");

    println!("Let's play the guessing game!");
    println!("Please guess the numnber");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("No number written");

    println!("You guessed number {guess}")
}
