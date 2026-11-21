#![feature(test)]
extern crate test;

const INPUT: &str = include_str!("../input");

fn main() {
    let sum: u32 = algo();
    println!("{sum}")
}

fn algo() -> u32 {
    INPUT.split("\n").map(|s| Move::round_score(s)).sum()
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Strategy {
    Lose,
    Draw,
    Win,
}

#[derive(Debug, Clone, Copy)]
enum Player {
    Elf(Move),
    You(Strategy),
}

impl From<&str> for Player {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Elf(Move::Rock),
            "B" => Self::Elf(Move::Paper),
            "C" => Self::Elf(Move::Scissors),
            "X" => Self::You(Strategy::Lose),
            "Y" => Self::You(Strategy::Draw),
            "Z" => Self::You(Strategy::Win),
            _ => panic!(""),
        }
    }
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn round_score(s: &str) -> u32 {
        let moves = s
            .split_whitespace()
            .take(2)
            .map(|a| a.into())
            .collect::<Vec<Player>>();
        match &moves[..] {
            [Player::Elf(action), Player::You(strategy)] => {
                strategy.score() + strategy.action(action).score()
            }
            _ => panic!("invalid round"),
        }
    }
}

impl Strategy {
    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    fn action(&self, o: &Move) -> Move {
        match (self, o) {
            (Self::Lose, Move::Rock) => Move::Scissors,
            (Self::Lose, Move::Paper) => Move::Rock,
            (Self::Lose, Move::Scissors) => Move::Paper,
            (Self::Win, Move::Rock) => Move::Paper,
            (Self::Win, Move::Paper) => Move::Scissors,
            (Self::Win, Move::Scissors) => Move::Rock,
            (Self::Draw, _) => *o,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1() {
        assert_eq!(13448, algo());
    }

    #[bench]
    fn bench_top3_sorted_sum(b: &mut Bencher) {
        b.iter(|| algo());
    }
}
