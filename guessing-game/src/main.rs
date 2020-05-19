use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("{}[2J", 27 as char); // clear screen
    println!("Guess the random number!");

    let secret_number = rand::thread_rng().gen_range(1, 101); // inclusive on lowend, exclusive on top end

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // pass in a _mutable_ refrence to guess
            .expect("Failed to read line");
    
        // guess, of type u32. trim guess then parse out a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // on success, pass num to guess
            Err(_) => continue, //prompt for input again
        };

        println!("You guessed: {}", guess);

        // "switch" on guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // end the loop, the game is over
            }
        }
    }

    println!("Loop exited");
}
