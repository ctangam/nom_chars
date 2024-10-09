use nom::{
    bytes::complete::tag, character::complete::{char, digit1}, combinator::{map, map_res}, sequence::separated_pair, IResult
};
use std::str::FromStr;

use crate::point::Point;

#[derive(Debug, Eq, PartialEq)]
pub struct Line(pub Point, pub Point);

impl Line {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_points = separated_pair(Point::parse, tag(" -> "), Point::parse);
        map(parse_two_points, |(p1, p2)| Line(p1, p2))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            (
                "0,9 -> 5,9",
                Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
                "",
            ),
            (
                "0,9 -> 5,9xyz",
                Line(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
                "xyz",
            ),
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Line::parse(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }
}