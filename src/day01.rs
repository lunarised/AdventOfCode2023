use std::ops::Index;

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
        let mut outString = String::from("");
        // outString.push(sub_string.chars().next().unwrap());
        // outString.push(sub_string.chars().last().unwrap());
        // calibration_sum += outString.parse::<i32>().unwrap();
    }
    tokenise_number_words(puzzle_input);
    return calibration_sum;
}

pub fn day01b(puzzle_input: &str) -> i32 {
    return 32;
}

fn tokenise_number_words(unprocessed_line: &str) -> &str {
    let mut enumerableLine = unprocessed_line.lines();

    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut resultSum = 0;
    for line in enumerableLine {
        let mut ind = 0;
        let mut number_vector: Vec<i32> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                number_vector.push(char.to_digit(10).unwrap() as i32);
            } else {
                for num in nums {
                    if line.find(num).unwrap_or(100000) == ind {
                        number_vector.push(nums.iter().position(|&n| n == num).unwrap() as i32)
                    }
                }
            }
            ind += 1;
        }
        println!("{:?}", number_vector);
        resultSum += ((number_vector.first().unwrap() * 10) + number_vector.last().unwrap())
    }
    println!("{}", resultSum);
    return "wow";
}
