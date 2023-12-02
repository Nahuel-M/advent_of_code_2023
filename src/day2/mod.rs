use crate::utils::nom::number;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::number::complete::u32;
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;
use std::fs::read_to_string;

pub fn day2_1() {
    let sum: u32 = include_str!("../../inputs/day2.txt")
        .lines()
        .map(Game::from_str)
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.number)
        .sum();
    println!("Day 2 - Part 1: {sum}");
}

pub fn day2_2() {
    let sum: u32 = include_str!("../../inputs/day2.txt")
        .lines()
        .map(Game::from_str)
        .map(Game::min_cubes)
        .map(Round::to_power)
        .sum();
    println!("Day 2 - Part 2: {sum}");
}

#[derive(Debug)]
struct Game {
    number: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from_str(input: &str) -> Game {
        Game::parse(input).unwrap().1
    }
    fn parse(input: &str) -> IResult<&str, Game> {
        let (input, number) = delimited(tag("Game "), number, tag(": "))(input)?;
        let (input, rounds) = separated_list1(tag("; "), Round::parse)(input)?;
        Ok((input, Game { number, rounds }))
    }

    fn min_cubes(self) -> Round {
        self.rounds
            .into_iter()
            .reduce(|acc, e| Round {
                red: acc.red.max(e.red),
                green: acc.green.max(e.green),
                blue: acc.blue.max(e.blue),
            })
            .unwrap()
    }
}
#[derive(Debug, Clone, Copy)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn parse(input: &str) -> IResult<&str, Round> {
        let (input, result): (&str, Vec<(u32, &str)>) = separated_list1(
            tag(", "),
            separated_pair(number, space1, alt((tag("red"), tag("green"), tag("blue")))),
        )(input)?;
        let round = Round {
            red: num_for_color("red", &result),
            green: num_for_color("green", &result),
            blue: num_for_color("blue", &result),
        };
        Ok((input, round))
    }

    fn to_power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn num_for_color(color: &str, cubes: &[(u32, &str)]) -> u32 {
    cubes
        .iter()
        .find(|(_, c)| c == &color)
        .map(|(n, _)| *n)
        .unwrap_or(0)
}
