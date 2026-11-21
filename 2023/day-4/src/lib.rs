use std::collections::HashSet;

const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    numbers: Vec<u8>,
    winning_numbers: HashSet<u8>,
}

impl Card {
    fn winning_number_count(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.contains(n))
            .count()
    }

    fn winnings(&self) -> usize {
        let count = self.winning_number_count();
        if count == 0 {
            return 0;
        }
        2usize.pow(u32::try_from(count - 1).unwrap())
    }
}

fn parse_cards(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Card {
    let line = line.strip_prefix("Card").unwrap();
    let (id, line) = line.split_once(':').unwrap();
    let (numbers, winners) = line.split_once('|').unwrap();
    Card {
        id: id.trim().parse().unwrap(),
        numbers: numbers
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
        winning_numbers: winners
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect(),
    }
}

mod part_1 {
    fn solution(input: &str) -> usize {
        let cards = super::parse_cards(input);
        cards.map(|c| c.winnings()).sum()
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 13);
    }

    #[test]
    fn test_part_1() {
        eprintln!("Solution Day 4 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use crate::Card;

    fn solution(input: &str) -> usize {
        let cards: Vec<Card> = super::parse_cards(input).collect();
        let mut card_copies = vec![1; cards.len()];
        for (i, card) in cards.iter().enumerate() {
            let win_count = card.winning_number_count();
            if win_count == 0 {
                continue;
            }
            let copies = card_copies[i];
            for j in 1..=win_count {
                if i + j < card_copies.len() {
                    card_copies[i + j] += copies;
                }
            }
        }

        card_copies.iter().sum()
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 30);
    }

    #[test]
    fn test_part_2() {
        eprintln!("Solution Day 4 Part 2: [{}]", solution(super::INPUT));
    }
}
