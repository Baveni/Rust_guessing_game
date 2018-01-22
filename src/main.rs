extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // uncomment this to print out the secret number
    //("The secret number is {}", secret_number);

    loop {
        println!("input your guess. >> ");

        // let is to define a variable and mut is to make var mutable othervise
        // the var is immutable
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to red line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // the _ is catchall value
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
