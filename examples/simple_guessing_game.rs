//To run this example `cargo run --example simple_guessing_game --release`
//This program is based on the guessing game form the rust book.
//https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use read_input::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let guess: i32 = input().msg("Please input your guess: ").get();

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
    dont_disappear::enter_to_continue::default();
}
