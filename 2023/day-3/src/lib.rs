use std::collections::HashMap;

const INPUT: &str = include_str!("../input_part_1");
const EXAMPLE_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

mod part_1 {
    use crate::Schematic;

    fn solution(input: &str) -> usize {
        let schematic: Schematic = input.into();
        schematic
            .map
            .values()
            .filter_map(|&part| schematic.get_part_number(part))
            .sum()
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 4361);
    }

    #[test]
    fn test_part_1() {
        eprintln!("Solution Day 3 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    use crate::Schematic;

    fn solution(input: &str) -> usize {
        let schematic: Schematic = input.into();
        schematic
            .map
            .values()
            .filter_map(|&part| schematic.get_gear_ratio(part))
            .sum()
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 467_835);
    }

    #[test]
    fn test_part_2() {
        eprintln!("Solution Day 3 Part 2: [{}]", solution(super::INPUT));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Position(usize, usize);

#[derive(Debug, Copy, Clone)]
struct SchematicPart {
    pos: Position,
    r#type: PartType,
}

#[derive(Debug, Copy, Clone)]
enum PartType {
    Number { number: usize, len: usize },
    Symbol(char),
}

impl SchematicPart {
    fn len(&self) -> usize {
        match self.r#type {
            PartType::Number { len, .. } => len,
            PartType::Symbol(_) => 1,
        }
    }
}

struct Schematic {
    map: HashMap<Position, SchematicPart>,
    max_len: usize,
}

impl Schematic {
    fn new() -> Self {
        Schematic {
            map: HashMap::new(),
            max_len: 0,
        }
    }

    fn put(&mut self, part: SchematicPart) {
        self.map.insert(part.pos, part);
        self.max_len = self.max_len.max(part.len())
    }

    fn get_part_number(&self, part: SchematicPart) -> Option<usize> {
        let PartType::Number { number, len } = part.r#type else {
            return None;
        };

        // find adjacent parts
        let x_min = part.pos.0.saturating_sub(1);
        let y_min = part.pos.1.saturating_sub(1);
        let x_max = part.pos.0.saturating_add(len);
        let y_max = part.pos.1.saturating_add(1);
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                if let Some(p) = self.map.get(&Position(x, y)) {
                    if matches!(p.r#type, PartType::Symbol(_)) {
                        return Some(number);
                    }
                }
            }
        }
        None
    }

    fn get_gear_ratio(&self, part: SchematicPart) -> Option<usize> {
        if !matches!(part.r#type, PartType::Symbol('*')) {
            return None;
        }

        let mut product: Option<usize> = None;

        // find adjacent parts
        let x_min = part.pos.0.saturating_sub(self.max_len + 1);
        let y_min = part.pos.1.saturating_sub(1);
        let x_max = part.pos.0.saturating_add(1);
        let y_max = part.pos.1.saturating_add(1);
        for x in (x_min..=x_max).rev() {
            for y in y_min..=y_max {
                if let Some(p) = self.map.get(&Position(x, y)) {
                    if let PartType::Number { number, len } = p.r#type {
                        if x + len < part.pos.0 {
                            continue;
                        }
                        match product {
                            None => product = Some(number),
                            Some(product) => return Some(product * number),
                        }
                    }
                }
            }
        }
        None
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let mut schematic = Schematic::new();
        value
            .lines()
            .enumerate()
            .for_each(|(y, line)| crate::parse_line_into(y, line, &mut schematic));
        schematic
    }
}

fn parse_line_into(y: usize, line: &str, schematic: &mut Schematic) {
    let mut num: Option<usize> = None;
    let mut start: usize = 0;
    let mut len: usize = 0;
    for (index, byte) in line.as_bytes().iter().enumerate() {
        if byte.is_ascii_digit() {
            let digit = (byte - b'0') as usize;
            match num {
                None => {
                    num = {
                        start = index;
                        Some(digit)
                    }
                }
                Some(n) => num = Some(n * 10 + digit),
            }
            len += 1;
        } else {
            if let Some(n) = num {
                schematic.put(SchematicPart {
                    pos: Position(start, y),
                    r#type: PartType::Number { number: n, len },
                });
                num = None;
                len = 0;
            }

            if !byte.is_ascii_alphanumeric() && *byte != b'.' {
                schematic.put(SchematicPart {
                    pos: Position(index, y),
                    r#type: PartType::Symbol(*byte as char),
                });
            }
        }
    }
    if let Some(n) = num {
        schematic.put(SchematicPart {
            pos: Position(start, y),
            r#type: PartType::Number { number: n, len },
        });
    }
}
