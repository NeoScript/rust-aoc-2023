use ::std::str::FromStr;
use util::{GameCard, GameResults};

mod util;

fn main() {
    let input = include_str!("./4.txt");
    let part_1_results = part1(input);

    println!("{}", part_1_results);
}

fn part1(input: &str) -> String {
    // First we want to ingest the input
    // Create a vec of games
    // Then go thru each game and determine points
    // then sum up points

    let cards: Vec<&str> = input.split("\n").collect();

    let points: i32 = cards
        .iter()
        .map(|&x| GameCard::from_str(x).unwrap())
        .map(|card| card.get_points())
        .sum();

    format!("{}", points)
}
