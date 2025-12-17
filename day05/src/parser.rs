use nom::{
    IResult, Parser, bytes::complete::tag, character::complete::digit1, multi::separated_list1,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}
impl Range {
    pub fn contains(&self, number: i64) -> bool {
        return self.start <= number && number <= self.end;
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
