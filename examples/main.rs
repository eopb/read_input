#![warn(clippy::pedantic)]
//To run example `cargo run --example main --release`

extern crate read_input;

use read_input::*;

fn main() {
    println!(
        "output {}",
        i32::read_input(
            Some("Please input a number"),
            "That does not look like a number",
            |_| true
        )
    );
    println!(
        "output {}",
        String::read_input(
            Some("Please input your name"),
            "That does not look like a name",
            |_| true
        )
    );
    println!("output {}", i32::simple_input());
    println!("output {}", i32::valid_input(|x| 4 < *x && *x < 9));
    println!("output {}", char::simple_input());
}
