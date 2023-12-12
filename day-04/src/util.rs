use std::{collections::HashSet, num::ParseIntError, str::FromStr};

pub struct GameCard {
    id: i32,
    winning_numbers: Vec<i32>,
    game_numbers: Vec<i32>,
}

pub trait GameResults {
    fn get_points(&self) -> i32;
}

impl GameResults for GameCard {
    fn get_points(&self) -> i32 {
        // create basic score
        // now create hashmap of winning numbers
        // count matches in winnning + game numbers
        // points = 2^(matches-1)

        let winners: HashSet<i32> = HashSet::from_iter(self.winning_numbers.iter().cloned());

        let matches_found: u32 = self
            .game_numbers
            .iter()
            .filter(|&n| winners.contains(n))
            .count()
            .try_into()
            .unwrap();
        
        if matches_found > 0{
            2_i32.pow(matches_found-1)
        }else{
            0
        }

    }
}

fn parse_numbers(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.split_whitespace().map(|x| x.parse::<i32>()).collect()
}

impl FromStr for GameCard {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = input.split([':', '|']).collect();

        let game_id: i32 = splits[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse::<i32>()
            .map_err(|_| "Failed to parse game id".to_string())?;

        let winning_numbers: Vec<i32> = parse_numbers(splits[1]).map_err(|e| e.to_string())?;
        let game_numbers: Vec<i32> = parse_numbers(splits[2]).map_err(|e| e.to_string())?;

        Ok(GameCard {
            id: game_id,
            winning_numbers,
            game_numbers,
        })
    }
}
