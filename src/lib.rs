//! Go the the [readme](https://crates.io/crates/read_input) file for documentation.

// `impl ToString` is better than `&impl ToString`. Clippy is not ready for impl trait.
#![allow(clippy::needless_pass_by_value)]

pub mod prelude;
pub mod shortcut;

use std::{
    cmp::PartialOrd,
    io,
    io::Write,
    ops::{Range, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive},
    str::FromStr,
    string::ToString,
};

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

pub trait InputBuild<T: FromStr> {
    /// Changes or adds a prompt message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn msg(self, msg: impl ToString) -> Self;
    /// Changes or adds a prompt message and makes it repeat. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn repeat_msg(self, msg: impl ToString) -> Self;
    /// Changes fallback error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn err(self, err: impl ToString) -> Self;
    /// Adds a validation check on input. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self;
    /// Adds a validation check on input with custom error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self;
    /// Removes all validation checks made by `.add_test()` and `.add_err_test()`.
    fn clear_tests(self) -> Self;
    /// Used specify custom error messages that depend on the errors produced by `from_str()`. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self;
    fn is_in<U: IsInFunc<T>>(self, is: U) -> Self;
    fn is_in_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self;
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
    /// Changes or adds a default input value. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn default(self, default: T) -> InputBuilderOnce<T> {
        InputBuilderOnce {
            builder: self,
            default: Some(default),
        }
    }
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn get(&self) -> T {
        read_input::<T>(&self.msg, &self.err, None, &self.test, &*self.err_match)
    }
    fn is_in_err_opt<U: IsInFunc<T>>(self, is: U, err: Option<String>) -> Self {
        InputBuilder {
            test: {
                let mut x = self.test;
                x.push((is.contains_func(), err));
                x
            },
            ..self
        }
    }
}

impl<T: FromStr + 'static> InputBuild<T> for InputBuilder<T> {
    fn msg(self, msg: impl ToString) -> Self {
        InputBuilder {
            msg: PromptMsg::from_str(msg),
            ..self
        }
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        InputBuilder {
            msg: PromptMsg::repeat_from_str(msg),
            ..self
        }
    }
    fn err(self, err: impl ToString) -> Self {
        InputBuilder {
            err: err.to_string(),
            ..self
        }
    }

    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.is_in_err_opt(test, None)
    }
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        self.is_in_err_opt(test, Some(err.to_string()))
    }
    fn clear_tests(self) -> Self {
        InputBuilder {
            test: Vec::new(),
            ..self
        }
    }
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        InputBuilder {
            err_match: Box::new(err_match),
            ..self
        }
    }
    fn is_in<U: IsInFunc<T>>(self, is: U) -> Self {
        self.is_in_err_opt(is, None)
    }
    fn is_in_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self {
        self.is_in_err_opt(is, Some(err.to_string()))
    }
}

impl<T: FromStr> InputBuilderOnce<T> {
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
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

impl<T: FromStr + 'static> InputBuild<T> for InputBuilderOnce<T> {
    fn msg(self, msg: impl ToString) -> Self {
        Self {
            builder: self.builder.msg(msg),
            ..self
        }
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        Self {
            builder: self.builder.repeat_msg(msg),
            ..self
        }
    }
    fn err(self, err: impl ToString) -> Self {
        Self {
            builder: self.builder.err(err),
            ..self
        }
    }
    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        Self {
            builder: self.builder.add_test(test),
            ..self
        }
    }
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        Self {
            builder: self.builder.add_err_test(test, err),
            ..self
        }
    }
    fn clear_tests(self) -> Self {
        Self {
            builder: self.builder.clear_tests(),
            ..self
        }
    }
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        Self {
            builder: self.builder.err_match(err_match),
            ..self
        }
    }
    fn is_in<U: IsInFunc<T>>(self, is: U) -> Self {
        Self {
            builder: self.builder.is_in(is),
            ..self
        }
    }
    fn is_in_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self {
        Self {
            builder: self.builder.is_in_err(is, err),
            ..self
        }
    }
}

impl<T: FromStr> Default for InputBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait InputConstraints<T>: InputBuild<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
    Self: std::marker::Sized,
{
    fn min(self, min: T) -> Self {
        self.is_in(min..)
    }
    fn max(self, max: T) -> Self {
        self.is_in(..=max)
    }
    fn min_max(self, min: T, max: T) -> Self {
        self.is_in(min..=max)
    }
    fn min_err(self, min: T, err: impl ToString) -> Self {
        self.is_in_err(min.., err)
    }
    fn max_err(self, max: T, err: impl ToString) -> Self {
        self.is_in_err(..=max, err)
    }
    fn min_max_err(self, min: T, max: T, err: impl ToString) -> Self {
        self.is_in_err(min..=max, err)
    }
}

impl<T> InputConstraints<T> for InputBuilder<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
}

impl<T> InputConstraints<T> for InputBuilderOnce<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
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

pub trait IsInFunc<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool>;
}

impl<T: PartialOrd + 'static> IsInFunc<T> for Range<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.start <= x && x < &self.end)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeInclusive<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.start() <= x && x <= self.end())
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeFrom<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.start <= x)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeTo<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.end > x)
    }
}

impl<T: PartialOrd + 'static> IsInFunc<T> for RangeToInclusive<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| &self.end >= x)
    }
}

impl<T: PartialEq + 'static> IsInFunc<T> for Vec<T> {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.contains(x))
    }
}

impl<T> IsInFunc<T> for [T]
where
    Self: Sized,
    T: PartialEq,
    T: 'static,
    T: Sized,
{
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(move |x| self.contains(x))
    }
}

impl<T: 'static, F: Fn(&T) -> bool + 'static> IsInFunc<T> for F {
    fn contains_func(self) -> Box<Fn(&T) -> bool> {
        Box::new(self)
    }
}
