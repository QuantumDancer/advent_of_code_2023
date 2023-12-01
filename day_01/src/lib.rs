use std::fmt::Display;

pub fn parse(input: &str) -> Vec<&str> {
    input.split('\n').filter(|l| !l.is_empty()).collect()
}

pub fn process_part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| line.chars().filter(|c| c.is_numeric()).collect::<String>())
        .map(|numbers| {
            let first = numbers.chars().next().expect("String is not empty");
            let last = numbers.chars().next_back().expect("String is not empty");
            first.to_digit(10).expect("This should be a number") * 10
                + last.to_digit(10).expect("This should be a number")
        })
        .sum()
}

fn convert_numbers<T: AsRef<str> + Display>(input: T) -> String {
    let number_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut line = input.to_string();
    let mut marker = 0;

    while marker < line.len() {
        for (idx, number_word) in number_words.iter().enumerate() {
            let range = marker..marker + number_word.len();
            if let Some(number) = line.get(range.clone()) {
                if &number == number_word {
                    line.replace_range(marker..marker + 1, &(idx + 1).to_string());
                    break;
                }
            }
        }
        marker += 1
    }
    line.chars().filter(|c| c.is_numeric()).collect()
}

pub fn process_part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(convert_numbers)
        .map(|numbers| {
            let first = numbers.chars().next().expect("String is not empty");
            let last = numbers.chars().next_back().expect("String is not empty");
            first.to_digit(10).expect("This should be a number") * 10
                + last.to_digit(10).expect("This should be a number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_process_part1() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let output = process_part1(&input);
        assert_eq!(output, 142)
    }

    #[test]
    fn test_convert_numbers() {
        let tests = [
            ("two1nine", "219"),
            ("eightwothree", "823"),
            ("abcone2threexyz", "123"),
            ("zoneight234", "18234"),
            ("eightoneight", "818"),
            ("3three7three118", "3373118"),
        ];
        for (input, expected) in tests {
            assert_eq!(convert_numbers(input), expected);
        }
    }

    #[test]
    fn test_process_part2() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let output = process_part2(&input);
        assert_eq!(output, 281)
    }
}
