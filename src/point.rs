use nom::{
    bytes::complete::tag, character::complete::{char, digit1}, combinator::{map, map_res}, sequence::separated_pair, IResult
};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(input)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let parse_two_numbers = separated_pair(parse_numbers, char(','), parse_numbers);
        map(parse_two_numbers, |(x, y)| Point { x, y })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
        let tests = [
            ("1,2", Point { x: 1, y: 2 }, ""),
            ("1,2asdf", Point { x: 1, y: 2 }, "asdf"),
        ];
        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = Point::parse(input).unwrap();
            assert_eq!(output, expected_output);
            assert_eq!(remaining_input, expected_remaining_input);
        }
    }
}