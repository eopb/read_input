//To run this example `cargo run --example how_long_untill --release`
//Example program that tells you how long it is untill the date you typed in.
//This program is here to show that types from external crates can uses `read_input`

extern crate chrono;
extern crate read_input;

use chrono::prelude::*;
use read_input::*;
use std::str::FromStr;
#[derive(Debug)]
struct DateDDMMYY(Date<Local>);

fn main() {
    let mut date = DateDDMMYY(Local::now().date());
    date = simple_input::<DateDDMMYY>();
    println!("You guessed: {:?}", date);
}
impl FromStr for DateDDMMYY {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<&str> = s.split('/').collect();
        let mut early_return = false;
        if numbers.len() != 3 {
            return Err(());
        }
        let numbers: Vec<u32> = numbers
            .iter()
            .map(|num| {
                let x = u32::from_str(num.trim());
                match x {
                    Ok(num) => num,
                    Err(_) => {
                        early_return = true;
                        1
                    }
                }
            })
            .collect();
        if early_return {
            return Err(());
        }
        println!("{:?}", numbers);
        Ok(DateDDMMYY(Local::now().date()))
    }
}
