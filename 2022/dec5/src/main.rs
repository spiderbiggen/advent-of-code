#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;
const INPUT: &str = include_str!("../input");

fn main() {
    let result = algo1_part1();
    println!("Part1: {result}");
    let result = algo1_part2();
    println!("Part2: {result}");
}

#[derive(Debug)]
struct MoveTo(usize, usize, usize);

impl From<&str> for MoveTo {
    fn from(s: &str) -> Self {
        if s.starts_with("move") {
            let [_, a, _, b, _, c] = s.split_whitespace().next_chunk().unwrap();
            Self(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap())
        } else {
            panic!();
        }
    }
}

fn algo1_part1() -> String {
    let mut lines = INPUT.lines().peekable();
    let mut input_lines: Vec<&str> = Vec::with_capacity(10);
    while let Some(line) = lines.next_if(|s| s.starts_with('[')) {
        input_lines.push(line);
    }
    input_lines.reverse();
    let line = lines.next().unwrap();
    let mut stacks: Vec<Vec<char>> =
        vec![Vec::with_capacity(input_lines.len()); line.trim().split_whitespace().count()];
    for s in input_lines.iter() {
        for (i, c) in s.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks.get_mut(i).unwrap().push(c)
            }
        }
    }
    // skip empty line
    lines.next();
    for MoveTo(n, from, to) in lines.map(|s| s.into()) {
        for _ in 1..=n {
            let c = stacks.get_mut(from - 1).unwrap().pop().unwrap();
            stacks.get_mut(to - 1).unwrap().push(c);
        }
    }
    stacks.iter().filter_map(|v| v.last()).collect()
}

fn algo1_part2() -> String {
    let mut lines = INPUT.lines().peekable();
    let mut input_lines: Vec<&str> = Vec::with_capacity(10);
    while let Some(line) = lines.next_if(|s| s.starts_with('[')) {
        input_lines.push(line);
    }
    input_lines.reverse();
    let line = lines.next().unwrap();
    let mut stacks: Vec<Vec<char>> =
    vec![Vec::with_capacity(input_lines.len()); line.trim().split_whitespace().count()];
    for s in input_lines.iter() {
        for (i, c) in s.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks.get_mut(i).unwrap().push(c)
            }
        }
    }
    // skip empty line
    lines.next();
    let instructions: Vec<MoveTo> = lines.map(|s| s.into()).collect();
    for MoveTo(n, from, to) in instructions.into_iter() {
        let mut temp = vec![];
        for _ in 1..=n {
            let c = stacks.get_mut(from - 1).unwrap().pop().unwrap();
            temp.push(c)
        }
        temp.reverse();

        stacks.get_mut(to - 1).unwrap().append(&mut temp);
    }
    stacks.iter().filter_map(|v| v.last()).collect()
}

