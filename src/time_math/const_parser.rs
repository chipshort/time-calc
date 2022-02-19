use nom::IResult;
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::bytes::complete::tag;
use nom::number::complete::float;
use nom::combinator::{opt, map};

use crate::time_math::types::{Duration, New2};
use crate::time_math::parser_helper::*;
use nom::sequence::pair;

fn minutes(i: &str) -> IResult<&str, Duration> {
    let (i, minutes) = parse_i32_unit("m", i)?;

    Ok((i, Duration::new(0, minutes)))
}

fn hours(i: &str) -> IResult<&str, Duration> {
    let (i, hours) = parse_i32_unit("h", i)?;

    Ok((i, Duration::new(hours, 0)))
}

fn hours_minutes(i: &str) -> IResult<&str, Duration> {
    let (i, hours) = parse_i32_unit("h", i)?;
    let (i, mins) = digit1(i)?;
    let (i, _) = opt(tag("m"))(i)?; //optional m suffix

    let minutes = mins.parse::<i32>().unwrap();

    Ok((i, Duration::new(hours, minutes)))
}

fn decimal_hours(i: &str) -> IResult<&str, Duration> {
    let (i, h) = float(i)?;
    let (i, _) = tag("h")(i)?; //force h suffix

    let hours = h as i32;
    let minutes = ((h - (hours as f32)) * 60f32) as i32;

    Ok((i, Duration::new(hours, minutes)))
}

pub fn literal(i: &str) -> IResult<&str, Duration> {
    //TODO: allow negative literal
    alt((pos_literal, neg_literal))(i)
}

pub fn pos_literal(i: &str) -> IResult<&str, Duration> {
    alt((minutes, hours_minutes, hours, decimal_hours))(i)
}

pub fn neg_literal(i: &str) -> IResult<&str, Duration> {
    map(pair(tag("-"), pos_literal), |(_, d)| -d)(i)
}