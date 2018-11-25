extern crate dont_disappear;
extern crate read_input;
extern crate url;

use read_input::*;
use std::error::Error;
use url::Url;

fn main() {
    println!(
        "You inputted the URL {:#?}",
        input_new::<Url>()
            .repeat_msg("Please input a URL: ")
            .err_match(|e| Some(format!("Error \"{}\"", e.description())))
            .get()
    );
    dont_disappear::enter_to_continue::default();
}
