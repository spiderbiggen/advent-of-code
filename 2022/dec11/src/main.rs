#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;

use std::collections::VecDeque;
const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result}");
    let result = algo1_part2(INPUT_EXAMPLE);
    println!("Example2: {result}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result}");
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
    activity: usize,
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .filter_map(|s| {
            let mut lines = s.lines();
            lines.next()?;
            let item_ids = lines
                .next()
                .map(|l| l.split_at(18))
                .map(|(_, p)| p.split(", ").filter_map(|d| d.parse::<usize>().ok()))?
                .collect();
            let operation = match lines.next()?.split_at(23).1.split_once(' ')? {
                ("*", "old") => Operation::Square,
                ("*", d) => Operation::Mul(d.parse().ok()?),
                ("+", d) => Operation::Add(d.parse().ok()?),
                _ => return None,
            };
            Some(Monkey {
                items: item_ids,
                operation: operation,
                test: lines.next()?.split_at(21).1.parse().ok()?,
                if_true: lines.next()?.split_at(29).1.parse().ok()?,
                if_false: lines.next()?.split_at(30).1.parse().ok()?,
                activity: 0,
            })
        })
        .collect()
}

fn algo1_part1(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys.get_mut(m).unwrap().items.pop_front() {
                let mut monkey = monkeys.get_mut(m).unwrap();
                monkey.activity += 1;
                let test_value = monkey.test;
                let if_true = monkey.if_true;
                let if_false = monkey.if_false;
                let mut result = match monkey.operation {
                    Operation::Add(x) => item + x,
                    Operation::Mul(x) => item * x,
                    Operation::Square => item * item,
                };
                result = result / 3;
                if result % test_value == 0 {
                    monkeys.get_mut(if_true).unwrap().items.push_back(result);
                } else {
                    monkeys.get_mut(if_false).unwrap().items.push_back(result);
                }
            }
        }
    }
    monkeys.sort_by_key(|m| m.activity);
    monkeys.iter().rev().take(2).map(|m| m.activity).product()
}

fn algo1_part2(input: &'static str) -> usize {
    let mut monkeys = parse_input(input);
    let common_divisor: usize = monkeys.iter().map(|m| m.test).product();
    for _ in 0..10_000 {
        for m in 0..monkeys.len() {
            while let Some(item) = monkeys.get_mut(m).unwrap().items.pop_front() {
                let mut monkey = monkeys.get_mut(m).unwrap();
                monkey.activity += 1;
                let test_value = monkey.test;
                let if_true = monkey.if_true;
                let if_false = monkey.if_false;
                let mut result = match monkey.operation {
                    Operation::Add(x) => item.overflowing_add(x).0,
                    Operation::Mul(x) => item.overflowing_mul(x).0,
                    Operation::Square => item.overflowing_mul(item).0,
                };
                result = result % common_divisor;
                if result % test_value == 0 {
                    monkeys.get_mut(if_true).unwrap().items.push_back(result);
                } else {
                    monkeys.get_mut(if_false).unwrap().items.push_back(result);
                }
            }
        }
    }
    monkeys.sort_by_key(|m| m.activity);
    monkeys.iter().rev().take(2).map(|m| m.activity).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(1736, algo1_part1(INPUT));
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
