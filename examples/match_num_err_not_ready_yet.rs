//Unstable

//To run this example `cargo run --example match_num_err --release`

// #![feature(int_error_matching)]
// extern crate core;
// extern crate dont_disappear;
// extern crate read_input;

// use read_input::prelude::*;

fn main() {
    // use core::num::IntErrorKind::*;
    // println!(
    //     "You inputted {:#?}",
    //     input_new::<i16>()
    //         .err_match(|x| Some(
    //             match x.kind() {
    //                 Empty => "You did not input any value. Try again.",
    //                 InvalidDigit => "You typed an invalid digit. Try again using only numbers.",
    //                 Overflow => "Integer is too large to store. Try again with a smaller number.",
    //                 Underflow => "Integer is too small to store. Try again with a smaller number.",
    //                 _ => "That value did not pass for an unexpected reason.",
    //             }
    //             .to_string()
    //         ))
    //         .repeat_msg("Please input a number: ")
    //         .get()
    // );
    dont_disappear::enter_to_continue::default();
}
