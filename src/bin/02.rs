use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{preceded, terminated},
    IResult,
};

advent_of_code::solution!(2);

#[derive(Debug, Default, PartialEq)]
struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Cubes {
    fn max(&self, other: &Cubes) -> Cubes {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    number: u32,
    grabs: Vec<Cubes>,
}

impl Game {
    fn max(&self) -> Cubes {
        self.grabs
            .iter()
            .fold(Cubes::default(), |acc, cubes| acc.max(&cubes))
    }
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    let mut ans = Cubes::default();
    let (input, _) = nom::multi::separated_list1(tag(", "), |i| {
        let (i, count) = terminated(complete::u32, tag(" "))(i)?;
        let (i, _) = alt((
            map(tag("red"), |_| ans.red += count),
            map(tag("blue"), |_| ans.blue += count),
            map(tag("green"), |_| ans.green += count),
        ))(i)?;
        Ok((i, ()))
    })(input)?;
    Ok((input, ans))
}

fn parse_line(input: &str) -> IResult<&str, Game> {
    let (input, number) = preceded(tag("Game "), complete::u32)(input)?;
    let (input, grabs) = preceded(tag(": "), separated_list1(tag("; "), parse_cubes))(input)?;
    Ok((input, Game { number, grabs }))
}

fn parse_games(input: &str) -> Vec<Game> {
    all_consuming(many1(terminated(parse_line, newline)))(input)
        .unwrap()
        .1
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_games(input)
            .into_iter()
            .filter(|game| {
                let cubes = game.max();
                cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14
            })
            .map(|game| game.number)
            .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_games(input)
            .into_iter()
            .map(|game| game.max().power())
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
