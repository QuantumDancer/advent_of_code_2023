use std::fs;

use day_01::{parse, process_part1};

fn main() {
    let input = fs::read_to_string("input_part1.txt").expect("Could not read the file");
    let parsed_input = parse(&input);
    let output = process_part1(&parsed_input);
    println!("{output}");
}
