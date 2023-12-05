use std::{collections::HashMap, num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("Did not find a colon in the input line")]
    ParseErrorNoColon,
    #[error("Cannot parse card id")]
    ParseCardIdErorr,
    #[error("Did not find a pipe in the input line")]
    ParseErrorNoPipe,
    #[error("Could not parse number: `{0}`")]
    ParseNumberError(String),
}

#[derive(Debug, PartialEq)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn new(id: usize, winning_numbers: Vec<usize>, numbers: Vec<usize>) -> Card {
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }

    fn amount_of_correct_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect::<Vec<_>>()
            .len()
    }

    fn points(&self) -> usize {
        let n_correct_numbers = self.amount_of_correct_numbers();
        if n_correct_numbers > 0 {
            2_usize.pow((n_correct_numbers - 1) as u32)
        } else {
            0
        }
    }
}

impl FromStr for Card {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_string, all_numbers) = s.split_once(':').ok_or(AOCError::ParseErrorNoColon)?;

        // chop of "Card "
        let id: usize = card_string[5..]
            .trim()
            .parse()
            .map_err(|_| AOCError::ParseCardIdErorr)?;

        let (winning_numbers, numbers) = all_numbers
            .split_once('|')
            .ok_or(AOCError::ParseErrorNoPipe)?;

        let winning_numbers = winning_numbers
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| {
                num.parse()
                    .map_err(|e: ParseIntError| AOCError::ParseNumberError(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let numbers = numbers
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|num| {
                num.parse()
                    .map_err(|e: ParseIntError| AOCError::ParseNumberError(e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Card::new(id, winning_numbers, numbers))
    }
}

pub fn parse_input(input: &str) -> Vec<Card> {
    input
        .trim()
        .split('\n')
        .map(|line| line.parse::<Card>().unwrap())
        .collect()
}

pub fn process_part1(cards: &[Card]) -> usize {
    cards.iter().map(|c| c.points()).sum()
}

pub fn process_part2(cards: &[Card]) -> usize {
    let mut amounts: HashMap<usize, usize> = HashMap::with_capacity(cards.len());
    for i in 1..=cards.len() {
        amounts.insert(i, 1);
    }
    for card in cards {
        let this_card_amount = *amounts.get(&card.id).unwrap();
        for i in card.id + 1..=card.id + card.amount_of_correct_numbers() {
            *amounts.get_mut(&i).unwrap() += this_card_amount;
        }
    }
    amounts.values().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);

        assert_eq!(
            parsed_input[0],
            Card::new(
                1,
                vec![41, 48, 83, 86, 17],
                vec![83, 86, 6, 31, 17, 9, 48, 53]
            )
        );
    }

    #[test]
    fn test_process_part1() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process_part1(&parsed_input);
        assert_eq!(output, 13)
    }

    #[test]
    fn test_process_part2() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process_part2(&parsed_input);
        assert_eq!(output, 30)
    }
}
