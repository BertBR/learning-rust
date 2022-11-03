use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut rounds: u32 = 5;
    println!("Please input your guess.");

    loop {
        if rounds == 0 {
            println!("You have no more rounds left!");
            println!("The secret number was: {}", secret_number);
            break;
        }
        let rounds_msg = format!("You have {} rounds to guess the number", rounds);
        
        println!("{rounds_msg}");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        rounds -= 1;
    }
}
