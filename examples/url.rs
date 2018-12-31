//To run this example `cargo run --example url --release`

use read_input::prelude::*;
use read_input::shortcut::with_description;
use url::Url;

fn main() {
    println!(
        "You inputted the URL {:#?}",
        input::<Url>()
            .err_match(with_description)
            .repeat_msg("Please input a URL: ")
            .get()
    );
    dont_disappear::enter_to_continue::default();
}
