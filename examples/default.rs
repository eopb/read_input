//To run this example `cargo run --example default --release`
//This example shows `input_d` in use.

use read_input::shortcut::input_d;

fn main() {
    println!("Type i32");
    input_d::<i32>().get();
    println!("Type u32");
    input_d::<u32>().get();
    println!("Type f32");
    input_d::<f32>().get();
    println!("Type bool");
    input_d::<bool>().get();
    println!("Type char");
    input_d::<char>().get();
    dont_disappear::enter_to_continue::default();
}
