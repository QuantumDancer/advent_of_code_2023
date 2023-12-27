use std::cmp::Ordering;
use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("Could not parse number: `{0}`")]
    ParseNumberError(String),
    #[error("Could not find the seed block")]
    SeedBlockMissingError,
    #[error("Map header is missing")]
    MapHeaderMissingError,
    #[error("Error while parsing map header")]
    MapHeaderParseError,
    #[error("Could not parse range into three parts")]
    RangeParseError,
}

#[derive(Debug, PartialEq)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<CategoryMap>,
}

#[derive(Debug, PartialEq)]
struct CategoryMap {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

#[derive(Debug, PartialEq)]
pub struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SourceIdRange {
    start: usize,
    length: usize,
}

impl SourceIdRange {
    fn new(start: usize, length: usize) -> SourceIdRange {
        SourceIdRange { start, length }
    }
}

impl From<usize> for SourceIdRange {
    fn from(value: usize) -> Self {
        SourceIdRange {
            start: value,
            length: 1,
        }
    }
}

impl Ord for SourceIdRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SourceIdRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Almanac {
    fn convert(
        &self,
        source_id: &[SourceIdRange],
        source: &str,
        destionation: &str,
    ) -> Option<Vec<SourceIdRange>> {
        // check if we have the current source-destination combination in the map collection
        for map in &self.maps {
            if map.source == source && map.destination == destionation {
                return Some(map.calculate(source_id));
            }
        }

        // if we don't have the current combination, find a map that has the current source as source
        for map in &self.maps {
            if map.source == source {
                // convert the value and continue with the new map
                let new_source_id = map.calculate(source_id);
                return self.convert(&new_source_id, &map.destination, destionation);
            }
        }

        // We should never end up here
        None
    }
}

impl FromStr for Almanac {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = s.split("\n\n");

        let seeds = if let Some(seed_string) = blocks.next() {
            seed_string
                .trim()
                .split(' ')
                .skip(1) // skip "seeds:"
                .map(|s| {
                    s.parse()
                        .map_err(|e: ParseIntError| AOCError::ParseNumberError(e.to_string()))
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            return Err(AOCError::SeedBlockMissingError);
        };

        let maps = blocks
            .map(|block| block.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Almanac { seeds, maps })
    }
}

impl CategoryMap {
    fn calculate(&self, source_ids: &[SourceIdRange]) -> Vec<SourceIdRange> {
        source_ids
            .iter()
            .flat_map(|source_id| self.calculate_single(source_id))
            .collect()
    }

    fn calculate_single(&self, source_id: &SourceIdRange) -> Vec<SourceIdRange> {
        // calculate the overlap betwen the source_id and each range
        let mut remaining: Vec<SourceIdRange> = vec![*source_id];
        let mut overlaps = Vec::new();
        for range in self.ranges.iter() {
            let mut remaining_new = Vec::new();
            for sid in remaining.iter() {
                let mut overlap = range.overlap(sid);
                if let Some(matching) = overlap.matching {
                    overlaps.push((matching, range));
                }
                remaining_new.append(&mut overlap.remaining)
            }
            remaining = remaining_new;
        }

        let mut result = Vec::with_capacity(overlaps.len() + remaining.len());
        // calculate destination id for ranges where we have overlap
        for (source_id, range) in overlaps.iter() {
            let destination_id = range.translate(source_id);
            result.push(destination_id);
        }
        // ranges that are not matched keep the same source ids
        result.append(&mut remaining);

        // filter out ranges that have 0 length
        result
            .into_iter()
            .filter(|source_id| source_id.length > 0)
            .collect()
    }
}

impl FromStr for CategoryMap {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let header = lines.next().ok_or(AOCError::MapHeaderMissingError)?;
        let (source, destination) = if let Some((mapping_info, _)) = header.split_once(' ') {
            let mapping_info_parts: Vec<_> = mapping_info.splitn(3, '-').collect();
            (
                mapping_info_parts
                    .first()
                    .ok_or(AOCError::MapHeaderParseError)?
                    .to_string(),
                mapping_info_parts
                    .last()
                    .ok_or(AOCError::MapHeaderParseError)?
                    .to_string(),
            )
        } else {
            return Err(AOCError::MapHeaderParseError);
        };
        let ranges = lines.map(|l| l.parse()).collect::<Result<Vec<_>, _>>()?;
        Ok(CategoryMap {
            source,
            destination,
            ranges,
        })
    }
}

#[derive(Debug, PartialEq)]
struct RangeOverlap {
    matching: Option<SourceIdRange>,
    remaining: Vec<SourceIdRange>,
}

impl Range {
    fn new(destination_start: usize, source_start: usize, length: usize) -> Range {
        Range {
            destination_start,
            source_start,
            length,
        }
    }

    fn overlap(&self, source_id: &SourceIdRange) -> RangeOverlap {
        let range_start = self.source_start;
        let range_end = self.source_start + self.length;
        let range_length = self.length;
        let source_id_start = source_id.start;
        let source_id_end = source_id.start + source_id.length;

        if range_end < source_id_start {
            RangeOverlap {
                matching: None,
                remaining: vec![*source_id],
            }
        } else if range_start < source_id_start
            && range_end >= source_id_start
            && range_end <= source_id_end
        {
            let matching = Some(SourceIdRange::new(
                source_id_start,
                range_end - source_id_start,
            ));
            let remaining = SourceIdRange::new(range_end, source_id_end - range_end);
            RangeOverlap {
                matching,
                remaining: vec![remaining],
            }
        } else if range_start < source_id_start && range_end > source_id_end {
            RangeOverlap {
                matching: Some(*source_id),
                remaining: vec![],
            }
        } else if range_start >= source_id_start && range_end <= source_id_end {
            let matching = Some(SourceIdRange::new(range_start, range_length));
            let mut remaining = Vec::new();
            if range_start > source_id_start {
                remaining.push(SourceIdRange::new(
                    source_id_start,
                    range_start - source_id_start,
                ));
            }
            if range_end < source_id_end {
                remaining.push(SourceIdRange::new(range_end, source_id_end - range_end));
            }
            RangeOverlap {
                matching,
                remaining,
            }
        } else if range_start <= source_id_end && range_end > source_id_end {
            let matching = Some(SourceIdRange::new(range_start, source_id_end - range_start));
            let remaining = SourceIdRange::new(source_id_start, range_start - source_id_start);
            RangeOverlap {
                matching,
                remaining: vec![remaining],
            }
        } else {
            // range_start > source_id_end
            RangeOverlap {
                matching: None,
                remaining: vec![*source_id],
            }
        }
    }

    fn translate(&self, source_id: &SourceIdRange) -> SourceIdRange {
        let start = source_id.start - self.source_start + self.destination_start;
        SourceIdRange::new(start, source_id.length)
    }
}

impl FromStr for Range {
    type Err = AOCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().splitn(3, ' ').collect();

        fn extract_part(parts: &[&str], id: usize) -> Result<usize, AOCError> {
            parts
                .get(id)
                .ok_or(AOCError::RangeParseError)?
                .parse()
                .map_err(|e: ParseIntError| AOCError::ParseNumberError(e.to_string()))
        }

        let destination_start = extract_part(&parts, 0)?;
        let source_start = extract_part(&parts, 1)?;
        let length = extract_part(&parts, 2)?;
        Ok(Range::new(destination_start, source_start, length))
    }
}

pub fn parse_input(input: &str) -> Almanac {
    input.trim().parse().expect("Could not parse input file")
}

pub fn process_part1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .clone()
        .into_iter()
        .flat_map(|seed| {
            almanac
                .convert(&[seed.into()], "seed", "location")
                .expect("Could not convert from seed to location")
        })
        .min()
        .expect("Could not find minimum")
        .start
}

pub fn process_part2(almanac: &Almanac) -> usize {
    let seed_ranges: Vec<SourceIdRange> = almanac
        .seeds
        .chunks(2)
        .map(|chunk| match chunk {
            &[start, length] => SourceIdRange { start, length },
            _ => panic!("Unexpected chunk size"),
        })
        .collect();

    seed_ranges
        .iter()
        .flat_map(|seed_range| {
            almanac
                .convert(&[*seed_range], "seed", "location")
                .expect("Could not convert from seed to location")
        })
        .min()
        .expect("Could not find minimum")
        .start
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let almanac = parse_input(&input);

        assert_eq!(almanac.seeds, vec![79, 14, 55, 13]);

        let first_map = almanac.maps.first().unwrap();
        assert_eq!(first_map.source, "seed");
        assert_eq!(first_map.destination, "soil");
        assert_eq!(
            first_map.ranges,
            vec![Range::new(50, 98, 2), Range::new(52, 50, 48)]
        );
    }

    #[test]
    fn test_category_map_calculate() {
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let almanac = parse_input(&input);
        let first_map = almanac.maps.first().unwrap();
        assert_eq!(first_map.calculate(&[98.into()]), vec![50.into()]);
        assert_eq!(first_map.calculate(&[99.into()]), vec![51.into()]);
        assert_eq!(first_map.calculate(&[100.into()]), vec![100.into()]);
        assert_eq!(first_map.calculate(&[79.into()]), vec![81.into()]);
        assert_eq!(first_map.calculate(&[14.into()]), vec![14.into()]);
        assert_eq!(first_map.calculate(&[55.into()]), vec![57.into()]);
        assert_eq!(first_map.calculate(&[13.into()]), vec![13.into()]);
    }

    #[test]
    fn test_almanac_convert() {
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let almanac = parse_input(&input);

        assert_eq!(
            almanac.convert(&[79.into()], "seed", "location"),
            Some(vec![82.into()])
        );
        assert_eq!(
            almanac.convert(&[14.into()], "seed", "location"),
            Some(vec![43.into()])
        );
        assert_eq!(
            almanac.convert(&[55.into()], "seed", "location"),
            Some(vec![86.into()])
        );
        assert_eq!(
            almanac.convert(&[13.into()], "seed", "location"),
            Some(vec![35.into()])
        );
    }

    #[test]
    fn test_range_overlap() {
        let source_id = SourceIdRange::new(10, 10); // 10 - 19
                                                    // overlap checks
        let range_1 = Range::new(0, 4, 3); // 4 - 6
        let range_2 = Range::new(0, 8, 4); // 8 - 11
        let range_3 = Range::new(0, 14, 3); // 14 -  16
        let range_4 = Range::new(0, 18, 3); // 18 -  20
        let range_5 = Range::new(0, 25, 5); // 25 -  29
        let range_10 = Range::new(0, 5, 20);
        // edge cases
        let range_6 = Range::new(0, 8, 3); // 8 - 10
        let range_7 = Range::new(0, 10, 2); // 10 - 11
        let range_8 = Range::new(0, 18, 2); // 18 - 19
        let range_9 = Range::new(0, 19, 2); // 19 - 20

        assert_eq!(
            range_1.overlap(&source_id),
            RangeOverlap {
                matching: None,
                remaining: vec![source_id]
            }
        );
        assert_eq!(
            range_2.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(10, 2)), // 10 - 11
                remaining: vec![SourceIdRange::new(12, 8)]
            }
        );
        assert_eq!(
            range_3.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(14, 3)),
                remaining: vec![SourceIdRange::new(10, 4), SourceIdRange::new(17, 3)]
            }
        );
        assert_eq!(
            range_4.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(18, 2)),
                remaining: vec![SourceIdRange::new(10, 8)]
            }
        );
        assert_eq!(
            range_5.overlap(&source_id),
            RangeOverlap {
                matching: None,
                remaining: vec![source_id]
            }
        );
        assert_eq!(
            range_6.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(10, 1)),
                remaining: vec![SourceIdRange::new(11, 9)]
            }
        );
        assert_eq!(
            range_7.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(10, 2)),
                remaining: vec![SourceIdRange::new(12, 8)]
            }
        );
        assert_eq!(
            range_8.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(18, 2)),
                remaining: vec![SourceIdRange::new(10, 8)]
            }
        );
        assert_eq!(
            range_9.overlap(&source_id),
            RangeOverlap {
                matching: Some(SourceIdRange::new(19, 1)),
                remaining: vec![SourceIdRange::new(10, 9)]
            }
        );
        assert_eq!(
            range_10.overlap(&source_id),
            RangeOverlap {
                matching: Some(source_id),
                remaining: vec![]
            }
        );
    }

    #[test]
    fn test_process_part1() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process_part1(&parsed_input);
        assert_eq!(output, 35)
    }

    #[test]
    fn test_process_part2() {
        // setup_tracing();
        let input = fs::read_to_string("input_test.txt").expect("Could not read the file");
        let parsed_input = parse_input(&input);
        let output = process_part2(&parsed_input);
        assert_eq!(output, 46)
    }
}
