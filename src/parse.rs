use std::ops::RangeFrom;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1, multispace0};
use nom::error::ParseError;
use nom::{AsChar, InputIter, InputTakeAtPosition, IResult, Parser, Slice};
use nom::combinator::map;
use nom::sequence::delimited;

pub fn parse_white_space(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

// Func
// Parse the string "func" which can be between
// zero or more whitespace characters.
pub fn func(input: &str) -> IResult<&str, &str> {
    bws(tag("func"))(input)
}
// Param
// Parse the string "param" which can be between
// zero or more whitespace characters.
pub fn param(input: &str) -> IResult<&str, &str> {
    bws(tag("param"))(input)
}
// Result
// Parse the string "result" which can be between
// zero or more whitespace characters.
pub fn result(input: &str) -> IResult<&str, &str> {
    bws(tag("result"))(input)
}
// Export
// Parse the string "export" which can be between
// zero or more whitespace characters.
pub fn export(input: &str) -> IResult<&str, &str> {
    bws(tag("export"))(input)
}
// Module
// Parse the string "module" which can be between
// zero or more whitespace characters.
pub fn module(input: &str) -> IResult<&str, &str> {
    bws(tag("module"))(input)
}

// U32
// Parse an unsigned 32 bit integer value.
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
    map(digit1, |d: &str| {
        d.parse().expect("Integer format not supported")
    })(input)
}

// Parenthesis
// Apply a given parser between paranthesis.
// Returns the result of the given parser without the parenthesis.
pub fn pt<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
    where
        G: Parser<I, O, E>,
        I: Slice<RangeFrom<usize>> + InputIter,
        <I as InputIter>::Item: AsChar,
{
    delimited(char('('), inner, char(')'))
}
// Between whitespace
// Apply a given parser between whitespaces.
// Returns the result of the given parser without the whitespaces.
pub fn bws<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
    where
        G: Parser<I, O, E>,
        I: InputTakeAtPosition,
        <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func() {
        assert_eq!(func("  \n func"), Ok(("", "func")));
        assert_eq!(func("  \n func \n"), Ok(("", "func")));
        assert_eq!(func("  \n func \n test"), Ok(("test", "func")));
        assert_eq!(func("  \n func \n test \nc"), Ok(("test \nc", "func")));
    }
}