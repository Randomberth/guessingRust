use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess the number!");
    println!("");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess : ");

        let mut guess: String = String::new();
        //let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("");
        println!("Your guessed: {}", guess);
        println!("");
        println!("Generated: {secret_number}");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(" ");
                println!("Please input a number type");
                println!(" ");
                continue;
            }
        };

        println!("");
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("you guess is Smaller!"),
            Ordering::Greater => println!("you guess is Bigger!"),
            Ordering::Equal => {
                println!("Yeah baby!! match!!!");
                break;
            }
        }
    }
}
