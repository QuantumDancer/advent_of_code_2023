use std::fs;

use day_05::{parse_input, process_part1};

fn main() {
    // setup_tracing();
    let input = fs::read_to_string("input.txt").expect("Could not read the file");
    let parsed_input = parse_input(&input);
    let output = process_part1(&parsed_input);
    println!("{output}");
}
