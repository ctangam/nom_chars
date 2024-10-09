use line::Line;
use nom::{
    bytes::complete::tag, character::complete::{char, digit1}, combinator::{map, map_res}, multi::separated_list1, sequence::separated_pair, IResult
};
use std::str::FromStr;

mod point;
mod line;

pub fn parse_input(s: &str) -> Vec<Line> {
    let (remaining_input, lines) = separated_list1(char('\n'), Line::parse)(s).unwrap();
    assert!(remaining_input.is_empty());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        // Assuming you're logged in, you can download your Day 5 puzzle input from
        // https://adventofcode.com/2021/day/5/input
        // Then save it under src/data/input.txt
        // The `include_str!` macro reads a file into a String at compile-time.
        // I use it for all my AoC problems!
        let input = include_str!("data\\input.txt");
        let lines = parse_input(input);
        assert_eq!(lines.len(), 500);
    }
}

fn main() {
    println!("Hello, world!");
}
