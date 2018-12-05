//To run this example `cargo run --example main --release`

extern crate read_input;

use read_input::prelude::*;

fn main() {
    let settings = input_new()
        .msg("Please input a number between 4 and 9 that is not 6: ")
        .add_test(|x| 4 < *x && *x < 9)
        .add_err_test(
            |x| *x != 6,
            "That value is 6! I dont want 6. Please try again",
        )
        .err("That does not look like a number between 4 and 9. Please try again");
    println!("output {}", settings.get());
    println!("output {}", settings.get());
    println!("output {}", valid_input(|x| 4 < *x && *x < 9));
    println!("output {}", simple_input::<char>());
}
