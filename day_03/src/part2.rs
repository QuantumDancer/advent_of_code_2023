use std::fs;

use day_03::{parse_input, process, SolutionPart};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read the file");
    let parsed_input = parse_input(&input);
    let output = process(&parsed_input, SolutionPart::Part2);
    println!("{output}");
}
