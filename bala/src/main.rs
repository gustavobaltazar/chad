use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let rng = rand::thread_rng().gen_range(0..100);

    let guessed_number = loop {
        let guess = match input("Type a number").parse::<i32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Type a correct number!");
                continue;
            }
        };
        match guess.cmp(&rng) {
            Ordering::Less => println!("Guessed number is less than random one!"),
            Ordering::Greater => println!("Guessed number is greater than random one!"),
            Ordering::Equal => {
                println!("Congrats!!! you guessed correctly!");
                break guess;
            }
        }
    };
    println!("The random number it was {}", guessed_number)
}
fn input(message: &str) -> String {
    println!("{message}");
    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}
