use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::{many0, many1, separated_list1},
};

use crate::NumberOrOperator;
pub fn parse_whitespace(input: &str) -> IResult<&str, ()> {
    many1(tag(" ")).map(|_| ()).parse(input)
}
pub fn parse_whitespace_opt(input: &str) -> IResult<&str, ()> {
    many0(tag(" ")).map(|_| ()).parse(input)
}

pub fn parse_symbol(input: &str) -> IResult<&str, NumberOrOperator> {
    alt((tag("+"), tag("*")))
        .map(|ch: &str| NumberOrOperator::Operator(ch.chars().next().unwrap()))
        .parse(input)
}
pub fn parse_number(input: &str) -> IResult<&str, NumberOrOperator> {
    let res = digit1
        .map(|num: &str| NumberOrOperator::Number(num.parse().unwrap()))
        .parse(input);
    res
}

pub fn parse_num_or_op(input: &str) -> IResult<&str, NumberOrOperator> {
    alt((parse_symbol, parse_number)).parse(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<NumberOrOperator>> {
    separated_list1(parse_whitespace, parse_num_or_op).parse(input)
}

pub fn parse_line_part2(input: &str) -> IResult<&str, Vec<NumberOrOperator>> {
    separated_list1(parse_whitespace_opt, parse_num_or_op).parse(input)
}
