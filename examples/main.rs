//To run example `cargo run --example main --release`

extern crate read_input;

use read_input::*;

fn main() {
    println!(
        "output {}",
        i16::input_new()
            .msg("Please input a number between 4 and 9 that is not 6: ")
            .test(&|x| 4 < *x && *x < 9)
            .test(&|x| *x != 6)
            .err("That does not look like a number between 4 and 9. Please try again")
            .get()
    );
    println!("output {}", i32::simple_input());
    println!("output {}", i32::valid_input(&|x| 4 < *x && *x < 9));
    println!("output {}", char::simple_input());
}
