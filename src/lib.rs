//TODO Add option for default values for when the user just presses enter.
//TODO make api more simple (naming is a problem)

use std::io;

pub trait ReadInput
where
    Self: std::marker::Sized,
{
    fn input_read<F: Fn(&Self) -> bool>(test: F, err: &str) -> Self {
        Self::read_input(None, err, |x| test(x))
    }
    fn valid_input<F: Fn(&Self) -> bool>(test: F) -> Self {
        Self::read_input(None, "That value does not pass please try again", |x| {
            test(x)
        })
    }
    fn simple_input() -> Self {
        Self::read_input(None, "That value does not pass please try again", |_| true)
    }
    fn read_input<F: Fn(&Self) -> bool>(msg: Option<&str>, err: &str, test: F) -> Self {
        if let Some(msg) = msg {
            println!("{}", msg);
        };
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if let Some(num) = Self::string_to_self(input) {
                if test(&num) {
                    break num;
                }
            };
            println!("{}", err);
            continue;
        }
    }
    fn string_to_self(string: String) -> Option<Self>;
}

impl ReadInput for String {
    fn string_to_self(string: String) -> Option<Self> {
        Some(string)
    }
}

macro_rules! impl_read_inputn {
    ($($t:ty),*) => {$(
        impl ReadInput for $t {
            fn string_to_self(string: String) -> Option<Self> {
                match string.trim().parse() {Ok(val) => Some(val), Err(_) => None,}
            }
        }
    )*}
}

impl_read_inputn! { i8, u8, i16, u16,f32, i32, u32,f64, i64, u64, i128, u128, char }
