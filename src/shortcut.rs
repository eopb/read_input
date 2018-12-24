pub mod default_builder;

pub use self::default_builder::input_new_d;
pub use self::default_builder::DefaultBuilderSettings;

use crate::{InputBuild, InputBuilder, InputConstraints};
use std::{cmp::PartialOrd, error::Error, str::FromStr};

/// Creates a new instance of `InputBuilder` with default settings. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn input_new<T: FromStr>() -> InputBuilder<T> {
    InputBuilder::new()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn valid_input<T: FromStr + 'static>(test: impl Fn(&T) -> bool + 'static) -> T {
    input_new().add_test(test).get()
}

pub fn input_inside<T>(min: T, max: T) -> T
where
    T: FromStr,
    T: PartialOrd,
    T: 'static,
{
    input_new().min_max(min, max).get()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn simple_input<T: FromStr>() -> T {
    input_new().get()
}

pub fn with_description<T: Error>(x: &T) -> Option<String> {
    Some(format!("Error \"{}\"", (*x).description()))
}
