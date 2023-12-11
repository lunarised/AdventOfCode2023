use core::num;

#[derive(Debug)]
struct NumberLocation {
    row: i32,
    col: i32,
    length: i32,
}

pub fn day03a(puzzle_input: &str) -> i32 {
    let mut game_sum = 0;
    // let number_vector = Vec::new();
    let mut symbol_map: Vec<Vec<i32>> = vec![];
    let mut number_map: Vec<NumberLocation> = vec![];
    let game_lines = puzzle_input.lines();
    let mut line_number = 0;
    for line in game_lines {
        let mut row_vector = vec![];
        let line_tokens = line.chars().enumerate();
        let mut number_len = 0 as i32;
        for (ind, character) in line_tokens {
            if character.is_numeric() {
                number_len += 1;
            } else {
                if number_len != 0 {
                    let mut foo = NumberLocation {
                        row: line_number,
                        col: ind as i32 - number_len,
                        length: number_len,
                    };
                    number_len = 0;
                    number_map.push(foo);
                }
                if character != '.' {
                    row_vector.push(ind as i32);
                }
            }
        }
        //for char in line_tokens {}
        symbol_map.push(row_vector);
        line_number += 1;
    }

    println!("{:?}", number_map);
    println!("{:?}", symbol_map);
    return game_sum;
}
