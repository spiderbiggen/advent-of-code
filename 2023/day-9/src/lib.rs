const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

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
