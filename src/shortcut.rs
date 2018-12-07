pub mod default_builder;

pub use self::default_builder::input_new_d;
pub use self::default_builder::DefaultBuilderSettings;

use crate::input_new;
use std::error::Error;
use std::str::FromStr;

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
