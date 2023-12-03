use std::collections::HashMap;

use ndarray::Array2;

pub fn setup_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub fn parse_input(input: &str) -> Array2<char> {
    let rows: Vec<&str> = input.trim().split('\n').collect();

    let schematic_data: Vec<char> = rows
        .iter()
        .flat_map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let n_rows = rows.len();
    let n_cols = rows
        .first()
        .expect("There should be at least one row")
        .len();

    Array2::from_shape_vec((n_rows, n_cols), schematic_data)
        .expect("Should be able to construct 2D array from schematic")
}

pub enum SolutionPart {
    Part1,
    Part2,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

type Gears = HashMap<Point, Vec<u32>>;

pub fn process(schematic: &Array2<char>, part: SolutionPart) -> u32 {
    let mut valid_numbers: Vec<u32> = Vec::new();
    let mut gears: Gears = Gears::new();
    let (n_rows, n_cols) = schematic.dim();
    for y in 0..n_rows {
        let mut current_digits: Vec<char> = Vec::new();
        // let mut parsing_state = ParsingState::default();
        for x in 0..n_cols {
            let elem = schematic[(y, x)];
            let is_digit = if elem.is_ascii_digit() {
                current_digits.push(elem);
                true
            } else {
                false
            };

            // number in the middle of the schematic
            if !is_digit && !current_digits.is_empty() {
                if let Some(possible_number) = construct_new_number(
                    &current_digits,
                    x - current_digits.len(),
                    y,
                    schematic,
                    &mut gears,
                ) {
                    valid_numbers.push(possible_number);
                }
                current_digits.clear();
            }
        }

        // number at right border of schematic
        if !current_digits.is_empty() {
            if let Some(possible_number) = construct_new_number(
                &current_digits,
                n_cols - current_digits.len(),
                y,
                schematic,
                &mut gears,
            ) {
                valid_numbers.push(possible_number);
            }
            current_digits.clear();
        }
    }
    match part {
        SolutionPart::Part1 => valid_numbers.iter().sum(),
        SolutionPart::Part2 => gears
            .iter()
            .filter_map(|(_, numbers)| {
                if numbers.len() == 2 {
                    Some(numbers[0] * numbers[1])
                } else {
                    None
                }
            })
            .sum(),
    }
}

fn construct_new_number(
    current_digits: &[char],
    x_start: usize,
    y: usize,
    schematic: &Array2<char>,
    gears: &mut Gears,
) -> Option<u32> {
    tracing::info!("construct_new_number({current_digits:?}, {x_start}, {y})");
    let possible_number = current_digits
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    if is_valid_number(
        x_start as i32,
        (x_start + current_digits.len() - 1) as i32,
        y as i32,
        schematic,
        possible_number,
        gears,
    ) {
        Some(possible_number)
    } else {
        None
    }
}

fn is_symbol(x: i32, y: i32, schematic: &Array2<char>) -> Option<&char> {
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(char) = schematic.get((y as usize, x as usize)) {
        if !char.is_ascii_digit() && char != &'.' {
            tracing::debug!("Is valid number because of '{char}' at ({x}, {y})");
            Some(char)
        } else {
            None
        }
    } else {
        None
    }
}

fn add_gear_ratio(x: i32, y: i32, possible_number: u32, gears: &mut Gears) {
    let point = Point {
        x: x as usize,
        y: y as usize,
    };
    if let Some(gear) = gears.get_mut(&point) {
        (*gear).push(possible_number);
    } else {
        gears.insert(point, vec![possible_number]);
    }
}

fn is_valid_number(
    x_start: i32,
    x_end: i32,
    y: i32,
    schematic: &Array2<char>,
    possible_number: u32,
    gears: &mut Gears,
) -> bool {
    tracing::debug!("is_valid_number(x_start={x_start}, x_end={x_end}, y={y})");
    let mut result = false;
    if let Some(char) = is_symbol(x_start - 1, y, schematic) {
        result = true;
        if char == &'*' {
            add_gear_ratio(x_start - 1, y, possible_number, gears);
        }
    }
    if let Some(char) = is_symbol(x_end + 1, y, schematic) {
        result = true;
        if char == &'*' {
            add_gear_ratio(x_end + 1, y, possible_number, gears);
        }
    }
    for x in x_start - 1..=x_end + 1 {
        if let Some(char) = is_symbol(x, y + 1, schematic) {
            result = true;
            if char == &'*' {
                add_gear_ratio(x, y + 1, possible_number, gears);
            }
        }
        if let Some(char) = is_symbol(x, y - 1, schematic) {
            result = true;
            if char == &'*' {
                add_gear_ratio(x, y - 1, possible_number, gears);
            }
        }
    }
    tracing::debug!("Is valid number: {result}");
    result
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);

        assert_eq!(parsed_input[(0, 0)], '4');
        assert_eq!(parsed_input[(0, 9)], '.');
        assert_eq!(parsed_input[(5, 5)], '+');
        assert_eq!(parsed_input[(9, 9)], '.');
    }

    #[test]
    fn test_process_part1() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process(&parsed_input, SolutionPart::Part1);
        assert_eq!(output, 4361)
    }

    #[test]
    fn test_process_part2() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process(&parsed_input, SolutionPart::Part2);
        assert_eq!(output, 467835)
    }
}
