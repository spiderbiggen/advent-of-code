use std::collections::BTreeMap;

const INPUT: &str = include_str!("../puzzle_input");

fn parse_input(input: &str) -> (&str, BTreeMap<&str, (&str, &str)>) {
    let (directions, forks) = input.split_once("\n\n").unwrap();
    (directions, forks.lines().map(parse_line).collect())
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let mut split = line.split(" = ");
    let key = split.next().unwrap();
    let value = split.next().unwrap();
    let directions = value.trim_start_matches('(').trim_end_matches(')');
    (key, directions.split_once(", ").unwrap())
}

fn find_min_steps(
    directions: &str,
    nodes: &BTreeMap<&str, (&str, &str)>,
    start: &str,
    finish_matcher: impl Fn(&str) -> bool,
) -> u64 {
    let mut node = start;
    let directions = directions.chars().cycle().enumerate();
    for (index, direction) in directions {
        if direction == 'L' {
            node = nodes.get(node).unwrap().0;
        } else {
            node = nodes.get(node).unwrap().1;
        }
        if finish_matcher(node) {
            return index as u64 + 1;
        }
    }
    unreachable!()
}

mod part_1 {
    fn solution(input: &str) -> u64 {
        let (directions, map) = super::parse_input(input);
        super::find_min_steps(directions, &map, "AAA", |n| n == "ZZZ")
    }

    const EXAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn test_parse_second_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT2), 6);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 8 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use std::cmp::min;
    use std::mem::swap;
    use std::ops::Div;

    fn solution(input: &str) -> u64 {
        let (directions, map) = super::parse_input(input);

        map.keys()
            .copied()
            .filter(|key| key.ends_with('A'))
            .map(|start| super::find_min_steps(directions, &map, start, |n| n.ends_with('Z')))
            .fold(1u64, |acc, value| (acc * value).div(gcd(acc, value)))
    }

    fn gcd(mut n: u64, mut m: u64) -> u64 {
        // Stein's binary GCD algorithm
        // Base cases: gcd(n, 0) = gcd(0, n) = n
        if n == 0 {
            return m;
        } else if m == 0 {
            return n;
        }

        // Extract common factor-2: gcd(2ⁱ n, 2ⁱ m) = 2ⁱ gcd(n, m)
        // and reducing until odd gcd(2ⁱ n, m) = gcd(n, m) if m is odd
        let k = {
            let k_n = n.trailing_zeros();
            let k_m = m.trailing_zeros();
            n >>= k_n;
            m >>= k_m;
            min(k_n, k_m)
        };

        loop {
            // Invariant: n odd
            debug_assert!(n % 2 == 1, "n = {} is even", n);

            if n > m {
                swap(&mut n, &mut m);
            }
            m -= n;

            if m == 0 {
                return n << k;
            }

            m >>= m.trailing_zeros();
        }
    }

    const EXAMPLE_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse_example_input() {
        assert_eq!(solution(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 8 Part 2: [{}]", solution(super::INPUT));
    }
}
