use nom::IResult;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::character::complete::space0;
use crate::time_math::types::{Duration, Operator};
use crate::time_math::types::Operator::{Plus, Minus};
use const_parser::literal;
use nom::combinator::{all_consuming, map};
use nom::sequence::tuple;
use nom::multi::many0;

mod types;
mod const_parser;
mod parser_helper;

#[cfg(test)]
mod tests;

fn operator(i: &str) -> IResult<&str, Operator> {
    alt((
        map(tuple((space0, tag("+"), space0)), |_| Plus),
        map(tuple((space0, tag("-"), space0)), |_| Minus),
    ))(i)
}

fn plus_minus(i: &str) -> IResult<&str, Duration> {
    let (i, first) = literal(i)?;
    let (i, summands) = many0(tuple((operator, literal)))(i)?;
    let summands = summands.iter().map(|(op, d)| {
        match op {
            Plus => *d,
            Minus => -d,
        }
    });

    Ok((i, first + summands.sum::<Duration>()))
}

fn expr(i: &str) -> IResult<&str, Duration> {
    plus_minus(i)
}

pub fn exec(i: &str) -> Result<Duration, nom::Err<nom::error::Error<&str>>> {
    match all_consuming(expr)(i) {
        Ok((_, expr)) => Ok(expr),
        Err(e) => Err(e)
    }
}