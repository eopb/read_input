pub mod default_builder;

pub use self::default_builder::{input_new_d, DefaultBuilderSettings};

use {
    crate::{is_in_func::IsInFunc, InputBuild, InputBuilder},
    std::{error::Error, str::FromStr},
};

/// Creates a new instance of `InputBuilder` with default settings. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
pub fn input_new<T: FromStr>() -> InputBuilder<T> {
    InputBuilder::new()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/stable/README.md)
pub fn valid_input<T: FromStr + 'static>(test: impl Fn(&T) -> bool + 'static) -> T {
    input_new().add_test(test).get()
}

pub fn input_inside<T: FromStr, U>(is: U) -> T
where
    T: FromStr,
    T: 'static,
    U: IsInFunc<T>,
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
