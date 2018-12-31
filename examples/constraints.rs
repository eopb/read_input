//To run this example `cargo run --example constraints --release`

//This example shows use `InputConstraints` methods.

use read_input::prelude::*;

fn main() {
    input_new()
        .repeat_msg("Please input an number that is not 5: ")
        .not(5)
        .get();
    input_new()
        .repeat_msg("Please input an number that is more than 4: ")
        .min(4)
        .get();
    input_new()
        .repeat_msg("Please input an number that is less than 4: ")
        .max(4)
        .get();
    input_new()
        .repeat_msg("Please input an number that is from 4 to 9: ")
        .min_max(4, 9)
        .get();
}
