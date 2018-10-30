# Read Input
A simple tool that asks for data until the data is valid.

[![pipeline status](https://gitlab.com/efunb/read_input/badges/master/pipeline.svg)](https://gitlab.com/efunb/read_input/commits/master)
[![License](https://img.shields.io/crates/l/read_input.svg)](https://crates.io/crates/read_input)
[![Latest version](https://img.shields.io/crates/v/read_input.svg)](https://crates.io/crates/read_input)
[![Latest Docs](https://docs.rs/read_input/badge.svg)](https://docs.rs/read_input/)
[![downloads-badge](https://img.shields.io/crates/d/read_input.svg)](https://crates.io/crates/read_input)

## Help

If you run into any issues or need help with using `read_input` in your project please email [incoming+efunb/read_input@incoming.gitlab.com](mailto:incoming+efunb/read_input@incoming.gitlab.com)

## Why you need it.

When writing simple programs you will often need to take input from the user. If the user inputs invalid information the program needs to try asking them again. Having to make this loop distracts from the useful logic in your program.

Read input attempts to make it easy to get input from the user without having to think about converting types.

## How to use.

Add 
```toml
read_input = "0.1.1"
```
to your `cargo.toml` under `[dependencies]`
and add
```rust 
extern crate read_input;

use read_input::*;
```
to your main file.



You can get input with.

```rust
let input = Type::input_new().get();
```

Where `Type` is the type you want. Currently the types you can use include `i8`, `u8`, `i16`, `u16`, `f32`, `i32`, `u32`, `f64`, `i64`, `u64`, `i128`, `u128`, `char` and `String`.

For example, if you want a valid unsigned 32bit value you could write.

```rust
let input = u32::input_new().get();
```

You can also add your own checks to ensure the value meets your criteria. If you want a signed 16bit value between 4 and 9 you could write.

```rust
let input = i16::input_new().test(&|x| 4 < *x && *x < 9).get();
```

In the same style you can specify custom error messages. If you want a signed 16bit value between 4 and 9 you could write.

```rust
let input = i16::input_new()
    .test(&|x| 4 < *x && *x < 9)
    .err("That does not look like a number between 4 and 9. Please try again")
    .get()
```

Default values are also supported. If the user presses enter before typing anything the program could return a default value.

```rust
let input = f64::input_new().default(3.141).get();
```

## Docs

[API Documentation](https://docs.rs/read_input/)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/read_input).**