use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub mod guess;
use guess::Guess;

fn main() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=10);

    println!("The secret number is {secret_number}.");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guess = Guess::new(guess);

        println!("You guessed: {}!", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

    }
}
