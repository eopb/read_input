//To run this example `cargo run --example inside_array --release`

//This example shows use of `.inside()` with an array.

use read_input::prelude::*;

fn main() {
    input_new()
        .repeat_msg("Please input an animal: ")
        .inside(["cat".to_string(), "dog".to_string(), "giraffe".to_string()])
        .err("That is not an animal I know.")
        .get();
}
