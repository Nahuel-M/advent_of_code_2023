use nom::character::complete::{digit1, multispace0};
use nom::combinator::map_res;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{IResult, Parser};
use std::str::FromStr;

pub fn ws<'a, F, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}
