use std::io;
use std::io::{Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::rng().random_range(1..=100);
    loop{
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}