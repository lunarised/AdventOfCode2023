use std::cmp;

struct LegalDice {
    blue_dice: i32,
    red_dice: i32,
    green_dice: i32,
}

pub fn day02a(puzzle_input: &str) -> i32 {
    let mut game_sum = 0;
    let dice_count = LegalDice {
        blue_dice: 14,
        green_dice: 13,
        red_dice: 12,
    };
    let game_lines = puzzle_input.lines();
    for game in game_lines {
        let mut is_game_legal = true;
        let game_id = game.split(':').collect::<Vec<&str>>()[0];
        let game_state = game.split(':').collect::<Vec<&str>>()[1];
        let game_pulls = game_state.split(";").collect::<Vec<&str>>();
        for pull in game_pulls {
            let dice_pull = pull.split(",").collect::<Vec<&str>>();
            for dice in dice_pull {
                let dice_data = dice.trim().split(" ").collect::<Vec<&str>>();
                let legal_throw: bool = match dice_data[1] {
                    "red" => dice_data[0].parse::<i32>().unwrap() <= dice_count.red_dice,
                    "blue" => dice_data[0].parse::<i32>().unwrap() <= dice_count.blue_dice,
                    "green" => dice_data[0].parse::<i32>().unwrap() <= dice_count.green_dice,
                    _ => false,
                };
                if !legal_throw {
                    is_game_legal = false
                }
            }
        }
        if is_game_legal {
            game_sum += game_id.trim().split(" ").collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
        }
    }
    return game_sum;
}

pub fn day02b(puzzle_input: &str) -> i32 {
    let mut game_sum = 0;
    let game_lines = puzzle_input.lines();
    for game in game_lines {
        let mut min_dice_req = LegalDice {
            blue_dice: 0,
            red_dice: 0,
            green_dice: 0,
        };
        let game_state = game.split(':').collect::<Vec<&str>>()[1];
        let game_pulls = game_state.split(";").collect::<Vec<&str>>();
        for pull in game_pulls {
            let dice_pull = pull.split(",").collect::<Vec<&str>>();
            for dice in dice_pull {
                let dice_data = dice.trim().split(" ").collect::<Vec<&str>>();
                match dice_data[1] {
                    "red" => {
                        min_dice_req.red_dice =
                            cmp::max(dice_data[0].parse::<i32>().unwrap(), min_dice_req.red_dice)
                    }
                    "blue" => {
                        min_dice_req.blue_dice =
                            cmp::max(dice_data[0].parse::<i32>().unwrap(), min_dice_req.blue_dice)
                    }
                    "green" => {
                        min_dice_req.green_dice = cmp::max(
                            dice_data[0].parse::<i32>().unwrap(),
                            min_dice_req.green_dice,
                        )
                    }

                    _ => {}
                };
            }
        }
        game_sum += min_dice_req.red_dice * min_dice_req.green_dice * min_dice_req.blue_dice
    }
    return game_sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn part1_test() {
        let day02_test_input = utils::read_file("inputs/02atest.txt".to_string());
        assert_eq!(day02a(day02_test_input.as_str()), 8);
    }

    #[test]
    fn part2_test() {
        let day02_test_input = utils::read_file("inputs/02atest.txt".to_string());
        assert_eq!(day02b(day02_test_input.as_str()), 2286);
    }
}
