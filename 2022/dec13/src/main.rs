#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;

use std::cmp::Ordering;

use anyhow::{anyhow, bail, Result};

const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");

#[derive(Debug, PartialEq, Eq, Clone, Ord)]
enum Value {
    List(Vec<Value>),
    Int(usize),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::List(self_list), Self::List(other_list)) => {
                let mut iter_left = self_list.iter();
                let mut iter_right = other_list.iter();
                while let Some(left) = iter_left.next() {
                    if let Some(right) = iter_right.next() {
                        if let Some(comp) = left.partial_cmp(right) {
                            if comp != Ordering::Equal {
                                return Some(comp);
                            }
                        }
                    } else {
                        return Some(Ordering::Greater);
                    }
                }
                if iter_right.next().is_some() {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Self::Int(s), Self::Int(o)) => s.partial_cmp(o),
            (Self::Int(s), other) => Self::List(vec![Self::Int(*s)]).partial_cmp(other),
            (s, Self::Int(other)) => s.partial_cmp(&Self::List(vec![Self::Int(*other)])),
        }
    }
}

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result:?}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result:?}");
    let result = algo1_part2(INPUT_EXAMPLE);
    println!("Example2: {result:?}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result:?}");
}

fn parse_input(input: &str) -> Result<Vec<(Value, Value)>> {
    let groups = input.split("\n\n");
    let mut pairs: Vec<(Value, Value)> = Vec::with_capacity(groups.size_hint().0);

    for group in groups {
        if let Some((left, right)) = group.split_once('\n') {
            pairs.push((parse_line(left)?, parse_line(right)?))
        }
    }

    Ok(pairs)
}

fn parse_line(input: &str) -> Result<Value> {
    let mut stack: Vec<Value> = vec![];
    let mut iter = input.trim().bytes().peekable();
    let mut num: Option<usize> = None;
    while let Some(next) = iter.next() {
        if next == b',' || next == b']' {
            if let Some(n) = num {
                if let Some(Value::List(l)) = stack.last_mut() {
                    l.push(Value::Int(n));
                } else {
                    bail!("bruh what");
                }
                num = None;
            }
        }
        match next {
            n @ b'0'..=b'9' => {
                let a = num.get_or_insert(0);
                *a = *a * 10 + (n - b'0') as usize;
            }
            b'[' => stack.push(Value::List(vec![])),
            b']' if stack.len() > 1 => {
                if let Some(popped) = stack.pop() {
                    if let Some(Value::List(l)) = stack.last_mut() {
                        l.push(popped);
                    }
                }
            }
            b',' | b']' => {}
            b => bail!("Invalid input {b}"),
        }
    }
    stack.pop().ok_or(anyhow!("invalid stack"))
}

fn algo1_part1(input: &str) -> Result<usize> {
    let pairs = parse_input(input)?;
    let sum: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left <= right)
        .map(|(i, _)| i + 1)
        .sum();
    Ok(sum)
}

fn algo1_part2(input: &str) -> Result<usize> {
    let mut pairs = parse_input(input)?;
    let dividers = (
            Value::List(vec![Value::List(vec![Value::Int(2)])]),
            Value::List(vec![Value::List(vec![Value::Int(6)])]),
    );
    pairs.push(dividers.clone());
    let mut a = pairs
        .iter()
        .flat_map(|(left, right)| vec![left, right])
        .collect::<Vec<&Value>>();
    a.sort();
    let first = a.iter().enumerate().find(|(_, s)| ***s == dividers.0).ok_or(anyhow!("couldn't find first separator"))?.0 + 1;
    let second = a.iter().enumerate().find(|(_, s)| ***s == dividers.1).ok_or(anyhow!("couldn't find second separator"))?.0 + 1;
    Ok(first * second)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution1_part2(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }

    #[bench]
    fn bench_solution2_part2(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }
}
