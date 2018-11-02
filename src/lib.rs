use std::io;
use std::io::Write;

const DEFAULT_ERR: &str = "That value does not pass please try again";

pub struct InputBuilder<'a, T, F, FE>
where
    T: std::str::FromStr,
    F: Fn(&T) -> bool,
    FE: Fn(&T::Err) -> Option<String>,
{
    msg: Option<&'a str>,
    err: Option<&'a str>,
    default: Option<T>,
    test: Option<Vec<(F, Option<&'a str>)>>,
    err_match: FE,
}

impl<'a, T, F, FE> InputBuilder<'a, T, F, FE>
where
    T: std::str::FromStr,
    F: Fn(&T) -> bool,
    FE: Fn(&T::Err) -> Option<String>,
{
    pub fn msg(self, msg: &'a str) -> Self {
        InputBuilder {
            msg: Some(msg),
            ..self
        }
    }
    pub fn err(self, err: &'a str) -> Self {
        InputBuilder {
            err: Some(err),
            ..self
        }
    }
    pub fn default(self, default: T) -> Self {
        InputBuilder {
            default: Some(default),
            ..self
        }
    }
    pub fn test(self, test: F, err: Option<&'a str>) -> Self {
        InputBuilder {
            test: Some(match self.test {
                Some(v) => {
                    let mut x = v;
                    x.push((test, err));
                    x
                }
                None => vec![(test, err)],
            }),
            ..self
        }
    }
    pub fn err_match(self, err_match: FE) -> Self {
        InputBuilder { err_match, ..self }
    }
    pub fn get(self) -> T {
        read_input::<T, F>(
            self.msg,
            self.err,
            self.default,
            &self.test,
            &self.err_match,
        )
    }
}

pub fn input_new<'a, T>(
) -> InputBuilder<'a, T, &'a (dyn Fn(&T) -> bool), &'a (dyn Fn(&T::Err) -> Option<String>)>
where
    T: std::str::FromStr,
{
    InputBuilder {
        msg: None,
        err: None,
        default: None,
        test: None::<Vec<(&'a (dyn Fn(&T) -> bool), Option<&'a str>)>>,
        err_match: &|_| None,
    }
}

pub fn valid_input<'a, T>(test: &'a (dyn Fn(&T) -> bool)) -> T
where
    T: std::str::FromStr,
{
    input_new().test(&test, None).get()
}

pub fn simple_input<T>() -> T
where
    T: std::str::FromStr,
{
    input_new().get()
}

fn read_input<'a, T, F>(
    msg: Option<&str>,
    err: Option<&str>,
    default: Option<T>,
    test: &Option<Vec<(F, Option<&str>)>>,
    err_pass: &'a (dyn Fn(&T::Err) -> Option<String>),
) -> T
where
    T: std::str::FromStr,
    F: Fn(&T) -> bool,
{
    if let Some(msg) = msg {
        print!("{}", msg);
        io::stdout().flush().expect("could not flush output");
    };
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
                if test.as_ref().map_or(true, |v| {
                    v.iter().all(|f| {
                        if f.0(&value) {
                            true
                        } else {
                            test_err = Some(f.1.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                            false
                        }
                    })
                }) {
                    return value;
                } else {
                    println!("{}", test_err.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                }
            }
            Err(error) => {
                println!(
                    "{}",
                    err_pass(&error).unwrap_or(err.unwrap_or(DEFAULT_ERR).to_string())
                );
            }
        }
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
