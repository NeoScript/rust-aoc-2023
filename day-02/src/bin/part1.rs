use functions::*;

fn main() {
    let input = include_str!("./input1.txt");
    let result = process(input);
    println!("{}", result);
}

pub mod functions {
    pub fn process(input: &str) -> i32 {
        let lines: Vec<&str> = input.split("\n").collect();
        let games: Vec<Game> = lines.iter().map(|x| parse_game_state(*x)).collect();

        let sum = games
            .iter()
            .filter(|x| check_game_valid(12, 13, 14, *x))
            .map(|x| x.id)
            .sum();

        sum
    }
    #[derive(PartialEq, Eq, Debug)]
    pub struct Round {
        pub red: i32,
        pub green: i32,
        pub blue: i32,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub struct Game {
        pub id: i32,
        pub rounds: Vec<Round>,
    }

    pub fn check_game_valid(red: i32, green: i32, blue: i32, game: &Game) -> bool {
        for round in game.rounds.iter() {
            if round.red > red || round.green > green || round.blue > blue {
                return false;
            }
        }
        true
    }

    pub fn parse_game_state(summary: &str) -> Game {
        // lets determine game number first
        let first_split: Vec<&str> = summary.split(":").collect();

        let game_str = first_split[0];
        let game_id: i32 = game_str
            .trim()
            .split_once(" ")
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();

        let round_info: Vec<&str> = first_split[1].split(";").collect();

        let rounds: Vec<Round> = round_info.iter().map(|x| parse_round_stats(*x)).collect();
        Game {
            id: game_id,
            rounds,
        }
    }

    pub fn parse_round_stats(round_str: &str) -> Round {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        let info: Vec<&str> = round_str.split(",").collect();

        for i in info {
            let (count, color) = match i.trim().split_once(" ") {
                Some((count, color)) => (count.parse::<i32>().unwrap(), color),
                None => panic!("Invalid split for color and count."),
            };

            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("invalid color"),
            }
        }

        return Round { red, green, blue };
    }
}

#[cfg(test)]
mod tests {
    use super::functions::*;

    #[test]
    fn test_parse_round_stats() {
        let round_info = " 3 blue, 2 green, 4 red";
        let result = parse_round_stats(round_info);

        assert_eq!(
            result,
            Round {
                red: 4,
                green: 2,
                blue: 3
            }
        )
    }

    #[test]
    fn test_parse_game_state() {
        let game_info = "Game 2: 2 blue, 1 red; 3 green";
        let result = parse_game_state(game_info);

        assert_eq!(
            result,
            Game {
                id: 2,
                rounds: vec![
                    Round {
                        red: 1,
                        blue: 2,
                        green: 0
                    },
                    Round {
                        green: 3,
                        red: 0,
                        blue: 0
                    }
                ]
            }
        )
    }

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = process(input);
        assert_eq!(result, 8);
    }
}
