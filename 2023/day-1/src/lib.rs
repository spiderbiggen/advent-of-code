const INPUT: &str = include_str!("../input_part_1");

mod part_1 {
    fn solution(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let chars = line.chars();
                let mut first_n: Option<usize> = None;
                let mut last_n: Option<usize> = None;
                for x in chars {
                    if let Some(digit) = x.to_digit(10) {
                        first_n.get_or_insert(digit as usize);
                        last_n.replace(digit as usize);
                    }
                }
                first_n.unwrap() * 10 + last_n.unwrap()
            })
            .sum()
    }

    #[test]
    fn test_process_example() {
        let example = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solution(example), 142);
    }

    #[test]
    fn test_process_input() {
        eprintln!("Solution Day 1 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    fn solution(input: &str) -> usize {
        input.lines().map(process_line).sum()
    }

    fn process_line(line: &str) -> usize {
        // assuming ascii is probably fine for aoc
        let ascii = line.as_bytes();

        let mut first_n: Option<usize> = None;
        let mut last_n: Option<usize> = None;
        for i in 0..ascii.len() {
            if ascii[i].is_ascii_digit() {
                let digit = (ascii[i] - b'0') as usize;
                first_n.get_or_insert(digit);
                last_n.replace(digit);
                continue;
            }

            for j in i..ascii.len() {
                if ascii[j].is_ascii_digit() {
                    break;
                }
                let num: usize = match &ascii[i..=j] {
                    s if s.starts_with(b"one") => 1,
                    s if s.starts_with(b"two") => 2,
                    s if s.starts_with(b"three") => 3,
                    s if s.starts_with(b"four") => 4,
                    s if s.starts_with(b"five") => 5,
                    s if s.starts_with(b"six") => 6,
                    s if s.starts_with(b"seven") => 7,
                    s if s.starts_with(b"eight") => 8,
                    s if s.starts_with(b"nine") => 9,
                    _ => continue,
                };
                first_n.get_or_insert(num);
                last_n.replace(num);
                break;
            }
        }
        first_n.unwrap() * 10 + last_n.unwrap()
    }

    #[test]
    fn test_process_example() {
        let example = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solution(example), 281);
    }

    #[test]
    fn test_process_input() {
        eprintln!("Solution Day 1 Part 2: [{}]", solution(super::INPUT));
    }
}
