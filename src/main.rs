use std::io;
use rand::Rng;

fn main() {
    println!("guess the number!");
    println!("");
    println!("Please input your guess");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("");
    println!("Your guessed: {}", guess);
    println!("");
    println!("Your guessed: {secret_number}");
}
