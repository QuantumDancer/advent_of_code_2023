use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Game {
    id: u32,
    infos: Vec<GameInfo>,
}

impl Game {
    fn new(id: u32, infos: Vec<GameInfo>) -> Game {
        Game { id, infos }
    }

    fn is_possible(&self, max_values: &GameInfo) -> bool {
        for info in &self.infos {
            if info.r > max_values.r || info.g > max_values.g || info.b > max_values.b {
                return false;
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let r = self.infos.iter().map(|i| i.r).max().unwrap();
        let g = self.infos.iter().map(|i| i.g).max().unwrap();
        let b = self.infos.iter().map(|i| i.b).max().unwrap();
        r * g * b
    }
}

#[derive(Debug, PartialEq)]
pub struct GameInfo {
    r: u32,
    g: u32,
    b: u32,
}

impl GameInfo {
    pub fn new(r: u32, g: u32, b: u32) -> GameInfo {
        GameInfo { r, g, b }
    }
}

#[derive(Debug)]
pub struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(GameParseError);
        }

        let game_string = parts.first().unwrap();
        let info_string = parts.get(1).unwrap();

        let game_id = game_string
            .split(' ')
            .collect::<Vec<&str>>()
            .get(1)
            .expect("There should be a game ID")
            .parse()
            .unwrap();

        let infos = info_string
            .trim()
            .split(';')
            .map(|info_part| {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;

                for color_info in info_part.trim().split(',') {
                    if let Some((amount, color_name)) = color_info.trim().split_once(' ') {
                        match color_name {
                            "red" => r = amount.parse().unwrap(),
                            "green" => g = amount.parse().unwrap(),
                            "blue" => b = amount.parse().unwrap(),
                            _ => {
                                eprintln!("Got unexpected color name {color_name}")
                            }
                        }
                    }
                }

                GameInfo::new(r, g, b)
            })
            .collect();

        Ok(Game::new(game_id, infos))
    }
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .trim()
        .split('\n')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

pub fn process_part1(input: &[Game], max_values: &GameInfo) -> u32 {
    input
        .iter()
        .filter_map(|game| {
            if game.is_possible(max_values) {
                return Some(game.id);
            }
            None
        })
        .sum()
}

pub fn process_part2(input: &[Game]) -> u32 {
    input.iter().map(|game| game.power()).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let output = Game::new(
            1,
            vec![
                GameInfo::new(4, 0, 3),
                GameInfo::new(1, 2, 6),
                GameInfo::new(0, 2, 0),
            ],
        );
        assert_eq!(input.parse::<Game>().unwrap(), output)
    }

    #[test]
    fn test_process_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let parsed_input = parse_input(&input);
        let output = process_part1(&parsed_input, &GameInfo::new(12, 13, 14));
        assert_eq!(output, 8)
    }

    #[test]
    fn test_get_power() {
        let tests = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];

        for (input, expected) in tests {
            let game: Game = input.parse().unwrap();
            assert_eq!(game.power(), expected);
        }
    }
}
