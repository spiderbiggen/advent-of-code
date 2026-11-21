use std::collections::HashMap;
use std::ops::Range;

const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Category {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Category {
    fn from_str(value: &str) -> Self {
        match value {
            "seed-to-soil" => Category::SeedToSoil,
            "soil-to-fertilizer" => Category::SoilToFertilizer,
            "fertilizer-to-water" => Category::FertilizerToWater,
            "water-to-light" => Category::WaterToLight,
            "light-to-temperature" => Category::LightToTemperature,
            "temperature-to-humidity" => Category::TemperatureToHumidity,
            "humidity-to-location" => Category::HumidityToLocation,
            _ => panic!("invalid category: {value}"),
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Category::SeedToSoil => Some(Self::SoilToFertilizer),
            Category::SoilToFertilizer => Some(Self::FertilizerToWater),
            Category::FertilizerToWater => Some(Self::WaterToLight),
            Category::WaterToLight => Some(Self::LightToTemperature),
            Category::LightToTemperature => Some(Self::TemperatureToHumidity),
            Category::TemperatureToHumidity => Some(Self::HumidityToLocation),
            Category::HumidityToLocation => None,
        }
    }
}

#[derive(Debug)]
struct CategoryRanges(Vec<CategoryRange>);

impl CategoryRanges {
    fn new(mut input: Vec<CategoryRange>) -> Self {
        input.sort_by(|a, b| a.source.cmp(&b.source).then(a.length.cmp(&b.length)));
        CategoryRanges(input)
    }

    fn resolve_number(&self, input: u64) -> u64 {
        match self.0.binary_search_by(|r| r.source.cmp(&input)) {
            Ok(index) => {
                let r = &self.0[index];
                r.target + input - r.source
            }
            Err(0) => input,
            Err(max_index) => {
                let range = &self.0[max_index - 1];
                if input < range.source + range.length {
                    range.target + input - range.source
                } else {
                    input
                }
            }
        }
    }

    fn resolve_numbers(&self, input: Range<u64>) -> Vec<Range<u64>> {
        let mapping_iter = self.0.iter().filter(|&r| input.start < r.source + r.length);

        let mut range = input.clone();
        let mut output = vec![];
        for mapping in mapping_iter {
            // no matching mapping
            if input.start < mapping.source {
                let max = range.end.min(mapping.source);
                output.push(range.start..max);
                range = mapping.source..range.end;
                if range.is_empty() {
                    return output;
                }
            }

            // partial mapping
            if range.end < mapping.source + mapping.length {
                let min = mapping.target + range.start - mapping.source;
                let max = mapping.target + range.end - mapping.source;
                output.push(min..max);
                return output;
            }

            let max_number = range.end.min(mapping.source + mapping.length);
            let max = mapping.target + max_number - mapping.source;
            let min = mapping.target + range.start - mapping.source;
            output.push(min..max);
            range = max_number..range.end;
        }
        if !range.is_empty() {
            output.push(range);
        }
        output
    }
}

#[derive(Debug)]
struct CategoryRange {
    source: u64,
    target: u64,
    length: u64,
}

fn parse_input(input: &str) -> (Vec<u64>, HashMap<Category, CategoryRanges>) {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let map = parts.map(parse_map).collect();
    (seeds, map)
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let (_, nums) = input.split_once(':').unwrap();
    nums.trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_map(input: &str) -> (Category, CategoryRanges) {
    let (key, ranges) = input.split_once("map:\n").unwrap();
    let category = Category::from_str(key.trim());
    let ranges = ranges.lines().map(parse_range).collect();
    // eprintln!("{category:?}: {ranges:?}");
    (category, CategoryRanges::new(ranges))
}

fn parse_range(input: &str) -> CategoryRange {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let [target, source, length] = parts[..] else {
        panic!("invalid range: {input}");
    };
    CategoryRange {
        source: source.parse().unwrap(),
        target: target.parse().unwrap(),
        length: length.parse().unwrap(),
    }
}

mod part_1 {
    use std::collections::HashMap;

    use crate::{Category, CategoryRanges};

    fn solution(input: &str) -> u64 {
        let (seeds, mappings) = super::parse_input(input);
        seeds
            .iter()
            .copied()
            .map(|seeds| resolve_location(&mappings, seeds))
            .min()
            .unwrap()
    }

    fn resolve_location(mappings: &HashMap<Category, CategoryRanges>, seed: u64) -> u64 {
        let mut category = Category::SeedToSoil;
        let mut number = seed;
        loop {
            number = mappings.get(&category).unwrap().resolve_number(number);

            match category.next() {
                Some(c) => category = c,
                None => return number,
            }
        }
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 35);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 5 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use std::collections::HashMap;
    use std::ops::Range;

    use crate::{Category, CategoryRanges};

    fn solution(input: &str) -> u64 {
        let (seeds, mappings) = super::parse_input(input);
        let mut ranges = Vec::with_capacity(seeds.len() / 2);
        for i in (0..seeds.len()).step_by(2) {
            let range_start = seeds[i];
            let range_end = range_start + seeds[i + 1];
            let seed_range = range_start..range_end;
            ranges.push(seed_range);
        }
        let location_ranges = resolve_location(&mappings, ranges);
        location_ranges.into_iter().flatten().min().unwrap()
    }

    fn resolve_location(
        mappings: &HashMap<Category, CategoryRanges>,
        seeds: Vec<Range<u64>>,
    ) -> Vec<Range<u64>> {
        let mut category = Category::SeedToSoil;
        let mut ranges = seeds;
        loop {
            let mapping = mappings.get(&category).unwrap();
            ranges = ranges
                .into_iter()
                .flat_map(|range| mapping.resolve_numbers(range))
                .collect();

            match category.next() {
                Some(c) => category = c,
                None => return ranges,
            }
        }
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 46);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 5 Part 2: [{}]", solution(super::INPUT));
    }
}
