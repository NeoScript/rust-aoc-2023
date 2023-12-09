mod part1;

fn main() {
    let input = include_str!("./input1.txt");
    let result = part2::process(input);
    println!("{}", result);
}

pub mod part2 {
    use crate::part1::functions::{parse_game_state, Game};

    pub fn process(input: &str) -> i32 {
        let lines: Vec<&str> = input.split("\n").collect();
        let games: Vec<Game> = lines.iter().map(|x| parse_game_state(*x)).collect();

        let min_counts: Vec<MinCounts> = games.iter().map(|x| determine_min_counts(x)).collect();
        let powers:Vec<i32> = min_counts.iter().map(|m| m.red * m.green * m.blue).collect();
        
        let sum = powers.iter().sum();

        sum
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct MinCounts {
        pub red: i32,
        pub green: i32,
        pub 
        blue: i32,
    }

    pub fn determine_min_counts(game: &Game) ->MinCounts {
        let (mut red, mut green, mut blue) = (0,0,0);

        for round in game.rounds.iter(){
            red = red.max(round.red);
            green = green.max(round.green);
            blue = blue.max(round.blue);
        }

        MinCounts { red, green, blue}
    }
}

#[cfg(test)]
mod tests {
    use crate::part1::functions::{Game, Round};

    use super::part2::*;

    #[test]
    fn test_determine_min_counts() {
        let game = Game {
            id: 1,
            rounds: vec![
                Round {
                    red: 1,
                    green: 2,
                    blue: 3,
                },
                Round {
                    red: 3,
                    green: 2,
                    blue: 3,
                },
            ],
        };

        let result = determine_min_counts(&game);
        assert_eq!(result, MinCounts{red:3, green:2, blue: 3})
    }
}
