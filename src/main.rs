mod day01;
mod utils;
fn main() {
let text_input = utils::read_file("inputs/01a.txt".to_string());
println!("{}", day01::day01a(text_input.as_str()))
}
