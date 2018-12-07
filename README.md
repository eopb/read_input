# Read Input
A simple tool that asks for data until the data is valid.

[![pipeline status](https://gitlab.com/efunb/read_input/badges/stable/pipeline.svg)](https://gitlab.com/efunb/read_input/commits/stable)
[![License](https://img.shields.io/crates/l/read_input.svg)](https://crates.io/crates/read_input)
[![Latest version](https://img.shields.io/crates/v/read_input.svg)](https://crates.io/crates/read_input)
[![Latest Docs](https://docs.rs/read_input/badge.svg)](https://docs.rs/read_input/)
[![downloads-badge](https://img.shields.io/crates/d/read_input.svg)](https://crates.io/crates/read_input)

## Help

If you run into any issues or need help with using `read_input` in your project please email [incoming+efunb/read_input@incoming.gitlab.com](mailto:incoming+efunb/read_input@incoming.gitlab.com)

## Why you need it

When writing programs you will often need to take input from the user. If the user inputs invalid information the program needs to ask them again. Having to make this loop distracts from the useful logic in your program.

`read_input` attempts to make it easy to get input from the user without having to think about converting types.

## How to use

Add 
```toml
read_input = "0.7"
```
to your `cargo.toml` under `[dependencies]`
and add
```rust 
extern crate read_input;

use read_input::prelude::*;
```
to your main file.

---

You can get input with.

```rust
input_new::<Type>().get()
```

Where `Type` is the type you want. You can use all types that implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html). This currently includes the standard library types `isize`, `usize`, `i8`, `u8`, `i16`, `u16`, `f32`, `i32`, `u32`, `f64`, `i64`, `u64`, `i128`, `u128`, `char`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddrV4`, `SocketAddrV6` and `String`. Many crates also implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) for their types.

For example, if you want to assign a valid unsigned 32bit value to a variable called `input`, you could write.

```rust
let input = input_new::<u32>().get();
```

Rust can often work out the type. When this is the case so you can skip explicitly stating the type.

```rust
input_new().get()
```

### Input message

Custom messages are written on the same line as input and are specified with `.msg()`. Note that the type annotations can been moved from the `input_new()` function to the variable name when assigning input to variables.

```rust
let username: String = input_new().msg("Please input your name: ").get();
```

Alternatively `.repeat_msg()` can be used. Messages specified with `.repeat_msg()` will be repeated every time input is requested. You should try `.msg()` and `.repeat_msg()` to find what style works best for you.

```rust
let username: String = input_new().repeat_msg("Please input your name: ").get();
```

If you don't like having the message on the same line as input you can force input on to a new line by adding `\n` to the end of the message.

```rust
let username: String = input_new().repeat_msg("Please input your name: \n").get();
```

### Default values

If the user presses enter before typing anything `.get()` will return a default value when `.default()` is used. Note the absence type annotations. Rust can infer the type by looking at the type of value used in `.default()`.

```rust
let input = input_new().msg("Please input pi: ").default(3.141).get();
```

### Change error message

The default error message is "That value does not pass. Please try again". You can change the error message with `.err()`. For example.

```rust
let input = input_new::<u32>()
    .msg("Please input a positive number: ")
    .err("That does not look like a positive number. Please try again")
    .get();
```

### Add Checks

You can add your own checks to ensure the value meets your criteria. If you want a integer between 4 and 9 you could write.

```rust
let input = input_new().add_test(|x| 4 < *x && *x < 9).get();
```

In the same style you can specify custom test errors and multiple tests. If you want a value between 4 and 9 that is not 6 you could write.

```rust
let input = input_new()
    .msg("Please input a number between 4 and 9 that is not 6: ")
    .add_test(|x| 4 < *x && *x < 9)
    .add_err_test(
        |x| *x != 6,
        "That value is 6! I dont want 6. Please try again"
    )
    .err("That does not look like a number between 4 and 9. Please try again")
    .get();
```

### Match errors

You can specify custom error messages that depend on the errors produced by `from_str()` with `.err_match()`.

Here is an extract from the [`point_input`](https://gitlab.com/efunb/read_input/blob/stable/examples/point_input.rs) example showing this in practice.

```rust
let point = input_new::<Point>()
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
let input = input_new::<i16>()
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

The `with_description` function can be used if [`Err`](https://doc.rust-lang.org/std/str/trait.FromStr.html#associatedtype.Err) associated type for the [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) implementation for the type you are using implements [`Error`](https://doc.rust-lang.org/std/error/trait.Error.html). This can give quick error messages.

You will have to bring into scope with 

```rust
use read_input::shortcut::with_description;
```

and you can use it like this

```rust
let number = input_new::<i16>()
    .err_match(with_description)
    .repeat_msg("Please input a number: ")
    .get();
```

### Shortcut functions

Using `input_new().get()` can be a little verbose in simple situations. The functions `simple_input()` and `valid_input()` can make things simpler.

You can bring them into scope so that you can use them with

```rust
use read_input::shortcut::{simple_input, valid_input};
```

`simple_input()` is the same as `input_new().get()`.

`valid_input(|x| 4 < *x && *x < 9)` is the same as `input_new().add_test(|x| 4 < *x && *x < 9).get()`.

### `input_new_d`

`input_new_d()` works like `input_new()` but uses the default input settings that are specified by the `DefaultBuilderSettings` trait.

You can bring it into scope so that you can use them with

```rust
use read_input::shortcut::input_new_d;
```

and it can be used like `input_new()`

```rust
let input: u32 = input_new_d().get()
```

### Using `match` with checked input.

It is common to use match on values produced by input. For example if `.add_test()` or `valid_input()` is used on an integer, `match` would need to have branches for all possible integers even though the range of possible valid inputs may be quite small. In these cases, an unreachable wildcard can be used.

```rust
match valid_input(|x| 2 <= *x && *x <= 4) {
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

## More complex examples


| Example                                                                                                    | Download                                                                                                                                                                                                                                                                                                                                           | Description                                                                                                                                       |
| :--------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | ------------------------------------------------------------------------------------------------------------------------------------------------: |
| [`simple_guessing_game`](https://gitlab.com/efunb/read_input/blob/stable/examples/simple_guessing_game.rs) | [Windows](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/simple_guessing_game.exe?job=windows-optimized) [Linux](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/simple_guessing_game?job=linux-optimized) [Source](https://gitlab.com/efunb/read_input/blob/stable/examples/simple_guessing_game.rs) | The guessing game form the rust book made to use `read_input`.                                                                                    |
| [`guessing_game`](https://gitlab.com/efunb/read_input/blob/stable/examples/guessing_game.rs)               | [Windows](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/guessing_game.exe?job=windows-optimized) [Linux](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/guessing_game?job=linux-optimized) [Source](https://gitlab.com/efunb/read_input/blob/stable/examples/guessing_game.rs)                      | The guessing game form the rust book made to use `read_input` + some extra features.                                                              |
| [`how_long_until`](https://gitlab.com/efunb/read_input/blob/stable/examples/how_long_until.rs)             | [Windows](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/how_long_until.exe?job=windows-optimized) [Linux](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/how_long_until?job=linux-optimized) [Source](https://gitlab.com/efunb/read_input/blob/stable/examples/how_long_until.rs)                   | Program that uses `read_input` with the crate [`chrono`](https://crates.io/crates/chrono).                                                        |
| [`point_input`](https://gitlab.com/efunb/read_input/blob/stable/examples/point_input.rs)                   | [Windows](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/point_input.exe?job=windows-optimized) [Linux](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/point_input?job=linux-optimized) [Source](https://gitlab.com/efunb/read_input/blob/stable/examples/point_input.rs)                            | Program written to show the use of the `err_match()` method.                                                                                      |
| [`url`](https://gitlab.com/efunb/read_input/blob/stable/examples/url.rs)                                   | [Windows](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/url.exe?job=windows-optimized) [Linux](https://gitlab.com/efunb/read_input/-/jobs/artifacts/stable/raw/files/url?job=linux-optimized) [Source](https://gitlab.com/efunb/read_input/blob/stable/examples/url.rs)                                                    | Program that lets users input URLs with the [`url`](https://crates.io/crates/url) crate and prints helpful errors when invalid urls are inputted. |

## Docs

[API Documentation](https://docs.rs/read_input/)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/read_input).**