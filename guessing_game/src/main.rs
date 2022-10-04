use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let solution = rand::thread_rng().gen_range(1..=100);
    // println!("The random number is {solution}");

    println!("Let's play the guessing game!");
    println!("Please guess the number (in the range [1, 100])");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("No number written");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("You guessed number {guess}");

        match guess.cmp(&solution) {
            Ordering::Less => println!("Too little!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
