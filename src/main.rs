extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret numer is : {}", secret_number);
    println!("Enter your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failled to read line");
    println!("You guessed: {}", guess);
}
