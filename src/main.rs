extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is {}", secret_number);

    println!("input your guess. >> ");
    // let is to define a variable and mut is to make var mutable othervise
    // the var is immutable
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to red line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!");
    Ordering::Greater => println!("Too big!");
    Ordering::Equal => println!("You win");
    }
}