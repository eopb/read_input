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

When writing simple programs you will often need to take input from the user. If the user inputs invalid information the program needs to ask them again. Having to make this loop distracts from the useful logic in your program.

`read_input` attempts to make it easy to get input from the user without having to think about converting types.

## How to use.

Add 
```toml
read_input = "0.3"
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
let input = input_new::<Type>().get();
```

Where `Type` is the type you want. Currently the you can use all types that implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html). This currently includes the standard library types `isize`, `usize`, `i8`, `u8`, `i16`, `u16`, `f32`, `i32`, `u32`, `f64`, `i64`, `u64`, `i128`, `u128`, `char`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddrV4`, `SocketAddrV6` and `String`. Many crates also implement [`std::str::FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) for their types.

For example, if you want a valid unsigned 32bit value you could write.

```rust
let input = input_new::<u32>().get();
```

Often rust can work out the type so you can skip explicitly stating the type.

```rust
let input = input_new().get();
```

You can also add your own checks to ensure the value meets your criteria. If you want a integer between 4 and 9 you could write.

```rust
let input = input_new().test(&|x| 4 < *x && *x < 9, None).get()
```

In the same style you can specify custom error messages, custom test errors and multiple tests. If you want a value between 4 and 9 that is not 6 you could write.

```rust
let input = input_new()
    .msg("Please input a number between 4 and 9 that is not 6: ")
    .test(&|x| 4 < *x && *x < 9, None)
    .test(
        &|x| *x != 6,
        Some("That value is 6! I dont want 6. Please try again")
    )
    .err("That does not look like a number between 4 and 9. Please try again")
    .get()
```

Default values and custom messages are also supported. If the user presses enter before typing anything the program could return a default value. Custom messages are written on the same line as input.

```rust
let input = input_new().msg("Please input pi: ").default(3.141).get();
```

### Shortcut functions

Using `input_new().get()` can be a little verbose in simple situations. The functions `simple_input()` and `valid_input()` can make things simpler.

`simple_input()` is the same as `input_new().get()`.

`valid_input(&|x| 4 < *x && *x < 9)` is the same as `input_new().test(&|x| 4 < *x && *x < 9, None).get()`.

## How to use with custom type.

To use `read_input` with a custom type you need to implement `std::str::FromStr` for that type. [Documentation](https://doc.rust-lang.org/std/str/trait.FromStr.html)

## Docs

[API Documentation](https://docs.rs/read_input/)

## **Warning**

**If you are viewing this from GitHub then this is a read only copy. Please contribute to the GitLab copy [here](https://gitlab.com/efunb/read_input).**