//To run this example `cargo run --example how_long_until --release`
//Example program that tells you how long it is until the date you typed in.
//This program is here to show that types from external crates can uses `read_input`

extern crate chrono;
extern crate dont_disappear;
extern crate read_input;
use chrono::offset::{Local, TimeZone};
use chrono::prelude::*;
use read_input::prelude::*;
use std::str::FromStr;

struct DateDDMMYY(DateTime<Local>);

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

fn main() {
    println!(
        "That date is {} days away!",
        input_new::<DateDDMMYY>()
            .msg("Please input a date in the future in the format Y/M/D: ")
            .add_err_test(
                |time| time.0 > Local::now(),
                "Please input a date in the Future.",
            )
            .get()
            .0
            .signed_duration_since(Local::now())
            .num_days()
    );
    dont_disappear::enter_to_continue::default();
}
