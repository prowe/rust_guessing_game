extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::result;
use std::error::Error;

fn main() {
    println!("Guess the number!");

    let secret_number = gen_secret_number();
    
    let guess = read_guess().expect("bad guess");

    match guess.cmp(&secret_number) {
        Ordering::Less      => println!("Too small"),
        Ordering::Equal     => println!("You win"),
        Ordering::Greater   => println!("Too big")
    }
}

fn gen_secret_number() -> u32 {
    let num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", num);
    return num;
}

fn read_guess() -> Result<u32, Error> {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)?;

    let trimmed_guess = guess.trim();
    let guess : u32 = match guess.trim().parse() {
        Err(e) => return Err(e),
        Ok(g) => g
    };

    println!("You guessed: {}", guess);
    return Ok(guess);
}