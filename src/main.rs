mod day01;
mod day02;
mod day03;
mod utils;
fn main() {
    let text_input_1 = utils::read_file("inputs/01a.txt".to_string());
    println!("{}", day01::day01a(text_input_1.as_str()));
    println!("{}", day01::day01b(text_input_1.as_str()));
    let text_input_2 = utils::read_file("inputs/02a.txt".to_string());
    println!("{}", day02::day02a(text_input_2.as_str()));
    println!("{}", day02::day02b(text_input_2.as_str()));
    let text_input_3 = utils::read_file("inputs/03atest.txt".to_string());
    println!("{}", day03::day03a(text_input_3.as_str()));
}
