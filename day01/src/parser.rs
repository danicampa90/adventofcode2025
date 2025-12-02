use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;

#[derive(Debug)]
pub enum Rotation {
    Left(i32),
    Right(i32),
}

pub fn parse_rotation(input: &str) -> IResult<&str, Rotation> {
    let (rest, direction) = alt((tag("R"), tag("L"))).parse(input)?;
    let (rest, number) = map(digit1, |res: &str| res.parse().unwrap()).parse(rest)?;
    if direction == "R" {
        Ok((rest, Rotation::Right(number)))
    } else {
        Ok((rest, Rotation::Left(number)))
    }
}

pub fn parse_full_list(input: &str) -> IResult<&str, Vec<Rotation>> {
    many0(map((multispace0, parse_rotation), |(_res1, res2)| res2)).parse(input)
}
