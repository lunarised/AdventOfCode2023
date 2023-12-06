
pub fn day01a(puzzle_input: String) -> i32 {
    let mut calibration_sum = 0;
    let calibration_lines = puzzle_input.lines();
    for calibration_line in calibration_lines{
        let mut sub_string = String::from("");
        for letter in calibration_line.chars() {
            if letter.is_digit(10){
             sub_string.push(letter);   
            }
        }
        let mut outString = String::from("");
        outString.push(sub_string.chars().next().unwrap());
        outString.push(sub_string.chars().last().unwrap());
        calibration_sum += outString.parse::<i32>().unwrap();
    }
    return calibration_sum
}