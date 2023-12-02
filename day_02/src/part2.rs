use std::fs;

use day_02::{parse_input, process_part2};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read the file");
    let parsed_input = parse_input(&input);
    let output = process_part2(&parsed_input);
    println!("{output}");
}
