//To run this example `cargo run --example default --release`

extern crate dont_disappear;
extern crate read_input;

use read_input::prelude::shortcut::default_input_set;

fn main() {
    default_input_set::<i32>().get();
    default_input_set::<u32>().get();
    default_input_set::<f32>().get();
    default_input_set::<bool>().get();
    default_input_set::<char>().get();

    dont_disappear::enter_to_continue::default();
}
