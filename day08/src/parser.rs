use nom::{IResult, Parser, bytes::complete::tag, character::complete::digit1, sequence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxCoord {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
impl BoxCoord {
    pub(crate) fn distance_to(&self, box2: &BoxCoord) -> i64 {
        let dx = self.x - box2.x;
        let dy = self.y - box2.y;
        let dz = self.z - box2.z;
        return dx * dx + dy * dy + dz * dz;
    }
}

pub fn parse_num(input: &str) -> IResult<&str, i64> {
    digit1.map(|num: &str| num.parse().unwrap()).parse(input)
}
pub fn parse_connection(input: &str) -> IResult<&str, BoxCoord> {
    sequence::separated_pair(
        parse_num,
        tag(","),
        sequence::separated_pair(parse_num, tag(","), parse_num),
    )
    .map(|(x, (y, z))| BoxCoord { x, y, z })
    .parse(input)
}
