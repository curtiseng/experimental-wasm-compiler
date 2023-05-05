use nom::character::complete::{char, multispace0};
use nom::{AsChar, InputIter, InputTakeAtPosition, IResult, Parser, Slice};
use std::error::Error;
use std::ops::RangeFrom;
use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::sequence::delimited;

fn main() -> Result<(), Box<dyn Error>> {
    let wat = r#"
        (module
            (func (export "add") (param i32 i32) (result i32)
                local.get 0
                local.get 1
                i32.add
            ))
    "#;
    let (remainder, ws) = parse_white_space(wat)?;
    println!("remainder: {remainder}, ws: {ws}");
    let (remainder, module) = module(remainder)?;
    println!("remainder: {remainder}, module: {module}");
    let (remainder, func) = func(remainder)?;
    println!("remainder: {remainder}, func: {func}");
    Ok(())
}

fn parse_white_space(input: &str) -> IResult<&str, &str> {
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
    bws(pt(tag("module")))(input)
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
