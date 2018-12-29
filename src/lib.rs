#![deny(clippy::pedantic)]
//! Go the the [readme](https://crates.io/crates/read_input) file for documentation.

// `impl ToString` is better than `&impl ToString`. Clippy is not ready for impl trait.
#![allow(clippy::needless_pass_by_value)]

mod core;
pub mod prelude;
pub mod shortcut;
mod test_generators;
#[cfg(test)]
mod tests;

use crate::{core::read_input, test_generators::InsideFunc};
use std::{cmp::PartialOrd, rc::Rc, str::FromStr, string::ToString};

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
    fn inside<U: InsideFunc<T>>(self, is: U) -> Self;
    fn inside_err<U: InsideFunc<T>>(self, is: U, err: impl ToString) -> Self;
    fn toggle_msg_repeat(self) -> Self;
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
    fn not(self, this: T) -> Self {
        self.add_test(move |x: &T| *x != this)
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
    fn not_err(self, this: T, err: impl ToString) -> Self {
        self.add_err_test(move |x: &T| *x != this, err)
    }
}

#[derive(Clone)]
pub(crate) struct Prompt {
    pub msg: String,
    pub repeat: bool,
}

pub(crate) struct Test<T> {
    pub func: Rc<Fn(&T) -> bool>,
    pub err: Option<String>,
}

impl<T> Clone for Test<T>
where
    T: Clone,
    T: FromStr,
{
    fn clone(&self) -> Self {
        Self {
            func: self.func.clone(),
            err: self.err.clone(),
        }
    }
}

/// `InputBuilder` is a 'builder' used to store the settings that are used to fetch input.
pub struct InputBuilder<T: FromStr> {
    msg: Prompt,
    err: String,
    tests: Vec<Test<T>>,
    err_match: Rc<dyn Fn(&T::Err) -> Option<String>>,
}

impl<T: FromStr> InputBuilder<T> {
    /// Creates a new instance of `InputBuilder` with default settings.
    pub fn new() -> Self {
        Self {
            msg: Prompt {
                msg: String::new(),
                repeat: false,
            },
            err: DEFAULT_ERR.to_string(),
            tests: Vec::new(),
            err_match: Rc::new(|_| None),
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
    fn test_err_opt(self, func: Rc<Fn(&T) -> bool>, err: Option<String>) -> Self {
        Self {
            tests: {
                let mut x = self.tests;
                x.push(Test { func, err });
                x
            },
            ..self
        }
    }
}

impl<T: FromStr + 'static> InputBuild<T> for InputBuilder<T> {
    fn msg(self, msg: impl ToString) -> Self {
        Self {
            msg: Prompt {
                msg: msg.to_string(),
                repeat: false,
            },
            ..self
        }
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        Self {
            msg: Prompt {
                msg: msg.to_string(),
                repeat: true,
            },
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
        self.test_err_opt(Rc::new(test), None)
    }
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        self.test_err_opt(Rc::new(test), Some(err.to_string()))
    }
    fn clear_tests(self) -> Self {
        Self {
            tests: Vec::new(),
            ..self
        }
    }
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        Self {
            err_match: Rc::new(err_match),
            ..self
        }
    }
    fn inside<U: InsideFunc<T>>(self, is: U) -> Self {
        self.test_err_opt(is.contains_func(), None)
    }
    fn inside_err<U: InsideFunc<T>>(self, is: U, err: impl ToString) -> Self {
        self.test_err_opt(is.contains_func(), Some(err.to_string()))
    }
    fn toggle_msg_repeat(self) -> Self {
        Self {
            msg: Prompt {
                repeat: !self.msg.repeat,
                ..self.msg
            },
            ..self
        }
    }
}

impl<T> InputConstraints<T> for InputBuilder<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
}

impl<T: FromStr> Default for InputBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for InputBuilder<T>
where
    T: Clone,
    T: FromStr,
{
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            err: self.err.clone(),
            tests: self.tests.clone(),
            err_match: self.err_match.clone(),
        }
    }
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
    fn internal<F>(self, with: F) -> Self
    where
        F: FnOnce(InputBuilder<T>) -> InputBuilder<T>,
    {
        Self {
            builder: with(self.builder),
            ..self
        }
    }
}

impl<T: FromStr + 'static> InputBuild<T> for InputBuilderOnce<T> {
    fn msg(self, msg: impl ToString) -> Self {
        self.internal(|x| x.msg(msg))
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        self.internal(|x| x.repeat_msg(msg))
    }
    fn err(self, err: impl ToString) -> Self {
        self.internal(|x| x.err(err))
    }
    fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.internal(|x| x.add_test(test))
    }
    fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: impl ToString) -> Self {
        self.internal(|x| x.add_err_test(test, err))
    }
    fn clear_tests(self) -> Self {
        self.internal(|x| x.clear_tests())
    }
    fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        self.internal(|x| x.err_match(err_match))
    }
    fn inside<U: InsideFunc<T>>(self, is: U) -> Self {
        self.internal(|x| x.inside(is))
    }
    fn inside_err<U: InsideFunc<T>>(self, is: U, err: impl ToString) -> Self {
        self.internal(|x| x.inside_err(is, err))
    }
    fn toggle_msg_repeat(self) -> Self {
        self.internal(|x| x.toggle_msg_repeat())
    }
}

impl<T> InputConstraints<T> for InputBuilderOnce<T>
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
}

impl<T> Clone for InputBuilderOnce<T>
where
    T: Clone,
    T: FromStr,
{
    fn clone(&self) -> Self {
        Self {
            default: self.default.clone(),
            builder: self.builder.clone(),
        }
    }
}
