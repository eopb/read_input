use {
    crate::prompt_msg::PromptMsg,
    std::{io, io::Write, str::FromStr, string::ToString},
};

pub(crate) type TestFunc<T> = Fn(&T) -> bool;

fn try_flush() {
    io::stdout().flush().unwrap_or(())
}

fn input_str() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

pub(crate) fn parse_input<T: FromStr>(
    input: String,
    err: &str,
    tests: &[(Box<TestFunc<T>>, Option<String>)],
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
) -> Result<T, String> {
    match T::from_str(&input.trim()) {
        Ok(value) => {
            for (test, test_err) in tests {
                if !test(&value) {
                    return Err(test_err.clone().unwrap_or_else(|| err.to_string()));
                }
            }
            Ok(value)
        }
        Err(error) => Err(err_pass(&error).unwrap_or_else(|| err.to_string())),
    }
}

pub(crate) fn read_input<T: FromStr>(
    prompt: &PromptMsg,
    err: &str,
    default: Option<T>,
    tests: &[(Box<TestFunc<T>>, Option<String>)],
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
) -> T {
    print!("{}", prompt.msg);
    try_flush();

    loop {
        let input = input_str();

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
