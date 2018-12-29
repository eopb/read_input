//! Collection of functions that make things a little less verbose.

use crate::{test_generators::InsideFunc, InputBuild, InputBuilder};
use std::{error::Error, str::FromStr};

/// Creates a new instance of `InputBuilder` with default settings. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
pub fn input_new<T: FromStr>() -> InputBuilder<T> {
    InputBuilder::new()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
pub fn valid_input<T: FromStr + 'static>(test: impl Fn(&T) -> bool + 'static) -> T {
    input_new().add_test(test).get()
}

pub fn input_inside<T, U>(is: U) -> T
where
    T: FromStr,
    T: 'static,
    U: InsideFunc<T>,
{
    input_new().inside(is).get()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
pub fn simple_input<T: FromStr>() -> T {
    input_new().get()
}

pub fn with_description<T: Error>(x: &T) -> Option<String> {
    Some(format!("Error \"{}\"", (*x).description()))
}

pub fn input_new_d<T: DefaultBuilderSettings>() -> InputBuilder<T> {
    T::settings()
}

pub trait DefaultBuilderSettings: FromStr {
    fn settings() -> InputBuilder<Self>;
}

impl DefaultBuilderSettings for bool {
    fn settings() -> InputBuilder<Self> {
        input_new()
            .repeat_msg("Please input true or false: ")
            .err("Only type true or false.")
    }
}

impl DefaultBuilderSettings for char {
    fn settings() -> InputBuilder<Self> {
        input_new()
            .repeat_msg("Please input a character: ")
            .err("Only type a single character.")
    }
}

macro_rules! impl_default_builder_for_int {
    ($($t:ty),*) => {$(
    impl DefaultBuilderSettings for $t {
        fn settings() -> InputBuilder<Self> {
            input_new()
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
            input_new()
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
            input_new()
                .repeat_msg("Please input a number: ")
                .err("Only type numbers or decimal point.")
        }
    }
    )*}
}

impl_default_builder_for_float! { f32, f64 }
