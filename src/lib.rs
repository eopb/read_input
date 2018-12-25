#![deny(clippy::pedantic)]
//! Go the the [readme](https://crates.io/crates/read_input) file for documentation.

// `impl ToString` is better than `&impl ToString`. Clippy is not ready for impl trait.
#![allow(clippy::needless_pass_by_value)]

mod core;
mod is_in_func;
pub mod prelude;
mod prompt_msg;
pub mod shortcut;
#[cfg(test)]
mod tests;

use {
    crate::{
        core::{read_input, TestFunc},
        is_in_func::IsInFunc,
        prompt_msg::PromptMsg,
    },
    std::{cmp::PartialOrd, str::FromStr, string::ToString},
};

const DEFAULT_ERR: &str = "That value does not pass. Please try again";

pub trait InputBuild<T: FromStr> {
    /// Changes or adds a prompt message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn msg(self, msg: impl ToString) -> Self;
    /// Changes or adds a prompt message and makes it repeat. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn repeat_msg(self, msg: impl ToString) -> Self;
    /// Changes fallback error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn err(self, err: impl ToString) -> Self;
    /// Adds a validation check on input. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self;
    /// Adds a validation check on input with custom error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self;
    /// Removes all validation checks made by `.add_test()` and `.add_err_test()`.
    fn clear_tests(self) -> Self;
    /// Used specify custom error messages that depend on the errors produced by `from_str()`. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self;
    fn inside<U: IsInFunc<T>>(self, is: U) -> Self;
    fn inside_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self;
}

pub trait InputConstraints<T>: InputBuild<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
    Self: std::marker::Sized,
{
    fn min(self, min: T) -> Self {
        self.inside(min..)
    }
    fn max(self, max: T) -> Self {
        self.inside(..=max)
    }
    fn min_max(self, min: T, max: T) -> Self {
        self.inside(min..=max)
    }
    fn min_err(self, min: T, err: impl ToString) -> Self {
        self.inside_err(min.., err)
    }
    fn max_err(self, max: T, err: impl ToString) -> Self {
        self.inside_err(..=max, err)
    }
    fn min_max_err(self, min: T, max: T, err: impl ToString) -> Self {
        self.inside_err(min..=max, err)
    }
}

/// `InputBuilder` is a 'builder' used to store the settings that are used to fetch input.
pub struct InputBuilder<T: FromStr> {
    msg: PromptMsg,
    err: String,
    tests: Vec<(Box<TestFunc<T>>, Option<String>)>,
    err_match: Box<dyn Fn(&T::Err) -> Option<String>>,
}

impl<T: FromStr> InputBuilder<T> {
    /// Creates a new instance of `InputBuilder` with default settings.
    pub fn new() -> Self {
        Self {
            msg: PromptMsg::new(),
            err: DEFAULT_ERR.to_string(),
            tests: Vec::new(),
            err_match: Box::new(|_| None),
        }
    }
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    pub fn get(&self) -> T {
        read_input::<T>(&self.msg, &self.err, None, &self.tests, &*self.err_match)
    }
    /// Changes or adds a default input value. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    pub fn default(self, default: T) -> InputBuilderOnce<T> {
        InputBuilderOnce {
            builder: self,
            default: Some(default),
        }
    }
    fn inside_err_opt<U: IsInFunc<T>>(self, is: U, err: Option<String>) -> Self {
        Self {
            tests: {
                let mut x = self.tests;
                x.push((is.contains_func(), err));
                x
            },
            ..self
        }
    }
}

impl<T: FromStr + 'static> InputBuild<T> for InputBuilder<T> {
    fn msg(self, msg: impl ToString) -> Self {
        Self {
            msg: PromptMsg::from_str(msg),
            ..self
        }
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        Self {
            msg: PromptMsg::repeat_from_str(msg),
            ..self
        }
    }
    fn err(self, err: impl ToString) -> Self {
        Self {
            err: err.to_string(),
            ..self
        }
    }

    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.inside_err_opt(test, None)
    }
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        self.inside_err_opt(test, Some(err.to_string()))
    }
    fn clear_tests(self) -> Self {
        Self {
            tests: Vec::new(),
            ..self
        }
    }
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        Self {
            err_match: Box::new(err_match),
            ..self
        }
    }
    fn inside<U: IsInFunc<T>>(self, is: U) -> Self {
        self.inside_err_opt(is, None)
    }
    fn inside_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self {
        self.inside_err_opt(is, Some(err.to_string()))
    }
}

impl<T: FromStr> Default for InputBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> InputConstraints<T> for InputBuilder<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
}

pub struct InputBuilderOnce<T: FromStr> {
    builder: InputBuilder<T>,
    default: Option<T>,
}

impl<T: FromStr> InputBuilderOnce<T> {
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
    pub fn get(self) -> T {
        read_input::<T>(
            &self.builder.msg,
            &self.builder.err,
            self.default,
            &self.builder.tests,
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
    fn inside<U: IsInFunc<T>>(self, is: U) -> Self {
        Self {
            builder: self.builder.inside(is),
            ..self
        }
    }
    fn inside_err<U: IsInFunc<T>>(self, is: U, err: impl ToString) -> Self {
        Self {
            builder: self.builder.inside_err(is, err),
            ..self
        }
    }
}

impl<T> InputConstraints<T> for InputBuilderOnce<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
}
