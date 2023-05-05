use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "abcdefghi";
    let (remainder, abc) = parse_abc(input)?;
    let (remainder, def_or_ghi) = parse_def_or_ghi(remainder)?;
    println!("first parsed: {abc}; then parsed: {def_or_ghi}; left: {remainder}");
    Ok(())
}

fn parse_abc(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn parse_def_or_ghi(input: &str) -> IResult<&str, &str> {
    alt((tag("def"), tag("ghi")))(input)
}
