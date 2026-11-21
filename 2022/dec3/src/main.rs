#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("../input");

fn main() {
    let result_part1 = algo2_part1();
    println!("Part1: {result_part1}");
    let result_part2 = algo2_part2();
    println!("Part2: {result_part2}");
}

fn algo1_part1() -> u32 {
    INPUT
        .lines()
        .map(|l| split_input(l))
        .map(|(left, right)| find_common(left, right))
        .map(|c| prioritize(c))
        .sum()
}

fn algo1_part2() -> u32 {
    let mut lines = INPUT.lines();
    let mut sum = 0u32;
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let c = find_common_3(first, second, third);
        sum += prioritize(c)
    }
    sum
}

fn algo2_part1() -> u32 {
    INPUT
    .lines()
    .map(|l| split_input(l))
    .map(|(left, right)| char_bitmask(left) & char_bitmask(right))
    .map(|c| bitmask_priority(c))
    .sum()
}

fn algo2_part2() -> u32 {
    let mut lines = INPUT.lines();
    let mut sum = 0u32;
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next())
    {
        let bitmask = char_bitmask(first) & char_bitmask(second) & char_bitmask(third);
        sum += bitmask_priority(bitmask)
    }
    sum
}



fn split_input<'a>(i: &'a str) -> (&'a str, &'a str) {
    let split = i.len() / 2;
    (&i[..split], &i[split..])
}

fn find_common(left: &str, right: &str) -> char {
    left.chars().find(|&c| right.contains(c)).unwrap()
}

fn find_common_3(first: &str, second: &str, third: &str) -> char {
    first
        .chars()
        .find(|&c| second.contains(c) && third.contains(c))
        .unwrap()
}

fn prioritize(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn char_bitmask(s: &str) -> u64 {
    s.bytes().fold(0u64, |rec, b| rec | 1u64 << (b - 0x40))
}

fn bitmask_priority(m: u64) -> u32 {
    match (0x40 + m.trailing_zeros()) as u8 {
        c @ b'a'..=b'z' => (c - b'a' + 1) as u32,
        c @ b'A'..=b'Z' => (c - b'A' + 27) as u32,
        _ => panic!(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(7889, algo1_part1());
    }
    #[test]
    fn test_solution1_part2() {
        assert_eq!(2825, algo1_part2());
    }

    #[test]
    fn test_solution2_part1() {
        assert_eq!(7889, algo2_part1());
    }
    #[test]
    fn test_solution2_part2() {
        assert_eq!(2825, algo2_part2());
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| algo1_part2());
    }

    #[bench]
    fn bench_solution2(b: &mut Bencher) {
        b.iter(|| algo2_part2());
    }
}
