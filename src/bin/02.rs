use std::{error::Error, ops::Index};

advent_of_code::solution!(2);

#[derive(PartialEq, Debug)]
struct Game {
    id: u8,
    red_dice: Vec<u8>,
    green_dice: Vec<u8>,
    blue_dice: Vec<u8>,
}


impl Game {
    // Will determine the minumum dice needed and calulate the "power" of the set
    fn min_dice_needed_pow(&self) -> u32 {
        let min_red: u32 = self.red_dice.iter().max().unwrap().to_owned().into();
        let min_green: u32 = self.green_dice.iter().max().unwrap().to_owned().into();
        let min_blue: u32 = self.blue_dice.iter().max().unwrap().to_owned().into();
        let pow= min_red * min_blue * min_green;
        // println!("power: {}", pow);
        pow
    }

    /// Check to see if a game is possible for a given input of dice
    fn is_game_possible(&self, red_dice: u8, green_dice: u8, blue_dice: u8) -> bool {
        if self.red_dice.iter().max().unwrap() > &red_dice {
            return false;
        }

        if self.green_dice.iter().max().unwrap()  > &green_dice {
            return false;
        }

        if self.blue_dice.iter().max().unwrap()  > &blue_dice {
            return false;
        }


        true
    }


    fn parse(input: &str) -> Self {

        let (game_info, round_info) = input.split_once(":").unwrap();
        
        // Parse ID
        let id:u8 = game_info.strip_prefix("Game ").unwrap().parse().unwrap();
        // println!("Game ID: {}", id);

        // Parse dice
        let rounds:Vec<&str> = round_info.split(";").collect();

        let mut red: Vec<u8> = vec![];
        let mut green: Vec<u8> = vec![];
        let mut blue: Vec<u8> = vec![];

        for round in rounds {
            // Split up dice shown
            let dice_shown: Vec<&str> = round.split(",").map(|x| x.strip_prefix(" ").unwrap()).into_iter().collect();
            // println!("dice shown: {:?}", dice_shown);
            // Remove leading white space
            for dice in dice_shown {
                // Example dice format: "3 blue"
                let (num, color) = dice.split_once(" ").unwrap();
                let roll: u8 = num.parse().unwrap();

                
                match color {
                    "red" => {
                        red.push(roll);
                    }, 
                    "green" => {
                        green.push(roll);
                    }, 
                    _ => { // Blue or something else
                        blue.push(roll);
                    }
                }
            }

        }

        Self { id: id, red_dice: red, green_dice: green, blue_dice: blue }
    }
}


pub fn part_one(input: &str,) -> Option<u32> {
    let mut game_id_sum: u32 = 0;
    for game_line in input.split("\n").into_iter() {
        let game = Game::parse(game_line);
        // println!("GAME: {:?}", game);
        if game.is_game_possible(12, 13, 14) {
            game_id_sum += game.id.clone() as u32;
        }
    }

    Some(game_id_sum)
}


pub fn part_two(input: &str) -> Option<u32> {
    let mut game_pow_sum: u32 = 0;
    for game_line in input.split("\n").into_iter() {
        let game = Game::parse(game_line);
        game_pow_sum += game.min_dice_needed_pow();
    }

    Some(game_pow_sum)
}



#[cfg(test)]
mod tests {
const EXAMPLE_INPUT: &str = 
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    use super::*;

    #[test]
    fn test_part_one() {

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(result, Some(1853));
    }
    
    #[test]
    fn test_part_one_example_input() {
        let result = part_one(EXAMPLE_INPUT);
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_game_parse() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let parsed_game = Game::parse(input);

        let expected_game = Game {
            id: 1,
            red_dice: vec![4, 1],
            green_dice: vec![2,2],
            blue_dice: vec![3,6],
        };
        assert_eq!(parsed_game, expected_game);
    }

    
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(72706));
    }

    #[test]
    fn test_part_two_example_input() {
        let result = part_two(EXAMPLE_INPUT);
        assert_eq!(result, Some(2286));
    }
}
