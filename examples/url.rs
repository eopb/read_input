//To run this example `cargo run --example url --release`

extern crate dont_disappear;
extern crate read_input;
extern crate url;

use read_input::*;
use url::Url;

fn main() {
    println!(
        "You inputted the URL {:#?}",
        input_new::<Url>()
            .err_match(with_description)
            .repeat_msg("Please input a URL: ")
            .get()
    );
    dont_disappear::enter_to_continue::default();
}
