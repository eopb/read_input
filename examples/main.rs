#![warn(clippy::pedantic)]
//To run example `cargo run --example main --release`

extern crate read_input;

use read_input::*;

fn main() {
    println!(
        "output {}",
        i32::read_input("Please input a number", "That does not look like a number")
    );
}
