const INPUT: &str = include_str!("../puzzle_input");
const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

mod part_1 {
    use std::cmp::Ordering;
    use std::collections::BTreeMap;

    fn solution(input: &str) -> u64 {
        let mut hands = parse_input(input);
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank as u64 + 1))
            .sum()
    }

    fn parse_input(input: &str) -> Vec<Hand> {
        input.lines().map(Hand::from_line).collect()
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 6440);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 7 Part 1: [{}]", solution(super::INPUT));
    }

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl From<char> for Card {
        fn from(c: char) -> Self {
            match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("invalid card: {c}"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl From<&[Card; 5]> for HandType {
        fn from(value: &[Card; 5]) -> Self {
            let mut tree_map = BTreeMap::<Card, u8>::new();
            for &card in value {
                *tree_map.entry(card).or_default() += 1;
            }
            let mut hand_type = HandType::HighCard;
            for count in tree_map.values() {
                hand_type = match (count, hand_type) {
                    (2, HandType::HighCard) => HandType::OnePair,
                    (2, HandType::OnePair) => HandType::TwoPair,
                    (2, HandType::ThreeOfAKind) | (3, HandType::OnePair) => HandType::FullHouse,
                    (3, HandType::HighCard) => HandType::ThreeOfAKind,
                    (4, _) => HandType::FourOfAKind,
                    (5, _) => HandType::FiveOfAKind,
                    _ => hand_type,
                }
            }
            hand_type
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Hand {
        hand_type: HandType,
        cards: [Card; 5],
        bid: u64,
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.hand_type
                .cmp(&other.hand_type)
                .then(self.cards.cmp(&other.cards))
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Hand {
        fn from_line(line: &str) -> Hand {
            let mut cards = [Card::Two; 5];
            let (card_chars, bid) = line.split_once(' ').unwrap();
            for (i, c) in card_chars.chars().enumerate() {
                cards[i] = c.into();
            }
            Hand {
                hand_type: HandType::from(&cards),
                cards,
                bid: bid.parse().unwrap(),
            }
        }
    }
}

mod part_2 {
    use std::cmp::Ordering;
    use std::collections::BTreeMap;

    fn solution(input: &str) -> u64 {
        let mut hands = parse_input(input);
        hands.sort();
        hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| {
                // eprintln!("{hand:?}");
                hand.bid * (rank as u64 + 1)
            })
            .sum()
    }

    fn parse_input(input: &str) -> Vec<Hand> {
        input.lines().map(Hand::from_line).collect()
    }

    #[test]
    fn test_parse_example_input() {
        let input = super::EXAMPLE_INPUT;
        assert_eq!(solution(input), 5905);
    }

    #[test]
    fn test_parse_input() {
        eprintln!("Solution Day 7 Part 2: [{}]", solution(super::INPUT));
    }

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    enum Card {
        Joker,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Queen,
        King,
        Ace,
    }

    impl From<char> for Card {
        fn from(c: char) -> Self {
            match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Joker,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("invalid card: {c}"),
            }
        }
    }

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
    enum HandType {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }

    impl From<&[Card; 5]> for HandType {
        fn from(value: &[Card; 5]) -> Self {
            let mut tree_map = BTreeMap::<Card, u8>::new();
            for &card in value {
                *tree_map.entry(card).or_default() += 1;
            }
            let mut hand_type = HandType::HighCard;
            let mut jokers = *tree_map.get(&Card::Joker).unwrap_or(&0);
            let mut card_sets: Vec<_> = tree_map.into_iter().collect();
            card_sets.sort_by_key(|&(_, count)| count);
            card_sets.reverse();

            for (card, count) in card_sets {
                if card == Card::Joker {
                    continue;
                };
                hand_type = match (count + jokers, hand_type) {
                    (2, HandType::HighCard) => HandType::OnePair,
                    (2, HandType::OnePair) => HandType::TwoPair,
                    (2, HandType::ThreeOfAKind) | (3, HandType::OnePair) => HandType::FullHouse,
                    (3, HandType::HighCard) => HandType::ThreeOfAKind,
                    (4, _) => HandType::FourOfAKind,
                    (5, _) => HandType::FiveOfAKind,
                    _ => hand_type,
                };
                jokers = 0;
            }
            if jokers == 5 {
                hand_type = HandType::FiveOfAKind;
            }
            hand_type
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct Hand {
        hand_type: HandType,
        cards: [Card; 5],
        bid: u64,
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            self.hand_type
                .cmp(&other.hand_type)
                .then(self.cards.cmp(&other.cards))
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Hand {
        fn from_line(line: &str) -> Hand {
            let mut cards = [Card::Two; 5];
            let (card_chars, bid) = line.split_once(' ').unwrap();
            for (i, c) in card_chars.chars().enumerate() {
                cards[i] = c.into();
            }
            Hand {
                hand_type: HandType::from(&cards),
                cards,
                bid: bid.parse().unwrap(),
            }
        }
    }
}
