//To run this example `cargo run --example inside_vector --release`

//This example shows use of `.inside()` with a vector.

use read_input::prelude::*;

fn main() {
    input()
        .repeat_msg("Please input an animal: ")
        .inside(vec![
            "cat".to_string(),
            "dog".to_string(),
            "giraffe".to_string(),
        ])
        .err("That is not an animal I know.")
        .get();
}
