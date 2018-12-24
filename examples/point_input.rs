//To run this example `cargo run --example point_input --release`
//Example program that allows a user to input a point in 2D space.
//This program adapted from the `std::str::FromStr` trait documentation example.
//The program was written to show the use of the `err_match()` method in `read_input`

use read_input::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

enum ParsePointError {
    FailedParse(String),
    Not2Dimensional(usize),
    NonNumeric,
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_s = s
            .trim_matches(|p| p == '(' || p == ')')
            .trim()
            .replace(|p| p == ' ', "");
        {
            if !clean_s.contains(|c| {
                ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', ',', '-'].contains(&c)
            }) {
                return Err(ParsePointError::NonNumeric);
            }
        }
        let coords: Vec<&str> = clean_s.split(',').collect();
        if coords.len() != 2 {
            return Err(ParsePointError::Not2Dimensional(coords.len()));
        }
        Ok(Point {
            x: match coords[0].parse::<i32>() {
                Ok(num) => num,
                Err(_) => return Err(ParsePointError::FailedParse(coords[0].to_string())),
            },
            y: match coords[1].parse::<i32>() {
                Ok(num) => num,
                Err(_) => return Err(ParsePointError::FailedParse(coords[1].to_string())),
            },
        })
    }
}

fn main() {
    println!(
        "You inputted\n{:#?}",
        input_new::<Point>()
            .repeat_msg("Please input a point in 2D space in the format (x, y): ")
            .err_match(|e| Some(match e {
                ParsePointError::FailedParse(s) => format!(
                    "Failed to parse \"{}\" it is not a number that can be parsed.",
                    s
                ),
                ParsePointError::Not2Dimensional(num) => {
                    format!("What you inputted was {} dimensional.", num)
                }
                ParsePointError::NonNumeric => "That contains a invalid character.".to_string(),
            }))
            .get()
    );
    dont_disappear::enter_to_continue::default();
}
