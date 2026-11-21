const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

#[derive(Debug, Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn winning_inputs(self) -> u64 {
        let range = 0..=self.time;
        range
            .into_iter()
            .filter(|m| (self.time - m) * m > self.distance)
            .count() as u64
    }
}

mod part_1 {
    use crate::Race;

    fn solution(input: &str) -> u64 {
        let races = parse_input(input);
        races.into_iter().map(Race::winning_inputs).product()
    }

    fn parse_input(input: &str) -> Vec<Race> {
        let mut lines = input.lines();
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();
        let times = time_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap());
        let distances = distance_line
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap());
        times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect()
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 288);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 6 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use crate::Race;

    fn solution(input: &str) -> u64 {
        let race = parse_input(input);
        race.winning_inputs()
    }

    fn parse_input(input: &str) -> Race {
        let mut lines = input.lines();
        let time_line = lines.next().unwrap();
        let distance_line = lines.next().unwrap();
        let (_, time_text) = time_line.split_once(' ').unwrap();
        let time_text = time_text.trim().replace(' ', "");
        let (_, distance_text) = distance_line.split_once(' ').unwrap();
        let distance_text = distance_text.trim().replace(' ', "");
        Race {
            time: time_text.parse().unwrap(),
            distance: distance_text.parse().unwrap(),
        }
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 71503);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 6 Part 2: [{}]", solution(super::INPUT));
    }
}
