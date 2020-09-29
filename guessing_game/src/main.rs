use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // let mut -> var in swift
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // pass in guess as a _mutable_ ref 
            .expect("Failed to read line"); // unwrap

        // shadow previous guess variable with this new one
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // not a mutable ref bc it doens't need to be
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            }
        }
    }
}
