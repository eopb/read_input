//To run example `cargo run --example main --release`

extern crate read_input;

use read_input::*;

fn main() {
    println!("output {}", u64::input_new().get());
    println!("output {}", i32::simple_input());
    println!("output {}", i32::valid_input(|x| 4 < *x && *x < 9));
    println!("output {}", char::simple_input());
}
