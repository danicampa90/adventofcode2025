use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, multi::separated_list1,
};

#[derive(Debug)]
pub struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn flatten(&self) -> impl Iterator<Item = i64> {
        self.start..self.end + 1
    }
}

pub fn parse_range(input: &str) -> IResult<&str, Range> {
    let (rest, (start, _, end)) = (digit1, tag("-"), digit1).parse(input)?;
    return Ok((
        rest,
        Range {
            start: start.parse().unwrap(), // assume the number fits in 64 bits
            end: end.parse().unwrap(),
        },
    ));
}

pub fn parse_range_list(input: &str) -> IResult<&str, Vec<Range>> {
    separated_list1(tag(","), parse_range).parse(input)
}
