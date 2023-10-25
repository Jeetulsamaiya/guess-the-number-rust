use rand::prelude::*;
use std::cmp::*;
use std::io;
fn main() {
    println!("Guess the Number");
    let random_number = thread_rng().gen_range(0..101);
    loop {
        println!("please enter a number to guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => {
                if (0..=100).contains(&number) {
                    number
                } else {
                    println!("please enter a number in range 0 to 100");
                    continue;
                }
            }
            Err(err) => {
                println!("invalid Input :-  {}, {}", err, guess);
                continue;
            }
        };
        println!("you guessed number {}", guess);
        // println!("random number is {}", random_number);
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => println!(" too big "),
            Ordering::Less => println!("too small "),
        }
    }
}
