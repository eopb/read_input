//! Go the the [readme](https://crates.io/crates/read_input) file for documentation.

// This lint asks for default when new takes type parameter.
#![allow(clippy::new_without_default)]
// `impl ToString` is better than `&impl ToString`. Clippy is not ready for impl trait.
#![allow(clippy::needless_pass_by_value)]

pub mod prelude;
pub mod shortcut;

use std::io;
use std::io::Write;
use std::str::FromStr;
use std::string::ToString;

const DEFAULT_ERR: &str = "That value does not pass. Please try again";

struct PromptMsg {
    msg: String,
    repeat: bool,
}

impl PromptMsg {
    fn new() -> Self {
        Self {
            msg: String::new(),
            repeat: false,
        }
    }
    fn from_str(s: impl ToString) -> Self {
        Self {
            msg: s.to_string(),
            repeat: false,
        }
    }
    fn repeat_from_str(s: impl ToString) -> Self {
        Self {
            msg: s.to_string(),
            repeat: true,
        }
    }
}

type TestFunc<T> = Fn(&T) -> bool;

/// `InputBuilder` is a 'builder' used to store the settings that are used to fetch input.
pub struct InputBuilder<T: FromStr> {
    msg: PromptMsg,
    err: String,
    test: Vec<(Box<TestFunc<T>>, Option<String>)>,
    err_match: Box<dyn Fn(&T::Err) -> Option<String>>,
}

pub struct InputBuilderOnce<T: FromStr> {
    builder: InputBuilder<T>,
    default: Option<T>,
}

impl<T: FromStr> InputBuilder<T> {
    /// Creates a new instance of `InputBuilder` with default settings.
    pub fn new() -> Self {
        InputBuilder {
            msg: PromptMsg::new(),
            err: DEFAULT_ERR.to_string(),
            test: Vec::new(),
            err_match: Box::new(|_| None),
        }
    }
    /// Changes or adds a prompt message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn msg(self, msg: impl ToString) -> Self {
        InputBuilder {
            msg: PromptMsg::from_str(msg),
            ..self
        }
    }
    /// Changes or adds a prompt message and makes it repeat. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn repeat_msg(self, msg: impl ToString) -> Self {
        InputBuilder {
            msg: PromptMsg::repeat_from_str(msg),
            ..self
        }
    }
    /// Changes fallback error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn err(self, err: impl ToString) -> Self {
        InputBuilder {
            err: err.to_string(),
            ..self
        }
    }
    /// Changes or adds a default input value. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn default(self, default: T) -> InputBuilderOnce<T> {
        InputBuilderOnce {
            builder: self,
            default: Some(default),
        }
    }
    fn test<F: 'static + Fn(&T) -> bool>(self, test: F, err: Option<String>) -> Self {
        InputBuilder {
            test: {
                let mut x = self.test;
                x.push((Box::new(test), err));
                x
            },
            ..self
        }
    }
    /// Adds a validation check on input. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.test(test, None)
    }
    /// Adds a validation check on input with custom error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        self.test(test, Some(err.to_string()))
    }
    /// Removes all validation checks made by `.add_test()` and `.add_err_test()`.
    pub fn clear_tests(self) -> Self {
        InputBuilder {
            test: Vec::new(),
            ..self
        }
    }
    /// Used specify custom error messages that depend on the errors produced by `from_str()`. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        InputBuilder {
            err_match: Box::new(err_match),
            ..self
        }
    }
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn get(&self) -> T {
        read_input::<T>(&self.msg, &self.err, None, &self.test, &*self.err_match)
    }
}

impl<T: FromStr> InputBuilderOnce<T> {
    pub fn msg(self, msg: impl ToString) -> Self {
        Self {
            builder: self.builder.msg(msg),
            ..self
        }
    }
    pub fn repeat_msg(self, msg: impl ToString) -> Self {
        Self {
            builder: self.builder.repeat_msg(msg),
            ..self
        }
    }
    pub fn err(self, err: impl ToString) -> Self {
        Self {
            builder: self.builder.err(err),
            ..self
        }
    }
    pub fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        Self {
            builder: self.builder.add_test(test),
            ..self
        }
    }
    pub fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        Self {
            builder: self.builder.add_err_test(test, err),
            ..self
        }
    }
    pub fn clear_tests(self) -> Self {
        Self {
            builder: self.builder.clear_tests(),
            ..self
        }
    }
    pub fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        Self {
            builder: self.builder.err_match(err_match),
            ..self
        }
    }
    pub fn get(self) -> T {
        read_input::<T>(
            &self.builder.msg,
            &self.builder.err,
            self.default,
            &self.builder.test,
            &*self.builder.err_match,
        )
    }
}

/// Creates a new instance of `InputBuilder` with default settings. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn input_new<T: FromStr>() -> InputBuilder<T> {
    InputBuilder::new()
}

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

fn read_input<T: FromStr>(
    prompt: &PromptMsg,
    err: &str,
    default: Option<T>,
    test: &[(Box<TestFunc<T>>, Option<String>)],
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
) -> T {
    print!("{}", prompt.msg);
    try_flush();

    let mut input = input_str();

    if input.trim().is_empty() {
        if let Some(x) = default {
            return x;
        }
    };

    loop {
        match T::from_str(&input.trim()) {
            Ok(value) => {
                let mut test_err = None;
                let passes_test = test.iter().all(|f| {
                    if f.0(&value) {
                        true
                    } else {
                        test_err = Some(f.1.clone().unwrap_or_else(|| err.to_string()));
                        false
                    }
                });
                if passes_test {
                    return value;
                } else {
                    println!("{}", test_err.unwrap_or_else(|| err.to_string()));
                }
            }
            Err(error) => {
                println!("{}", (err_pass(&error)).unwrap_or_else(|| err.to_string()));
            }
        }

        if prompt.repeat {
            print!("{}", prompt.msg);
            try_flush();
        };

        input = input_str();
    }
}
