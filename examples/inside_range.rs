//To run this example `cargo run --example inside_range --release`

//This example shows use of `.inside()` with a range.

use read_input::prelude::*;

fn main() {
    println!(
        "You inputted: {}",
        input()
            .repeat_msg("Please input a number between 1 and 10: ")
            .inside(2..10)
            .default(5)
            .get()
    );
}
