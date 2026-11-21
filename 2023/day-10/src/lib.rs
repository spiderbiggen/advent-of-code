const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
const EXAMPLE_INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

struct Neighbours {
    pub(crate) north: Option<Direction>,
    pub(crate) east: Option<Direction>,
    pub(crate) west: Option<Direction>,
    pub(crate) south: Option<Direction>,
}

pub(crate) enum Direction {
    Starting,
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    None,
}

pub(crate) struct PipeGrid {
    pub(crate) width: usize,
    pub(crate) pipes: Vec<Direction>,
}

pub(crate) struct PipeLoop {
    pub(crate) width: usize,
    pub(crate) pipes: Vec<usize>,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Starting,
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            'F' => Self::SouthEast,
            '7' => Self::SouthWest,
            '.' => Self::None,
            _ => panic!("invalid input {c}"),
        }
    }

    fn starting_direction(neighbors: Neighbours) -> Self {
        todo!()
    }

    fn connects_to(&self, neighbors: Neighbours) -> Neighbours {
        todo!()
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_differences(input: &[i64]) -> Vec<i64> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

mod part_1 {
    fn solution(input: &str) -> i64 {
        let variables = super::parse_input(input);
        variables.iter().map(|v| solve_differences(v)).sum()
    }

    fn solve_differences(input: &[i64]) -> i64 {
        if input.iter().all(|&a| a == 0) {
            return 0;
        }
        let differences = super::find_differences(input);
        input.last().unwrap() + solve_differences(&differences)
    }

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 114);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 9 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    fn solution(input: &str) -> i64 {
        let variables = super::parse_input(input);
        variables.iter().map(|v| solve_differences(v)).sum()
    }

    fn solve_differences(input: &[i64]) -> i64 {
        if input.iter().all(|&a| a == 0) {
            return 0;
        }
        let differences = super::find_differences(input);
        input.first().unwrap() - solve_differences(&differences)
    }

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 9 Part 2: [{}]", solution(super::INPUT));
    }
}
