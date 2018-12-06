//To run this example `cargo run --example default --release`

extern crate dont_disappear;
extern crate read_input;

use read_input::shortcut::default_input_set;

fn main() {
    println!("Type i32");
    default_input_set::<i32>().get();
    println!("Type u32");
    default_input_set::<u32>().get();
    println!("Type f32");
    default_input_set::<f32>().get();
    println!("Type bool");
    default_input_set::<bool>().get();
    println!("Type char");
    default_input_set::<char>().get();
    dont_disappear::enter_to_continue::default();
}
