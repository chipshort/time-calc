use std::ops::{Add, Sub, Neg};
use std::fmt::Display;
use std::fmt;
use std::iter::Sum;

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone, PartialEq)]
///A Duration in minutes
pub struct Duration(i32);

pub trait New2<T, U> {
    fn new(a: T, b: U) -> Self;
}

impl New2<i32, i32> for Duration {
    fn new(hours: i32, minutes: i32) -> Self {
        Duration::from((hours, minutes))
    }
}

impl From<(i32, i32)> for Duration {
    fn from((hours, minutes): (i32, i32)) -> Self {
        Duration { 0: hours * 60 + minutes }
    }
}

impl From<i32> for Duration {
    fn from(minutes: i32) -> Self {
        Duration { 0: minutes }
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration::from(self.0 + other.0)
    }
}
impl Add for &Duration {
    type Output = Duration;

    fn add(self, other: &Duration) -> Duration {
        Duration::from(self.0 + other.0)
    }
}
impl Sub for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration::from(self.0 - other.0)
    }
}
impl Sub for &Duration {
    type Output = Duration;

    fn sub(self, other: &Duration) -> Duration {
        Duration::from(self.0 - other.0)
    }
}
impl Neg for Duration {
    type Output = Duration;

    fn neg(self) -> Self::Output {
        Duration::from(-self.0)
    }
}
impl Neg for &Duration {
    type Output = Duration;

    fn neg(self) -> Self::Output {
        Duration::from(-self.0)
    }
}
impl Sum for Duration {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Duration::from(0), Add::add)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.0 / 60;
        let mut minutes = self.0 % 60;

        if minutes < 0 && hours < 0 {
            minutes = -minutes;
        }

        write!(f, "{}h{}m", hours, minutes)
    }
}