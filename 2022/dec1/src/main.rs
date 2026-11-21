#![feature(test)]
extern crate test;

use anyhow::Result;
use itertools::Itertools;
use std::env;
use std::fs;

const INPUT: &str = include_str!("../input");

fn main() -> Result<()> {
    let arg = env::args()
        .collect::<Vec<_>>()
        .get(1)
        .unwrap_or(&"1".to_string())
        .parse::<usize>()
        .unwrap();
    let amount = solution_2_sorted_sum(arg);
    println!("{amount}");
    Ok(())
}

fn solution_1_max() -> u32 {
    INPUT
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|i| i.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

fn solution_2_sorted_sum(n: usize) -> u32 {
    INPUT
        .split("\n\n")
        .map(|e| {
            e.split("\n")
                .map(|i| i.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .take(n)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1() {
        assert_eq!(68775, solution_1_max());
    }

    #[test]
    fn test_solution2_part1() {
        assert_eq!(68775, solution_2_sorted_sum(1));
    }

    #[test]
    fn test_solution2_part2() {
        assert_eq!(202585, solution_2_sorted_sum(3));
    }

    #[bench]
    fn bench_single_max(b: &mut Bencher) {
        b.iter(|| solution_1_max());
    }

    #[bench]
    fn bench_single_sorted_sum(b: &mut Bencher) {
        b.iter(|| solution_2_sorted_sum(1));
    }

    #[bench]
    fn bench_top3_sorted_sum(b: &mut Bencher) {
        b.iter(|| solution_2_sorted_sum(3));
    }
}