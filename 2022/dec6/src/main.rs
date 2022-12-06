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

fn is_unique(bytes: &[u8]) -> bool {
    let mut mask = 0u32;
    for &byte in bytes {
        let index = 1u32 << (byte - b'a');
        if (mask & index) != 0 {
            return false;
        }
        mask |= index;
    }
    true
}

fn unique_window_end(input: &str, window_size: usize) -> usize {
    let offset = input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| is_unique(window))
        .unwrap()
        .0;
    offset + window_size
}

fn algo1_part1() -> usize {
    unique_window_end(INPUT, 4)
}

fn algo1_part2() -> usize {
    unique_window_end(INPUT, 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(1361, algo1_part1());
    }
    #[test]
    fn test_solution1_part2() {
        assert_eq!(3263, algo1_part2());
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| algo1_part2());
    }
}
