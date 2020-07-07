use std::io;

fn main() {
    println!("Guess a number!");
    println!("Enter your guess");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failled to read line");
        
    println!("You guessed: {}", guess);
}
