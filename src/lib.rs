//TODO Add docs for how to implement.
//TODO Update old readme
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
    T: std::str::FromStr,
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
        read_input::<T, F>(self.msg, self.err, self.default, &self.test)
    }
}

pub fn input_new<'a, T>() -> InputSet<'a, T, &'a (dyn Fn(&T) -> bool)> {
    InputSet {
        msg: None,
        err: None,
        default: None,
        test: None::<Vec<(&'a (dyn Fn(&T) -> bool), Option<&'a str>)>>,
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

fn read_input<T, F>(
    msg: Option<&str>,
    err: Option<&str>,
    default: Option<T>,
    test: &Option<Vec<(F, Option<&str>)>>,
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
        if let Ok(num) = T::from_str(&input.trim()) {
            let mut test_err = None;
            if test.as_ref().map_or(true, |v| {
                v.iter().all(|f| {
                    if f.0(&num) {
                        true
                    } else {
                        test_err = Some(f.1.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
                        false
                    }
                })
            }) {
                return num;
            } else {
                println!("{}", test_err.unwrap_or(err.unwrap_or(DEFAULT_ERR)));
            }
        } else {
            println!("{}", err.unwrap_or(DEFAULT_ERR));
        }
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
