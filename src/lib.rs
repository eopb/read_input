//! Go the the [readme](https://crates.io/crates/read_input) file for documentation.

#![deny(clippy::pedantic, missing_docs)]
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

/// Trait for comman types that store input settings.
pub trait InputBuild<T: FromStr> {
    /// Changes or adds a prompt message that gets printed once when input if fetched.
    fn msg(self, msg: impl ToString) -> Self;
    /// Changes or adds a prompt message and that is repeated each time input is requested.
    fn repeat_msg(self, msg: impl ToString) -> Self;
    /// Changes fallback error message.
    fn err(self, err: impl ToString) -> Self;
    /// Adds a validation check on input.
    fn add_test<F: Fn(&T) -> bool + 'static>(self, test: F) -> Self;
    /// Adds a validation check on input with a custom error message printed when the test
    /// fails.
    fn add_err_test<F>(self, test: F, err: impl ToString) -> Self
    where
        F: Fn(&T) -> bool + 'static;
    /// Removes all validation checks made by `.add_test()`, `.add_err_test()`,
    /// `.inside()` and `.inside_err()`.
    fn clear_tests(self) -> Self;
    /// Used specify custom error messages that depend on the errors produced by `from_str()`.
    fn err_match<F>(self, err_match: F) -> Self
    where
        F: Fn(&T::Err) -> Option<String> + 'static;
    /// Ensures that input is within a range, array or vector.
    fn inside<U: InsideFunc<T>>(self, constraint: U) -> Self;
    /// Ensures that input is within a range, array or vector with a custom error message
    /// printed when input fails.
    fn inside_err<U: InsideFunc<T>>(self, constraint: U, err: impl ToString) -> Self;
    /// Toggles whether a prompt message gets printed once or each time input is requested.
    fn toggle_msg_repeat(self) -> Self;
}

/// Trait for changing input settings by adding constraints that require `PartialOrd`
/// on the input type.
pub trait InputConstraints<T>: InputBuild<T>
where
    T: FromStr + PartialOrd + 'static,
    Self: Sized,
{
    /// Sets a minimum input value.
    fn min(self, min: T) -> Self {
        self.inside(min..)
    }
    /// Sets a minimum input value with custom error message.
    fn min_err(self, min: T, err: impl ToString) -> Self {
        self.inside_err(min.., err)
    }
    /// Sets a maximum input value.
    fn max(self, max: T) -> Self {
        self.inside(..=max)
    }
    /// Sets a maximum input value with custom error message.
    fn max_err(self, max: T, err: impl ToString) -> Self {
        self.inside_err(..=max, err)
    }
    /// Sets a minimum and maximum input value.
    fn min_max(self, min: T, max: T) -> Self {
        self.inside(min..=max)
    }
    /// Sets a minimum and maximum input value with custom error message.
    fn min_max_err(self, min: T, max: T, err: impl ToString) -> Self {
        self.inside_err(min..=max, err)
    }
    /// Sets a restricted input value.
    fn not(self, this: T) -> Self {
        self.add_test(move |x: &T| *x != this)
    }
    /// Sets a restricted input value with custom error message.
    fn not_err(self, this: T, err: impl ToString) -> Self {
        self.add_err_test(move |x: &T| *x != this, err)
    }
}

#[derive(Clone)]
pub(crate) struct Prompt {
    pub msg: String,
    pub repeat: bool,
}

#[derive(Clone)]
pub(crate) struct Test<T> {
    pub func: Rc<Fn(&T) -> bool>,
    pub err: Option<String>,
}

/// 'builder' used to store the settings that are used to fetch input.
///
/// `.get()` method only takes these settings by reference so can be called multiple times.
///
/// This type does not have support for default input value.
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
    /// 'gets' the input form the user.
    pub fn get(&self) -> T {
        read_input::<T>(&self.msg, &self.err, None, &self.tests, &*self.err_match)
    }
    /// Changes or adds a default input value.
    pub fn default(self, default: T) -> InputBuilderOnce<T> {
        InputBuilderOnce {
            builder: self,
            default: Some(default),
        }
    }
    // Internal function for adding tests and constraints.
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

impl<T: FromStr> InputBuild<T> for InputBuilder<T> {
    fn msg(mut self, msg: impl ToString) -> Self {
        self.msg = Prompt {
            msg: msg.to_string(),
            repeat: false,
        };
        self
    }
    fn repeat_msg(mut self, msg: impl ToString) -> Self {
        self.msg = Prompt {
            msg: msg.to_string(),
            repeat: true,
        };
        self
    }
    fn err(mut self, err: impl ToString) -> Self {
        self.err = err.to_string();
        self
    }

    fn add_test<F: Fn(&T) -> bool + 'static>(self, test: F) -> Self {
        self.test_err_opt(Rc::new(test), None)
    }
    fn add_err_test<F>(self, test: F, err: impl ToString) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.test_err_opt(Rc::new(test), Some(err.to_string()))
    }
    fn clear_tests(self) -> Self {
        Self {
            tests: Vec::new(),
            ..self
        }
    }
    fn err_match<F>(self, err_match: F) -> Self
    where
        F: Fn(&T::Err) -> Option<String> + 'static,
    {
        Self {
            err_match: Rc::new(err_match),
            ..self
        }
    }
    fn inside<U: InsideFunc<T>>(self, constraint: U) -> Self {
        self.test_err_opt(constraint.contains_func(), None)
    }
    fn inside_err<U: InsideFunc<T>>(self, constraint: U, err: impl ToString) -> Self {
        self.test_err_opt(constraint.contains_func(), Some(err.to_string()))
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

impl<T: FromStr + Clone> Clone for InputBuilder<T> {
    fn clone(&self) -> Self {
        Self {
            msg: self.msg.clone(),
            err: self.err.clone(),
            tests: self.tests.clone(),
            err_match: self.err_match.clone(),
        }
    }
}

/// 'builder' used to store the settings that are used to fetch input.
///
/// `.get()` method takes ownership of the settings so can be called only once without cloning.
///
/// This type has support for default input value.
pub struct InputBuilderOnce<T: FromStr> {
    builder: InputBuilder<T>,
    default: Option<T>,
}

impl<T: FromStr> InputBuilderOnce<T> {
    /// 'gets' the input form the user.
    pub fn get(self) -> T {
        read_input::<T>(
            &self.builder.msg,
            &self.builder.err,
            self.default,
            &self.builder.tests,
            &*self.builder.err_match,
        )
    }
    // Function that makes it less verbose to change settings of internal `InputBuilder`.
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

impl<T: FromStr> InputBuild<T> for InputBuilderOnce<T> {
    fn msg(self, msg: impl ToString) -> Self {
        self.internal(|x| x.msg(msg))
    }
    fn repeat_msg(self, msg: impl ToString) -> Self {
        self.internal(|x| x.repeat_msg(msg))
    }
    fn err(self, err: impl ToString) -> Self {
        self.internal(|x| x.err(err))
    }
    fn add_test<F: Fn(&T) -> bool + 'static>(self, test: F) -> Self {
        self.internal(|x| x.add_test(test))
    }
    fn add_err_test<F>(self, test: F, err: impl ToString) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.internal(|x| x.add_err_test(test, err))
    }
    fn clear_tests(self) -> Self {
        self.internal(|x| x.clear_tests())
    }
    fn err_match<F>(self, err_match: F) -> Self
    where
        F: Fn(&T::Err) -> Option<String> + 'static,
    {
        self.internal(|x| x.err_match(err_match))
    }
    fn inside<U: InsideFunc<T>>(self, constraint: U) -> Self {
        self.internal(|x| x.inside(constraint))
    }
    fn inside_err<U: InsideFunc<T>>(self, constraint: U, err: impl ToString) -> Self {
        self.internal(|x| x.inside_err(constraint, err))
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
