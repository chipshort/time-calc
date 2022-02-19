use crate::time_math::types::{Duration, New2};
use crate::time_math::const_parser::literal;
use crate::time_math::exec;

fn compare_literal(expr: &str, should: &str) -> () {
    let (i, duration) = literal(expr).unwrap();
    assert_eq!(i, "");
    assert_eq!(duration.to_string(), should);
}

fn compare_exec(expr: &str, should: &str) -> () {
    let duration = exec(expr).unwrap();
    assert_eq!(duration.to_string(), should);
}

#[test]
fn test_literal_parsing() {
    compare_literal("5h50m", "5h50m");
    compare_literal("5h", "5h0m");
    compare_literal("2h40", "2h40m");
    compare_literal("3m", "0h3m");
    compare_literal("3.5h", "3h30m");
}

#[test]
#[should_panic]
fn test_literal_parsing_without_unit() {
    literal("40").unwrap();
}

#[test]
fn test_literal_formatting() {
    compare_literal("5h70m", "6h10m");
}

#[test]
#[should_panic]
fn test_expr_left_over_input() {
    exec("5h70m+").unwrap();
}

#[test]
fn test_nested_expr() {
    compare_exec("5h50m-3h+2m", "2h52m");
}

#[test]
fn test_double_negation() {
    compare_exec("1h--1h-1h", "1h0m");
}

#[test]
fn test_exec() {
    compare_exec("12h34m+56m-10m", "13h20m");
    compare_exec("2h-1.5h", "0h30m");
}

#[test]
fn test_ignore_spaces() {
    compare_exec("12m +    1h   -   1h", "0h12m")
}