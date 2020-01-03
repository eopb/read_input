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
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
) -> io::Result<T> {
    // Flush only when possible.
    fn try_flush() {
        io::stdout().flush().unwrap_or(())
    }

    fn input_as_string() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input)
    }

    print!("{}", prompt.msg);
    try_flush();

    loop {
        let input = input_as_string()?;

        if input.trim().is_empty() {
            if let Some(x) = default {
                return Ok(x);
            }
        };

        match parse_input(input, err, tests, err_pass) {
            Ok(v) => return Ok(v),
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
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
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
