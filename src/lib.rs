use std::io;

pub trait ReadInput {
    fn read_input<F: Fn(&Self) -> bool>(msg: &str, err: &str, test: F) -> Self;
}

impl ReadInput for String {
    fn read_input<F: Fn(&Self) -> bool>(msg: &str, err: &str, test: F) -> Self {
        println!("{}", msg);
        let mut input = String::new();
        loop {
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if test(&input) {
                break input;
            }

            println!("{}", err);
            continue;
        }
    }
}

macro_rules! impl_read_inputn {
    ($($t:ty),*) => {$(
    impl ReadInput for $t {
        fn read_input<F: Fn(&Self) -> bool>(msg: &str, err: &str, test: F) -> Self {
            println!("{}", msg);
            let mut input = String::new();
            loop {
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                if let Ok(num) = input.trim().parse() {
                    if test(&num) {
                        break num;
                    }
                };
                println!("{}", err);
                continue;
            }
        }
    }
    )*}
}

impl_read_inputn! { i8, u8, i16, u16,f32, i32, u32,f64, i64, u64, i128, u128 }
