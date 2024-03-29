//! ## How to use
//!
//! Add
//! ```toml
//! read_input = "0.8"
//! ```
//! to your `cargo.toml` under `[dependencies]` and add
//! ```rust
//! use read_input::prelude::*;
//! ```
//! to your main file.
//!
//! ---
//!
//! You can get input with.
//!
//! ```no_run
//! # use read_input::prelude::*;
//! # type Type = String;
//! input::<Type>().get();
//! ```
//!
//! Where `Type` is the type you want.
//! You can use all types that implement [`std::str::FromStr`].
//! This currently includes the standard library types [`isize`], [`usize`], [`i8`], [`u8`], [`i16`], [`u16`], [`f32`], [`i32`], [`u32`], [`f64`], [`i64`], [`u64`], [`i128`], [`u128`], [`char`], [`Ipv4Addr`], [`Ipv6Addr`], [`SocketAddrV4`], [`SocketAddrV6`] and [`String`].
//! Many crates also implement [`std::str::FromStr`] for their types.
//!
//! [`Ipv4Addr`]: std::net::Ipv4Addr
//! [`Ipv6Addr`]: std::net::Ipv6Addr
//! [`SocketAddrV4`]: std::net::SocketAddrV4
//! [`SocketAddrV6`]: std::net::SocketAddrV6
//!
//! For example, if you want to assign a valid unsigned 32bit value to a variable called `input`, you could write.
//!
//! ```no_run
//! # use read_input::prelude::*;
//! let input = input::<u32>().get();
//! ```
//!
//! Rust can often work out the type. When this is the case you can skip explicitly stating the type.
//!
//! ```no_run
//! # fn foo() -> String {
//! # use read_input::prelude::*;
//! input().get()
//! # }
//! ```
//!
//! The [`input()`] function uses a common pattern called the builder pattern.
//! Many settings can be use by adding methods between [`input()`] and [`get()`].
//! Available methods can be found on the [InputBuild] Trait;
//!
//! [`input()`]: shortcut::input
//! [`get()`]: InputBuilder::get
//!
//! ## How to use with custom type
//!
//! To use `read_input` with a custom type you need to implement [`std::str::FromStr`] for that type.
//!
//! [Working example](https://github.com/eopb/read_input/blob/master/examples/point_input.rs)

#![deny(missing_docs)]
#![allow(clippy::must_use_candidate)]
// `impl ToString` is better than `&impl ToString`. Clippy is not ready for impl trait.
#![allow(clippy::needless_pass_by_value)]

mod core;
pub mod prelude;
pub mod shortcut;
mod test_generators;
#[cfg(test)]
mod tests;

use crate::{core::read_input, test_generators::InsideFunc};
use std::cell::RefCell;
use std::io::Write;
use std::{cmp::PartialOrd, io, rc::Rc, str::FromStr, string::ToString};

const DEFAULT_ERR: &str = "That value does not pass. Please try again";

/// Trait implemented by [InputBuilder] and [InputBuilderOnce] to standardize input settings.
pub trait InputBuild<T: FromStr> {
    /// Changes or adds a prompt message that gets printed once when input if fetched.
    ///
    /// Custom messages are written on the same line as the input cursor.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let username: String = input().msg("Please input your name: ").get();
    /// ```
    ///
    /// If you wish to fetch input from the next line append a `\n`.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let username: String = input().msg("Please input your name:\n").get();
    /// ```
    fn msg(self, msg: impl ToString) -> Self;
    /// Changes or adds a prompt message and that is repeated each time input is requested.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let username: String = input().repeat_msg("Please input your name: ").get();
    /// ```
    fn repeat_msg(self, msg: impl ToString) -> Self;
    /// Changes fallback error message.
    ///
    /// The default error message is "That value does not pass. Please try again".
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let input = input::<u32>()
    ///     .msg("Please input a positive number: ")
    ///     .err("That does not look like a positive number. Please try again")
    ///     .get();
    /// ```
    fn err(self, err: impl ToString) -> Self;
    /// Adds a validation check on input to ensure the value meets your criteria.
    ///
    /// If you want an integer that is not 6 you could write.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let input = input().add_test(|x: &u8| *x != 6).get();
    /// ```
    /// However for this example it would be better to use [InputConstraints::not]
    fn add_test<F: Fn(&T) -> bool + 'static>(self, test: F) -> Self;
    /// Does the same thing as [InputBuild::err], but with a custom error message printed when the test
    /// fails.
    ///
    ///
    /// If you want a value from 4 to 9 that is not 6 you could write.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let input = input()
    ///     .msg("Please input a number from 4 to 9 that is not 6: ")
    ///     .inside_err(
    ///         4..=9,
    ///         "That does not look like a number from 4 to 9. Please try again"
    ///     )
    ///     .add_err_test(
    ///         |x| *x != 6,
    ///         "That value is 6! I don't want 6. Please try again"
    ///     )
    ///     .err("That does not look like a number. Please try again")
    ///     .get();
    /// ```
    fn add_err_test<F>(self, test: F, err: impl ToString) -> Self
    where
        F: Fn(&T) -> bool + 'static;
    /// Removes all validation checks made by [`InputBuild::add_test`], [`InputBuild::add_err_test`],
    /// [`InputBuild::inside`] and [`InputBuild::inside_err`].
    fn clear_tests(self) -> Self;
    /// Used specify custom error messages that depend on the errors produced by [`FromStr`].
    ///
    /// You can specify custom error messages that depend on the errors produced by [`FromStr`] with [`InputBuild::err_match()`].
    ///
    /// Here is an extract from the [`point_input`](https://github.com/eopb/read_input/blob/master/examples/point_input.rs) example showing this in practice.
    ///
    /// ```ignore
    /// # use read_input::prelude::*;
    /// let point = input::<Point>()
    ///     .repeat_msg("Please input a point in 2D space in the format (x, y): ")
    ///     .err_match(|e| {
    ///         Some(match e {
    ///             ParsePointError::FailedParse(s) => format!(
    ///                 "Failed to parse \"{}\" it is not a number that can be parsed.",
    ///                 s
    ///             ),
    ///             ParsePointError::Not2Dimensional(num) => {
    ///                 format!("What you inputted was {} dimensional.", num)
    ///             }
    ///             ParsePointError::NonNumeric => "That contains a invalid character.".to_string(),
    ///         })
    ///     })
    ///     .get();
    /// ```
    ///
    /// In nightly rust this can also be done with integers with the feature flag `#![feature(int_error_matching)]` shown in the example [`match_num_err`](https://github.com/eopb/read_input/blob/master/examples/match_num_err.rs).
    ///
    /// ```ignore
    /// # use read_input::prelude::*;
    /// use core::num::IntErrorKind::*;
    /// let input = input::<i16>()
    ///     .err_match(|x| {
    ///         Some(
    ///             match x.kind() {
    ///                 Empty => "You did not input any value. Try again.",
    ///                 InvalidDigit => "You typed an invalid digit. Try again using only numbers.",
    ///                 Overflow => "Integer is too large to store. Try again with a smaller number.",
    ///                 Underflow => "Integer is too small to store. Try again with a smaller number.",
    ///                 _ => "That value did not pass for an unexpected reason.",
    ///             }
    ///             .to_string(),
    ///         )
    ///     })
    ///     .repeat_msg("Please input a number: ")
    ///     .get();
    /// ```
    fn err_match<F>(self, err_match: F) -> Self
    where
        F: Fn(&T::Err) -> Option<String> + 'static;
    /// Ensures that input is within a range, array or vector.
    ///
    /// If you want an integer from 4 to 9 you could write.
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let input = input().inside([4, 5, 6, 7, 8, 9]).get();
    /// ```
    ///
    /// or alternatively
    ///
    /// ```no_run
    /// # use read_input::prelude::*;
    /// let input = input().inside(4..=9).get();
    /// ```
    fn inside<U: InsideFunc<T>>(self, constraint: U) -> Self;
    /// Does the same thing as [`InputBuild::inside`], but with a custom error message
    /// printed when input fails.
    fn inside_err<U: InsideFunc<T>>(self, constraint: U, err: impl ToString) -> Self;
    /// Toggles whether a prompt message gets printed once or each time input is requested.
    fn toggle_msg_repeat(self) -> Self;
    /// Send prompts to custom writer instead of stdout
    fn prompting_on(self, prompt_output: RefCell<Box<dyn Write>>) -> Self;
    /// Send prompts to stderr instead of stdout
    fn prompting_on_stderr(self) -> Self;
}

/// A set of validation tests that use `InputBuild::test` under the hood.
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
    pub func: Rc<dyn Fn(&T) -> bool>,
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
    prompt_output: RefCell<Box<dyn Write>>,
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
            prompt_output: RefCell::new(Box::new(std::io::stdout())),
        }
    }
    /// 'gets' the input form the user.
    ///
    /// Panics if unable to read input line.
    pub fn get(&self) -> T {
        self.try_get().expect("Failed to read line")
    }
    /// 'gets' the input form the user.
    ///
    /// # Errors
    ///
    /// Returns `Err` if unable to read input line.
    pub fn try_get(&self) -> io::Result<T> {
        read_input::<T>(
            &self.msg,
            &self.err,
            None,
            &self.tests,
            &*self.err_match,
            &mut (*self.prompt_output.borrow_mut()),
        )
    }
    /// Changes or adds a default input value.
    ///
    /// If the user presses enter before typing anything `.get()` will return a default value when [InputBuilder::default] is used.
    ///
    /// ```rust
    /// # use read_input::prelude::*;
    /// let input = input().msg("Please input pi: ").default(3.141).get();
    /// ```
    pub fn default(self, default: T) -> InputBuilderOnce<T> {
        InputBuilderOnce {
            builder: self,
            default: Some(default),
        }
    }
    // Internal function for adding tests and constraints.
    fn test_err_opt(mut self, func: Rc<dyn Fn(&T) -> bool>, err: Option<String>) -> Self {
        self.tests.push(Test { func, err });
        self
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
    fn clear_tests(mut self) -> Self {
        self.tests = Vec::new();
        self
    }
    fn err_match<F>(mut self, err_match: F) -> Self
    where
        F: Fn(&T::Err) -> Option<String> + 'static,
    {
        self.err_match = Rc::new(err_match);
        self
    }
    fn inside<U: InsideFunc<T>>(self, constraint: U) -> Self {
        self.test_err_opt(constraint.contains_func(), None)
    }
    fn inside_err<U: InsideFunc<T>>(self, constraint: U, err: impl ToString) -> Self {
        self.test_err_opt(constraint.contains_func(), Some(err.to_string()))
    }
    fn toggle_msg_repeat(mut self) -> Self {
        self.msg.repeat = !self.msg.repeat;
        self
    }

    fn prompting_on(mut self, prompt_output: RefCell<Box<dyn Write>>) -> Self {
        self.prompt_output = prompt_output;
        self
    }

    fn prompting_on_stderr(self) -> Self {
        self.prompting_on(RefCell::new(Box::new(std::io::stderr())))
    }
}

impl<T: FromStr + PartialOrd + 'static> InputConstraints<T> for InputBuilder<T> {}

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
            prompt_output: RefCell::new(Box::new(std::io::stdout())),
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
    ///
    /// Panics if unable to read input line.
    pub fn get(self) -> T {
        self.try_get().expect("Failed to read line")
    }
    /// 'gets' the input form the user.
    ///
    /// # Errors
    ///
    /// Returns `Err` if unable to read input line.
    pub fn try_get(self) -> io::Result<T> {
        read_input::<T>(
            &self.builder.msg,
            &self.builder.err,
            self.default,
            &self.builder.tests,
            &*self.builder.err_match,
            &mut (*self.builder.prompt_output.borrow_mut()),
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
        self.internal(InputBuild::clear_tests)
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
        self.internal(InputBuild::toggle_msg_repeat)
    }

    fn prompting_on(self, prompt_output: RefCell<Box<dyn Write>>) -> Self {
        self.internal(|x| x.prompting_on(prompt_output))
    }

    fn prompting_on_stderr(self) -> Self {
        self.internal(|x| x.prompting_on(RefCell::new(Box::new(std::io::stderr()))))
    }
}

impl<T: FromStr + PartialOrd + 'static> InputConstraints<T> for InputBuilderOnce<T> {}

impl<T> Clone for InputBuilderOnce<T>
where
    T: Clone + FromStr,
{
    fn clone(&self) -> Self {
        Self {
            default: self.default.clone(),
            builder: self.builder.clone(),
        }
    }
}
