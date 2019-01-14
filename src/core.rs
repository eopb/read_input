use crate::{Prompt, Test};
use std::{
    io::{self, Write},
    str::FromStr,
    string::ToString,
};

// Core function when running `.get()`.
pub(crate) fn read_input<T: FromStr>(
    prompt: &Prompt,
    err: &str,
    default: Option<T>,
    tests: &[Test<T>],
    err_pass: &Fn(&T::Err) -> Option<String>,
) -> T {
    // Flush only when possible.
    fn try_flush() {
        io::stdout().flush().unwrap_or(())
    }

    fn input_as_string() -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    print!("{}", prompt.msg);
    try_flush();

    loop {
        let input = input_as_string();

        if input.trim().is_empty() {
            if let Some(x) = default {
                return x;
            }
        };

        match parse_input(input, err, tests, err_pass) {
            Ok(v) => return v,
            Err(e) => println!("{}", e),
        };

        if prompt.repeat {
            print!("{}", prompt.msg);
            try_flush();
        };
    }
}

pub(crate) fn parse_input<T: FromStr>(
    input: String,
    err: &str,
    tests: &[Test<T>],
    err_pass: &Fn(&T::Err) -> Option<String>,
) -> Result<T, String> {
    match T::from_str(&input.trim()) {
        Ok(value) => {
            for test in tests {
                if !(test.func)(&value) {
                    return Err(test.err.clone().unwrap_or_else(|| err.to_string()));
                }
            }
            Ok(value)
        }
        Err(error) => Err(err_pass(&error).unwrap_or_else(|| err.to_string())),
    }
}
