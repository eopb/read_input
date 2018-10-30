use std::io;
use std::io::Write;

pub struct InputSet<'a, T, F>
where
    F: Fn(&T) -> bool,
{
    msg: Option<&'a str>,
    err: Option<&'a str>,
    default: Option<T>,
    test: Option<F>,
}

impl<'a, T, F> InputSet<'a, T, F>
where
    T: Sized,
    T: ReadInput<F>,
    F: Sized,
    F: Fn(&T) -> bool,
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
    pub fn test(self, test: F) -> Self {
        Self {
            test: Some(test),
            ..self
        }
    }
    pub fn get(self) -> T {
        T::read_input(self.msg, self.err, self.default, self.test)
    }
}

pub trait ReadInput<F>
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
            test: None::<F>,
        }
    }

    fn valid_input(test: F) -> Self {
        Self::read_input(None, None, None, Some(test))
    }
    fn simple_input() -> Self {
        Self::read_input(None, None, None, None::<F>)
    }
    fn read_input(
        msg: Option<&str>,
        err: Option<&str>,
        default: Option<Self>,
        test: Option<F>,
    ) -> Self {
        if let Some(msg) = msg {
            print!("{}", msg);
            io::stdout().flush().expect("could not flush output");
        };
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if let Some(x) = default {
            if input.trim().is_empty() {
                return x;
            }
        }
        if let Some(num) = Self::string_to_self(input) {
            if match &test {
                Some(v) => v(&num),
                None => true,
            } {
                return num;
            }
        };
        loop {
            println!(
                "{}",
                match err {
                    Some(v) => v,
                    None => "That value does not pass please try again",
                }
            );
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if let Some(num) = Self::string_to_self(input) {
                if match &test {
                    Some(v) => v(&num),
                    None => true,
                } {
                    break num;
                }
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
impl<'b> ReadInput<&'b (dyn Fn(&Self) -> bool)> for String {}

macro_rules! impl_read_inputn {
    ($($t:ty),*) => {$(
        impl StringToSelf for $t {
            fn string_to_self(string: String) -> Option<Self> {
                string.trim().parse().ok()
            }
        }
        impl<'b> ReadInput<&'b (dyn Fn(&Self) -> bool)> for $t {}
    )*}
}

impl_read_inputn! { i8, u8, i16, u16,f32, i32, u32, f64, i64, u64, i128, u128, char }
