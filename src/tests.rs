use crate::{core::parse_input, shortcut::input, InputBuild, InputBuilder};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;
use std::str::FromStr;

fn parse_with_builder<T: FromStr>(builder: InputBuilder<T>, input: String) -> Result<T, String> {
    parse_input(input, &builder.err, &builder.tests, &*builder.err_match)
}

#[test]
fn test_range() {
    assert_eq!(
        parse_with_builder(input().inside(4..9).err("1"), "3".to_string()),
        Err("1".to_string())
    );
    assert_eq!(
        parse_with_builder(input().inside(4..9).err("1"), "4".to_string()),
        Ok(4)
    );
    assert_eq!(
        parse_with_builder(input().inside(4..9).err("1"), "8".to_string()),
        Ok(8)
    );
    assert_eq!(
        parse_with_builder(input().inside(4..9).err("1"), "9".to_string()),
        Err("1".to_string())
    );
}

#[test]
fn test_range_from() {
    assert_eq!(
        parse_with_builder(input().inside(6..).err("1"), "5".to_string()),
        Err("1".to_string())
    );
    assert_eq!(
        parse_with_builder(input().inside(6..).err("1"), "6".to_string()),
        Ok(6)
    );
    assert_eq!(
        parse_with_builder(input().inside(6..).err("1"), "10".to_string()),
        Ok(10)
    );
}

#[test]
fn test_range_inclusive() {
    assert_eq!(
        parse_with_builder(input().inside(4..=9).err("1"), "3".to_string()),
        Err("1".to_string())
    );
    assert_eq!(
        parse_with_builder(input().inside(4..=9).err("1"), "4".to_string()),
        Ok(4)
    );
    assert_eq!(
        parse_with_builder(input().inside(4..=9).err("1"), "8".to_string()),
        Ok(8)
    );
    assert_eq!(
        parse_with_builder(input().inside(4..=9).err("1"), "9".to_string()),
        Ok(9)
    );
    assert_eq!(
        parse_with_builder(input().inside(4..=9).err("1"), "10".to_string()),
        Err("1".to_string())
    );
}

#[test]
fn test_range_to() {
    assert_eq!(
        parse_with_builder(input().inside(..6).err("1"), "2".to_string()),
        Ok(2)
    );
    assert_eq!(
        parse_with_builder(input().inside(..6).err("1"), "5".to_string()),
        Ok(5)
    );
    assert_eq!(
        parse_with_builder(input().inside(..6).err("1"), "6".to_string()),
        Err("1".to_string())
    );
    assert_eq!(
        parse_with_builder(input().inside(..6).err("1"), "7".to_string()),
        Err("1".to_string())
    );
}

#[test]
fn test_range_to_inclusive() {
    assert_eq!(
        parse_with_builder(input().inside(..=6).err("1"), "2".to_string()),
        Ok(2)
    );
    assert_eq!(
        parse_with_builder(input().inside(..=6).err("1"), "5".to_string()),
        Ok(5)
    );
    assert_eq!(
        parse_with_builder(input().inside(..=6).err("1"), "6".to_string()),
        Ok(6)
    );
    assert_eq!(
        parse_with_builder(input().inside(..=6).err("1"), "7".to_string()),
        Err("1".to_string())
    );
}

#[test]
fn test_range_full() {
    assert_eq!(
        parse_with_builder(input().inside(..).err("1"), "2".to_string()),
        Ok(2)
    );
    assert_eq!(
        parse_with_builder(input().inside(..).err("1"), "5".to_string()),
        Ok(5)
    );
}

#[test]
fn test_array() {
    assert_eq!(
        parse_with_builder(input().inside(vec![2, 6, 7]).err("1"), "2".to_string()),
        Ok(2)
    );
    assert_eq!(
        parse_with_builder(input().inside(vec![2, 6, 7]).err("1"), "6".to_string()),
        Ok(6)
    );
    assert_eq!(
        parse_with_builder(input().inside(vec![2, 6, 7]).err("1"), "3".to_string()),
        Err("1".to_string())
    );
}

#[test]
fn writer() {
    input_output_test(
        input().msg("hi "),
        "bruh\n5",
        "hi That value does not pass. Please try again\n",
        5,
    )
}

fn input_output_test<T>(
    input_settings: InputBuilder<T>,
    input_bytes: &'static str,
    expected_prompt: &'static str,
    expected: T,
) where
    T: FromStr + std::fmt::Debug + PartialEq,
{
    let output: Rc<RefCell<Box<(dyn Write + 'static)>>> =
        Rc::new(RefCell::new(Box::new(TestWriter::new(&expected_prompt))));

    let thing = {
        input_settings
            .reading_from(input_bytes.as_bytes())
            .prompting_on(Rc::clone(&output))
            .get()
    };

    assert_eq!(thing, expected);
}

#[derive(Default)]
struct TestWriter<'a> {
    expectation: &'a str,
    buf: Vec<u8>,
}

impl<'a> TestWriter<'a> {
    fn new(expectation: &'a str) -> Self {
        Self {
            expectation,
            buf: Vec::new(),
        }
    }
}

impl<'a> Write for TestWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.append(&mut buf.to_vec());
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> Drop for TestWriter<'a> {
    fn drop(&mut self) {
        assert_eq!(std::str::from_utf8(&self.buf).unwrap(), self.expectation)
    }
}
