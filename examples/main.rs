//To run example `cargo run --example main --release`

extern crate read_input;

use read_input::*;

fn main() {
    println!(
        "output {}",
        input_new()
            .msg("Please input a number between 4 and 9 that is not 6: ")
            .test(&|x| 4 < *x && *x < 9, None)
            .test(
                &|x| *x != 6,
                Some("That value is 6! I dont what7= 6. Please try again")
            )
            .err("That does not look like a number between 4 and 9. Please try again")
            .get()
    );
    println!("output {}", valid_input::<i32>(&|x| 4 < *x && *x < 9));
    println!("output {}", simple_input::<char>());
}
