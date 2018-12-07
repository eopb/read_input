//To run this example `cargo run --example default --release`
//This example shows `input_new_d` in use.

extern crate dont_disappear;
extern crate read_input;

use read_input::shortcut::input_new_d;

fn main() {
    println!("Type i32");
    input_new_d::<i32>().get();
    println!("Type u32");
    input_new_d::<u32>().get();
    println!("Type f32");
    input_new_d::<f32>().get();
    println!("Type bool");
    input_new_d::<bool>().get();
    println!("Type char");
    input_new_d::<char>().get();
    dont_disappear::enter_to_continue::default();
}
