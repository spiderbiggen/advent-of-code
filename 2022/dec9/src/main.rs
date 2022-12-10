#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;

use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");
const INPUT_EXAMPLE2: &str = include_str!("../input_example2");

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Self::North,
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            c => panic!("invalid direction: {c}")
        }
    }
}

#[derive(Debug, Default)]
struct RopeBridge {
    head: Coordinates,
    tail: Vec<Coordinates>,
    visited: HashSet<Coordinates>,
}

impl RopeBridge {
    fn take_steps(&mut self, dir: Direction, steps: usize) {
        for _ in 0..steps {
            self.head = self.head.move_to(dir);
            let mut prev = self.head;
            for coord in self.tail.iter_mut() {
                *coord = (*coord).close_gap(prev);
                prev = *coord;
            }
            self.visited.insert(*self.tail.last().unwrap());
        }
    }

    fn print(&self) {
        let (x_min, y_min, x_max, y_max) = self.visited.iter().fold((0, 0, 0, 0), |(x_min, y_min, x_max, y_max), b| {
            (x_min.min(b.0), y_min.min(b.1), x_max.max(b.0), y_max.max(b.1))
        });
        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                print!("{}", if self.visited.contains(&Coordinates(x, y)) { 'X' } else { '.' });
            }
            println!();
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy, Hash, PartialOrd, Ord)]
struct Coordinates(isize, isize);

impl Coordinates {
    const GAP: isize = 1;
    fn move_to(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self(self.0, self.1 + 1),
            Direction::East => Self(self.0 + 1, self.1),
            Direction::South => Self(self.0, self.1 - 1),
            Direction::West => Self(self.0 - 1, self.1),
        }
    }

    fn close_gap(self, head: Self) -> Self {
        let delta_x = head.0 - self.0;
        let delta_y = head.1 - self.1;
        if delta_x.abs() > Self::GAP || delta_y.abs() > Self::GAP {
            Self(
                self.0 + delta_x.signum(),
                self.1 + delta_y.signum(),
            )
        } else {
            self
        }
    }
}

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result}");
    let result = algo1_part2(INPUT_EXAMPLE2);
    println!("Example2: {result}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result}");
}

fn parse_input(input: &'static str) -> impl Iterator<Item=(Direction, usize)> {
    input.lines()
        .filter_map(|s| s.rsplit_once(char::is_whitespace))
        .map(|(a, b)| (Direction::from(a), b.parse::<usize>().unwrap()))
}

fn algo1_part1(input: &'static str) -> usize {
    let mut bridge: RopeBridge = RopeBridge {
        tail: vec![Default::default()],
        ..Default::default()
    };
    for (dir, steps) in parse_input(input) {
        bridge.take_steps(dir, steps);
    }

    bridge.visited.len()
}


fn algo1_part2(input: &'static str) -> usize {
    let mut bridge: RopeBridge = RopeBridge {
        tail: vec![Default::default(); 9],
        ..Default::default()
    };
    for (dir, steps) in parse_input(input) {
        bridge.take_steps(dir, steps);
    }
    bridge.visited.len()
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(1736, algo1_part1(INPUT));
    }

    #[test]
    fn test_solution1_part2() {
        assert_eq!(268800, algo1_part2(INPUT));
    }

    #[bench]
    fn bench_solution_part1(b: &mut Bencher) {
        b.iter(|| algo1_part1(INPUT));
    }

    #[bench]
    fn bench_solution_part2(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }
}
