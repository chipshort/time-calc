use nom::IResult;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use std::str::FromStr;

pub fn parse_i32_unit<'a>(unit: &str, i: &'a str) -> IResult<&'a str, i32> {
    let (i, digits) = digit1(i)?;
    let (i, _) = tag(unit)(i)?;

    let value = i32::from_str(digits).unwrap();
    Ok((i, value))
}