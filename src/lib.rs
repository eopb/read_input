//TODO Add docs for how to implement.
use std::io;
use std::io::Write;

const DEFAULT_ERR: &str = "That value does not pass please try again";

pub struct InputSet<'a, T, F>
where
    F: Fn(&T) -> bool,
{
    msg: Option<&'a str>,
    err: Option<&'a str>,
    default: Option<T>,
    test: Option<Vec<(F, Option<&'a str>)>>,
}

impl<'a, T, F> InputSet<'a, T, F>
where
    T: Sized,
    T: ReadBuilder<F>,
    F: Sized,
    F: Fn(&T) -> bool,
    F: std::clone::Clone,
{
    pub fn msg(self, msg: &'a str) -> Self {
        Self {
            msg: Some(msg),
            ..self
        }
    }
    pub fn err(self, err: &'a str) -> Self {
        Self {
            err: Some(err),
            ..self
        }
    }
    pub fn default(self, default: T) -> Self {
        Self {
            default: Some(default),
            ..self
        }
    }
    pub fn test(self, test: F, err: Option<&'a str>) -> Self {
        Self {
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
    pub fn get(self) -> T {
        T::read_input(self.msg, self.err, self.default, self.test)
    }
}

pub trait ReadBuilder<F>
where
    Self: Sized,
    Self: StringToSelf,
    F: Sized,
    F: Fn(&Self) -> bool,
{
    fn input_new<'a>() -> InputSet<'a, Self, F> {
        InputSet {
            msg: None,
            err: None,
            default: None,
            test: None::<Vec<(F, Option<&'a str>)>>,
        }
    }

    fn valid_input(test: F) -> Self {
        Self::read_input(None, None, None, Some(vec![(test, None)]))
    }
    fn simple_input() -> Self {
        Self::read_input(None, None, None, None)
    }
    fn read_input(
        msg: Option<&str>,
        err: Option<&str>,
        default: Option<Self>,
        test: Option<Vec<(F, Option<&str>)>>,
    ) -> Self {
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
            } else {
                println!("{}", err.unwrap_or(DEFAULT_ERR));
            }
        } else {
            println!("{}", err.unwrap_or(DEFAULT_ERR));
        }

        if let Some(num) = Self::string_to_self(input) {
            if test.as_ref().map_or(true, |v| {
                v.iter().all(|f| {
                    if f.0(&num) {
                        true
                    } else {
                        println!("{}", f.1.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                        false
                    }
                })
            }) {
                return num;
            } else {
                println!("{}", err.unwrap_or(DEFAULT_ERR));
            }
        };
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if let Some(num) = Self::string_to_self(input) {
                if test.as_ref().map_or(true, |v| {
                    v.iter().all(|f| {
                        if f.0(&num) {
                            true
                        } else {
                            println!("{}", f.1.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                            false
                        }
                    })
                }) {
                    break num;
                }
            } else {
                println!("{}", err.unwrap_or(DEFAULT_ERR));
            };
        }
    }
}

pub trait StringToSelf
where
    Self: Sized,
{
    fn string_to_self(string: String) -> Option<Self>;
}

impl StringToSelf for String {
    fn string_to_self(string: String) -> Option<Self> {
        Some(string)
    }
}
impl<'b> ReadBuilder<&'b (dyn Fn(&Self) -> bool)> for String {}

macro_rules! impl_read_inputn {
    ($($t:ty),*) => {$(
        impl StringToSelf for $t {
            fn string_to_self(string: String) -> Option<Self> {
                string.trim().parse().ok()
            }
        }
        impl<'b> ReadBuilder<&'b (dyn Fn(&Self) -> bool)> for $t {}
    )*}
}

impl_read_inputn! { i8, u8, i16, u16,f32, i32, u32, f64, i64, u64, i128, u128, char }
