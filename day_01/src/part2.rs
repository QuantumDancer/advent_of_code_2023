use std::fs;

use day_01::{parse, process_part2};

fn main() {
    let input = fs::read_to_string("input_part2.txt").expect("Could not read the file");
    let parsed_input = parse(&input);
    let output = process_part2(&parsed_input);
    println!("{output}");
}
