//To run this example `cargo run --example guessing_game --release`
//This program is based on the guessing game form the rust book.
//https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
//This version has some minor improvements.

extern crate rand;
extern crate read_input;

use rand::Rng;
use read_input::*;
use std::cmp::Ordering;

fn main() {
    println!("I am thinking of a number between 1 and 100.");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let guess: i32 = input_new()
            .msg("Please input your guess: ")
            .test(
                &|x| !(*x > 100),
                Some("That number is more than 100. Please try again"),
            )
            .test(
                &|x| !(*x < 1),
                Some("That number is less than 1. Please try again"),
            )
            .err("That does not look like a number. Please try again")
            .get();

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
