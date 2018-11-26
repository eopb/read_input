use std::error::Error;
use std::io;
use std::io::Write;
use std::str::FromStr;

const DEFAULT_ERR: &str = "That value does not pass. Please try again";

struct PromptMsg<'a> {
    msg: &'a str,
    repeat: bool,
}

impl<'a> PromptMsg<'a> {
    fn new() -> Self {
        Self {
            msg: "",
            repeat: false,
        }
    }
    fn from_str(s: &'a str) -> Self {
        Self {
            msg: s,
            repeat: false,
        }
    }
    fn repeat_from_str(s: &'a str) -> Self {
        Self {
            msg: s,
            repeat: true,
        }
    }
}

type TestFunc<T> = Fn(&T) -> bool;

/// `InputBuilder` is a 'builder' used to store the settings that are used to fetch input.
pub struct InputBuilder<'a, T: FromStr> {
    msg: PromptMsg<'a>,
    err: &'a str,
    default: Option<T>,
    test: Vec<(Box<TestFunc<T>>, Option<&'a str>)>,
    err_match: Box<dyn Fn(&T::Err) -> Option<String>>,
}

impl<'a, T: FromStr> InputBuilder<'a, T> {
    /// Creates a new instance of `InputBuilder` with default settings.
    pub fn new() -> InputBuilder<'a, T> {
        InputBuilder {
            msg: PromptMsg::new(),
            err: DEFAULT_ERR,
            default: None,
            test: Vec::new(),
            err_match: Box::new(|_| None),
        }
    }
    /// Changes or adds a prompt message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn msg(self, msg: &'a str) -> Self {
        InputBuilder {
            msg: PromptMsg::from_str(msg),
            ..self
        }
    }
    /// Changes or adds a prompt message and makes it repeat. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn repeat_msg(self, msg: &'a str) -> Self {
        InputBuilder {
            msg: PromptMsg::repeat_from_str(msg),
            ..self
        }
    }
    /// Changes fallback error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn err(self, err: &'a str) -> Self {
        InputBuilder { err, ..self }
    }
    /// Changes or adds a default input value. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn default(self, default: T) -> Self {
        InputBuilder {
            default: Some(default),
            ..self
        }
    }
    fn test<F: 'static + Fn(&T) -> bool>(self, test: F, err: Option<&'a str>) -> Self {
        InputBuilder {
            test: {
                let mut x = self.test;
                x.push((Box::new(test), err));
                x
            },
            ..self
        }
    }
    /// Adds a validation check on input. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.test(test, None)
    }
    /// Adds a validation check on input with custom error message. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: &'a str) -> Self {
        self.test(test, Some(err))
    }
    /// Removes all validation checks made by `.add_test()` and `.add_err_test()`.
    pub fn clear_tests(self) -> Self {
        InputBuilder {
            test: Vec::new(),
            ..self
        }
    }
    /// Used specify custom error messages that depend on the errors produced by `from_str()`. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn err_match<F: 'static + Fn(&T::Err) -> Option<String>>(self, err_match: F) -> Self {
        InputBuilder {
            err_match: Box::new(err_match),
            ..self
        }
    }
    /// 'gets' the input form the user. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
    pub fn get(self) -> T {
        read_input::<T>(
            &self.msg,
            self.err,
            self.default,
            &self.test,
            &*self.err_match,
        )
    }
}

pub trait FromStrErrHasDescription<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn err_description(self) -> InputBuilderWithErrDescription<'a, T>;
}

impl<'a, T> FromStrErrHasDescription<'a, T> for InputBuilder<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    fn err_description(self) -> InputBuilderWithErrDescription<'a, T> {
        InputBuilderWithErrDescription {
            builder: self.err_match(|x| Some(format!("Error \"{}\"", x.description()))),
        }
    }
}

pub struct InputBuilderWithErrDescription<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    builder: InputBuilder<'a, T>,
}

impl<'a, T> InputBuilderWithErrDescription<'a, T>
where
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    pub fn msg(self, msg: &'a str) -> Self {
        self.builder.msg(msg).err_description()
    }
    pub fn repeat_msg(self, msg: &'a str) -> Self {
        self.builder.repeat_msg(msg).err_description()
    }
    pub fn err(self, err: &'a str) -> Self {
        self.builder.err(err).err_description()
    }
    pub fn default(self, default: T) -> Self {
        self.builder.default(default).err_description()
    }
    pub fn add_test<F: 'static + Fn(&T) -> bool>(self, test: F) -> Self {
        self.builder.add_test(test).err_description()
    }
    pub fn add_err_test<F: 'static + Fn(&T) -> bool>(self, test: F, err: &'a str) -> Self {
        self.builder.add_err_test(test, err).err_description()
    }
    pub fn clear_tests(self) -> Self {
        self.builder.clear_tests().err_description()
    }
    pub fn get(self) -> T {
        read_input::<T>(
            &self.builder.msg,
            self.builder.err,
            self.builder.default,
            &self.builder.test,
            &*self.builder.err_match,
        )
    }
}

/// Creates a new instance of `InputBuilder` with default settings. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn input_new<'a, T: FromStr>() -> InputBuilder<'a, T> {
    InputBuilder::new()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn valid_input<T: FromStr>(test: impl Fn(&T) -> bool + 'static) -> T {
    input_new::<T>().add_test(test).get()
}

/// Shortcut function. This is documented in the [readme](https://gitlab.com/efunb/read_input/blob/master/README.md)
pub fn simple_input<T: FromStr>() -> T {
    input_new().get()
}

fn read_input<'a, T: FromStr>(
    prompt: &PromptMsg<'a>,
    err: &str,
    default: Option<T>,
    test: &[(Box<TestFunc<T>>, Option<&'a str>)],
    err_pass: &dyn Fn(&T::Err) -> Option<String>,
) -> T {
    print!("{}", prompt.msg);
    io::stdout().flush().expect("could not flush output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim().is_empty() {
        if let Some(x) = default {
            return x;
        }
    };
    loop {
        match T::from_str(&input.trim()) {
            Ok(value) => {
                let mut test_err = None;
                let passes_test = test.iter().all(|f| {
                    if f.0(&value) {
                        true
                    } else {
                        test_err = Some(f.1.unwrap_or(err));
                        false
                    }
                });
                if passes_test {
                    return value;
                } else {
                    println!("{}", test_err.unwrap_or(err));
                }
            }
            Err(error) => {
                println!("{}", err_pass(&error).unwrap_or_else(|| err.to_string()));
            }
        }

        if prompt.repeat {
            print!("{}", prompt.msg);
            io::stdout().flush().expect("could not flush output");
        };

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
