use input_new;
use std::error::Error;
use std::str::FromStr;
use InputBuilder;

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn valid_input<T: FromStr>(test: impl Fn(&T) -> bool + 'static) -> T {
    input_new().add_test(test).get()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn simple_input<T: FromStr>() -> T {
    input_new().get()
}

pub fn with_description<T: Error>(x: &T) -> Option<String> {
    Some(format!("Error \"{}\"", (*x).description()))
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

macro_rules! impl_default_builder_for_int {
    ($($t:ty),*) => {$(
    impl DefaultBuilderSettings for $t {
        fn settings() -> InputBuilder<Self> {
            input_new()
                .repeat_msg("Please input a integer: ")
                .err("Only type integers.")
        }
    }
    )*}
}

impl_default_builder_for_int! { i8, i16, i32, i64, i128 }

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

impl_default_builder_for_whole! { u8, u16, u32, u64, u128 }

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

pub fn default_input_set<T: DefaultBuilderSettings>() -> InputBuilder<T> {
    T::settings()
}
