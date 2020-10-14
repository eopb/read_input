//! Collection of functions that make things a little less verbose.
//!
//! Using `input().get()` can be a little verbose in simple situations.

use crate::{test_generators::InsideFunc, InputBuild, InputBuilder};
use std::{error::Error, fmt::Display, str::FromStr};

/// Shortcut function. Fetches input that is validated with a test function.
///
/// `valid_input(|x| 4 < *x && *x < 9)` is the same as `input().add_test(|x| 4 < *x && *x < 9).get()`.
pub fn valid_input<T, F>(test: F) -> T
where
    T: FromStr,
    F: Fn(&T) -> bool + 'static,
{
    input().add_test(test).get()
}

/// `input_inside(..)` is the same as `input().inside(..).get()`.
///
/// Shortcut function. Fetches input that is within a range, array or vector.
pub fn input_inside<T, U>(constraint: U) -> T
where
    T: FromStr,
    U: InsideFunc<T>,
{
    input().inside(constraint).get()
}

/// `simple_input()` is the same as `input().get()`.
///
/// Fetches input that is valid for whatever type needed.
pub fn simple_input<T: FromStr>() -> T {
    input().get()
}

/// Creates a new instance of `InputBuilder` with generic, minimal settings.
pub fn input<T: FromStr>() -> InputBuilder<T> {
    InputBuilder::new()
}

/// [input_d] works like [input] but uses the default input settings that are specified by the [DefaultBuilderSettings] trait.
pub fn input_d<T: DefaultBuilderSettings>() -> InputBuilder<T> {
    T::settings()
}

/// Trait for describing specifically tailored input settings for types.
pub trait DefaultBuilderSettings: FromStr {
    /// Returns tailored `InputBuilder`.
    fn settings() -> InputBuilder<Self>;
}

impl DefaultBuilderSettings for bool {
    fn settings() -> InputBuilder<Self> {
        input()
            .repeat_msg("Please input true or false: ")
            .err("Only type true or false.")
    }
}

impl DefaultBuilderSettings for char {
    fn settings() -> InputBuilder<Self> {
        input()
            .repeat_msg("Please input a character: ")
            .err("Only type a single character.")
    }
}

macro_rules! impl_default_builder_for_int {
    ($($t:ty),*) => {$(
    impl DefaultBuilderSettings for $t {
        fn settings() -> InputBuilder<Self> {
            input()
                .repeat_msg("Please input an integer: ")
                .err("Only type integers.")
        }
    }
    )*}
}

impl_default_builder_for_int! { i8, i16, i32, i64, i128, isize }

macro_rules! impl_default_builder_for_whole {
    ($($t:ty),*) => {$(
    impl DefaultBuilderSettings for $t {
        fn settings() -> InputBuilder<Self> {
            input()
                .repeat_msg("Please input a positive integer: ")
                .err("Only type positive integers.")
        }
    }
    )*}
}

impl_default_builder_for_whole! { u8, u16, u32, u64, u128, usize }

macro_rules! impl_default_builder_for_float {
    ($($t:ty),*) => {$(
    impl DefaultBuilderSettings for $t {
        fn settings() -> InputBuilder<Self> {
            input()
                .repeat_msg("Please input a number: ")
                .err("Only type numbers or decimal point.")
        }
    }
    )*}
}

impl_default_builder_for_float! { f32, f64 }

/// This function can be used if [`Err`](https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err) associated type for the [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) implementation for the type you are using implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html). This can give quick error messages.
/// 
/// 
/// It is for use in [InputBuild::err_match] it like this
/// 
/// ```rust
/// use read_input::shortcut::with_display;
/// let number = input::<i16>()
///     .err_match(with_display)
///     .repeat_msg("Please input a number: ")
///     .get();
/// ```
 pub fn with_display<T: Display>(x: &T) -> Option<String> {
     Some(format!("Error: \"{}\"", x))
}

#[deprecated(
    since = "0.8.4",
    note = "Deprecated due to the depreciation of `std::error::Error::description`. Please use the `with_display` function instead."
)]
#[allow(deprecated)]
/// Produces an error message from an error type. Made for use in `.err_match()`
pub fn with_description<T: Error>(x: &T) -> Option<String> {
    Some(format!("Error: \"{}\"", (*x).description()))
}
