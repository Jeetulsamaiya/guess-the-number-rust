use std::io;
use rand::prelude::*;
use std::cmp::*;
fn main() {
    println!("Guess the Number");
    let random_number = thread_rng().gen_range(0..101);
    loop {
    println!("please enter a number to guess");
    let mut guess = String::new();
    io::stdin().read_line( &mut guess).expect("failed to read line");
    let guess : u32 = guess.trim().parse().expect("failed to parse");
    println!("you guessed number {}", guess);
    println!("random number is {}", random_number);
    match guess.cmp(&random_number){
        Ordering::Equal => {
            println!("you win");
            break;
        },
        Ordering::Greater => println!(" too big "),
        Ordering::Less => println!("too small "),
    }
    }
}
