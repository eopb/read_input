# Read Input
A simple CLI tool that asks for user input until the data inputted is valid.

[![License](https://img.shields.io/crates/l/read_input.svg)](https://crates.io/crates/read_input)
[![Latest version](https://img.shields.io/crates/v/read_input.svg)](https://crates.io/crates/read_input)
[![Latest Docs](https://docs.rs/read_input/badge.svg)](https://docs.rs/read_input/)
[![downloads-badge](https://img.shields.io/crates/d/read_input.svg)](https://crates.io/crates/read_input)


## Why you need it

When writing programs you will often need to take input from the user. If the user inputs invalid information the program needs to ask them again. Having to make this loop distracts from the useful logic in your program.

`read_input` attempts to make it easy to get input from the user without having to think about converting types.

## How to use

Add 
```toml
read_input = "0.8"
```
to your `cargo.toml` under `[dependencies]` and add
```rust
use read_input::prelude::*;
```
to your main file.

---

You can get input with.

```rust
input::<Type>().get()
```

Where `Type` is the type you want. You can use all types that implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html). This currently includes the standard library types `isize`, `usize`, `i8`, `u8`, `i16`, `u16`, `f32`, `i32`, `u32`, `f64`, `i64`, `u64`, `i128`, `u128`, `char`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddrV4`, `SocketAddrV6` and `String`. Many crates also implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) for their types.

For example, if you want to assign a valid unsigned 32bit value to a variable called `input`, you could write.

```rust
let input = input::<u32>().get();
```

Rust can often work out the type. When this is the case you can skip explicitly stating the type.

```rust
input().get()
```

### Input message

Custom messages are written on the same line as input and are specified with `.msg()`. Note that the type annotations can been moved from the `input()` function to the variable name when assigning input to variables.

```rust
let username: String = input().msg("Please input your name: ").get();
```

Alternatively `.repeat_msg()` can be used. Messages specified with `.repeat_msg()` will be repeated every time input is requested. You should try `.msg()` and `.repeat_msg()` to find what style works best for you.

```rust
let username: String = input().repeat_msg("Please input your name: ").get();
```

If you don't like having the message on the same line as input you can force input on to a new line by adding `\n` to the end of the message.

```rust
let username: String = input().repeat_msg("Please input your name: \n").get();
```

### Default values

If the user presses enter before typing anything `.get()` will return a default value when `.default()` is used. Note the absence type annotations. Rust can infer the type by looking at the type of value used in `.default()`.

```rust
let input = input().msg("Please input pi: ").default(3.141).get();
```

### Change error message

The default error message is "That value does not pass. Please try again". You can change the error message with `.err()`. For example.

```rust
let input = input::<u32>()
    .msg("Please input a positive number: ")
    .err("That does not look like a positive number. Please try again")
    .get();
```

### Add Checks

You can add your own checks to ensure the value meets your criteria.

If you want an integer that is not 6 you could write.

```rust
let input = input().add_test(|x| *x != 6).get();
```

The `.inside()` method can be used to ensure the inputted value is within a range.

If you want an integer from 4 to 9 you could write.

```rust
let input = input().inside(4..=9).get();
```

`.inside()` can also except an array or vector as well as ranges. `.inside(4..=9)` is the same as `.inside([4, 5, 6, 7, 8, 9])`.

In the same style you can specify custom test errors and multiple checks. Both `.add_test()` and `.inside()` have `.add_err_test()` and `.inside_err()` variants that allow for custom error messages.

If you want a value from 4 to 9 that is not 6 you could write.

```rust
let input = input()
    .msg("Please input a number from 4 to 9 that is not 6: ")
    .inside_err(
        4..=9,
        "That does not look like a number from 4 to 9. Please try again"
    )
    .add_err_test(
        |x| *x != 6,
        "That value is 6! I dont want 6. Please try again"
    )
    .err("That does not look like a number. Please try again")
    .get();
```

##### Other check methods

- Set a minimum value. `.min(minimum_value)`.
- Set a maximum value. `.max(maximum_value)`.
- Set a minimum and maximum value. `.min_max(minimum_value, maximum_value)`.
- Set a restricted value. `.not(unwanted_value)`.
- Set a minimum value with error message. `.min_err(minimum_value, error_message)`.
- Set a maximum value with error message. `.max_err(maximum_value, error_message)`.
- Set a minimum and maximum value with error message. `.min_max_err(minimum_value, maximum_value, error_message)`.
- Sets a restricted value with error message. `.not_err(unwanted_value, error_message)`.

### Match errors

You can specify custom error messages that depend on the errors produced by `from_str()` with `.err_match()`.

Here is an extract from the [`point_input`](https://gitlab.com/efunb/read_input/blob/stable/examples/point_input.rs) example showing this in practice.

```rust
let point = input::<Point>()
    .repeat_msg("Please input a point in 2D space in the format (x, y): ")
    .err_match(|e| {
        Some(match e {
            ParsePointError::FailedParse(s) => format!(
                "Failed to parse \"{}\" it is not a number that can be parsed.",
                s
            ),
            ParsePointError::Not2Dimensional(num) => {
                format!("What you inputted was {} dimensional.", num)
            }
            ParsePointError::NonNumeric => "That contains a invalid character.".to_string(),
        })
    })
    .get();
```

In nightly rust this can also be done with integers with the feature flag `#![feature(int_error_matching)]` shown in the example [`match_num_err`](https://gitlab.com/efunb/read_input/blob/stable/examples/match_num_err.rs).

```rust
use core::num::IntErrorKind::*;
let input = input::<i16>()
    .err_match(|x| {
        Some(
            match x.kind() {
                Empty => "You did not input any value. Try again.",
                InvalidDigit => "You typed an invalid digit. Try again using only numbers.",
                Overflow => "Integer is too large to store. Try again with a smaller number.",
                Underflow => "Integer is too small to store. Try again with a smaller number.",
                _ => "That value did not pass for an unexpected reason.",
            }
            .to_string(),
        )
    })
    .repeat_msg("Please input a number: ")
    .get();
```

The `with_display` function can be used if [`Err`](https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err) associated type for the [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) implementation for the type you are using implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html). This can give quick error messages.

You will have to bring into scope with 

```rust
use read_input::shortcut::with_display;
```

and you can use it like this

```rust
let number = input::<i16>()
    .err_match(with_display)
    .repeat_msg("Please input a number: ")
    .get();
```

### Shortcut functions

Using `input().get()` can be a little verbose in simple situations. The functions `simple_input()` and `valid_input()` can make things simpler.

You can bring them into scope so that you can use them with

```rust
use read_input::shortcut::{input_inside, simple_input, valid_input};
```

`simple_input()` is the same as `input().get()`.

`valid_input(|x| 4 < *x && *x < 9)` is the same as `input().add_test(|x| 4 < *x && *x < 9).get()`.

`input_inside(..)` is the same as `input().inside(..).get()`.

### `input_d`

`input_d()` works like `input()` but uses the default input settings that are specified by the `DefaultBuilderSettings` trait.

You can bring it into scope so that you can use them with

```rust
use read_input::shortcut::input_d;
```

and it can be used like `input()`

```rust
let input: u32 = input_d().get()
```

### Using `match` with checked input.

It is common to use match on values produced by input. For example if `.inside()` or `input_inside()` is used on an integer, `match` would need to have branches for all possible integers even though the range of possible valid inputs may be quite small. In these cases, an unreachable wildcard can be used.

```rust
match input_inside(2..=4) {
    2 => println!("You inputted the number 2"),
    3 => println!("You inputted the number 3"),
    4 => println!("You inputted the number 4"),
    _ => unreachable!(),
}
```

## How to use with custom type

To use `read_input` with a custom type you need to implement `std::str::FromStr` for that type.

[FromStr documentation](https://doc.rust-lang.org/std/str/trait.FromStr.html)

[Working example](https://gitlab.com/efunb/read_input/blob/stable/examples/point_input.rs)

## Docs

[API Documentation](https://docs.rs/read_input/)

