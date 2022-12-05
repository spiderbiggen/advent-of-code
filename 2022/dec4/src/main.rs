#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("../input");

struct Range(u32, u32);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.0 <= other.1 && self.1 >= other.0
    }
}

fn main() {
    let result_part1 = algo2_part1();
    println!("Part1: {result_part1}");
    let result_part2 = algo2_part2();
    println!("Part2: {result_part2}");
}

fn algo1_part1() -> usize {
    algo1_parse_input()
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .count()
}

fn algo1_part2() -> usize {
    algo1_parse_input()
        .filter(|(left, right)| left.overlaps(right))
        .count()
}

fn algo2_part1() -> usize {
    algo2_parse_input()
        .filter(|(left, right)| left.contains(right) || right.contains(left))
        .count()
}

fn algo2_part2() -> usize {
    algo2_parse_input()
        .filter(|(left, right)| left.overlaps(right))
        .count()
}

fn algo3_part2() -> usize {
    let mut sum = 0;
    for (left, right) in algo3_parse_input() {
        if left.overlaps(&right) {
            sum += 1;
        }
    }
    sum
}

fn algo4_part2() -> usize {
    let mut sum = 0;
    for line in INPUT.lines() {
        let Some((a, b)) = line.split_once(',') else {continue};
        let Some(((a0, a1), (b0, b1))) = a.split_once('-').zip(b.split_once('-')) else {continue};
        let Some((a0, a1)) = parse_u32(a0).zip(parse_u32(a1)) else {continue};
        let Some((b0, b1)) = parse_u32(b0).zip(parse_u32(b1)) else {continue};
        if Range(a0, a1).overlaps(&Range(b0, b1)) {
            sum += 1;
        }
    }
    sum
}

fn algo1_parse_input() -> impl Iterator<Item = (Range, Range)> {
    INPUT
        .lines()
        .filter_map(|l| {
            let arr: Option<[u32; 4]> = l
                .split([',', '-'])
                .take(4)
                .map(|p| p.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .ok();
            arr
        })
        .filter_map(|v| {
            if let [a0, a1, b0, b1] = &v[..] {
                Some((Range(*a0, *a1), Range(*b0, *b1)))
            } else {
                None
            }
        })
}

fn algo2_parse_input() -> impl Iterator<Item = (Range, Range)> {
    INPUT
        .lines()
        .filter_map(|l| l.split_once(','))
        .filter_map(|(a, b)| a.split_once('-').zip(b.split_once('-')))
        .filter_map(|((a0, a1), (b0, b1))| {
            let left = parse_u32(a0).zip(parse_u32(a1));
            let right = parse_u32(b0).zip(parse_u32(b1));
            left.zip(right)
        })
        .map(|((a0, a1), (b0, b1))| (Range(a0, a1), Range(b0, b1)))
}

fn algo3_parse_input() -> impl Iterator<Item = (Range, Range)> {
    INPUT
    .lines()
    .filter_map(|l| {
        let Some((a, b)) = l.split_once(',') else {return None};
        let Some(((a0, a1), (b0, b1))) = a.split_once('-').zip(b.split_once('-')) else {return None};
        let Some((a0, a1)) = parse_u32(a0).zip(parse_u32(a1)) else {return None};
        let Some((b0, b1)) = parse_u32(b0).zip(parse_u32(b1)) else {return None};
        Some((Range(a0, a1), Range(b0, b1)))
    })
}

fn parse_u32(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(464, algo1_part1());
    }
    #[test]
    fn test_solution1_part2() {
        assert_eq!(770, algo1_part2());
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

    #[bench]
    fn bench_solution3(b: &mut Bencher) {
        b.iter(|| algo3_part2());
    }

    #[bench]
    fn bench_solution4(b: &mut Bencher) {
        b.iter(|| algo4_part2());
    }
}
