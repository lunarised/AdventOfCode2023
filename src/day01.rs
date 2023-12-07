pub fn day01a(puzzle_input: &str) -> i32 {
    let mut calibration_sum = 0;
    let calibration_lines = puzzle_input.lines();
    for calibration_line in calibration_lines {
        let mut sub_string = String::from("");
        for letter in calibration_line.chars() {
            if letter.is_digit(10) {
                sub_string.push(letter);
            }
        }
        let mut out_string = String::from("");
        out_string.push(sub_string.chars().next().unwrap());
        out_string.push(sub_string.chars().last().unwrap());
        calibration_sum += out_string.parse::<i32>().unwrap();
    }
    return calibration_sum;
}

pub fn day01b(puzzle_input: &str) -> i32 {
    let enumerable_line = puzzle_input.lines();
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut result_sum = 0;
    for line in enumerable_line {
        let mut number_vector: Vec<i32> = Vec::new();
        for (ind, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number_vector.push(char.to_digit(10).unwrap() as i32);
            } else {
                for num in nums {
                    if line[ind..].find(num).unwrap_or(100000) == 0 {
                        number_vector.push(nums.iter().position(|&n| n == num).unwrap() as i32 + 1)
                    }
                }
            }
        }
        result_sum += (number_vector.first().unwrap() * 10) + number_vector.last().unwrap()
    }
    return result_sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
    #[test]
    fn part1_test() {
        let day01_test_input = utils::read_file("inputs/01atest.txt".to_string());
        assert_eq!(day01a(day01_test_input.as_str()), 142);
    }
    #[test]
    fn part2_test() {
        let day02_test_input = utils::read_file("inputs/01btest.txt".to_string());
        assert_eq!(day01b(day02_test_input.as_str()), 281);
    }
}
