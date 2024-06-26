#![cfg(feature = "alloc")]

use winnow::{
    ascii::{alphanumeric1 as alphanumeric, line_ending as eol},
    combinator::repeat,
    combinator::terminated,
    unpeek, IResult, Parser,
};

pub(crate) fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        eol.parse_peek(input)
    }
}

pub(crate) fn read_line(input: &str) -> IResult<&str, &str> {
    terminated(alphanumeric, unpeek(end_of_line)).parse_peek(input)
}

pub(crate) fn read_lines(input: &str) -> IResult<&str, Vec<&str>> {
    repeat(0.., unpeek(read_line)).parse_peek(input)
}

#[cfg(feature = "alloc")]
#[test]
fn read_lines_test() {
    let res = Ok(("", vec!["Duck", "Dog", "Cow"]));

    assert_eq!(read_lines("Duck\nDog\nCow\n"), res);
    assert_eq!(read_lines("Duck\nDog\nCow"), res);
}
