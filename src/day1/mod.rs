use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_till, take_until, take_while};
use nom::character::complete::{alphanumeric0, anychar, one_of};
use nom::combinator::{fail, not, value};
use nom::multi::{many0, many1};
use nom::sequence::preceded;
use nom::{FindSubstring, IResult};
use std::collections::HashSet;

use crate::utils::vec2::Vec2;

pub fn day1_1() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last = line.chars().rfind(|c| c.is_ascii_digit()).unwrap();
        total += format!("{first}{last}").parse::<u32>().unwrap();
    }
    println!("Day 1 - Part 1: {}", total);
}

pub fn day1_2() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    for line in input.lines() {
        let found = numbers
            .into_iter()
            .enumerate()
            .flat_map(|(number_index, number_name)| {
                line.match_indices(number_name)
                    .map(move |(i, _)| (number_index % 9 + 1, i))
            });

        let min = found.clone().min_by_key(|o| o.1).unwrap().0;
        let max = found.max_by_key(|o| o.1).unwrap().0;
        total += min * 10 + max;
    }

    println!("Day 1 - Part 2: {}", total);
}
