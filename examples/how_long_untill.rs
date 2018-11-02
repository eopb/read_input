//To run this example `cargo run --example how_long_untill --release`
//Example program that tells you how long it is untill the date you typed in.
//This program is here to show that types from external crates can uses `read_input`

extern crate chrono;
extern crate read_input;

use chrono::prelude::*;
use read_input::*;
use std::str::FromStr;
#[derive(Debug)]
struct DateDDMMYY(DateTime<Local>);

fn main() {
    let date = simple_input::<DateDDMMYY>();
    println!("You guessed: {:?}", date);
}
impl FromStr for DateDDMMYY {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s2 = s.to_string();
        s2.push_str(" 00:00:00");
        match Local.datetime_from_str(s2.trim(), "%Y/%m/%d %H:%M:%S") {
            Ok(time) => Ok(DateDDMMYY(time)),
            Err(_) => Err(()),
        }
    }
}
