#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;
const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result}");
    //    let result = algo1_part2(INPUT_EXAMPLE);
    //    println!("Example2: {result}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result}");
    let result = algo1_part2(INPUT);
//    println!("Part2: {result}");
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Noop,
    AddX(isize),
}

fn parse_input(input: &'static str) -> impl Iterator<Item = Instruction> {
    input.lines().filter_map(|s| match s {
        "noop" => Some(Instruction::Noop),
        s => s
            .split_once(char::is_whitespace)
            .map(|(a, i)| match a {
                "addx" => i.parse::<isize>().ok().map(|i| Instruction::AddX(i)),
                _ => None,
            })
            .flatten(),
    })
}

fn algo1_part1(input: &'static str) -> isize {
    let mut register_x = 1isize;
    let mut sum = 0isize;
    let mut counter = 20isize;
    for instruction in parse_input(input) {
        match instruction {
            Instruction::Noop => {
                counter += 1;
                if counter % 40 == 0 {
                    println!("N{}: {register_x}", counter - 20);
                    sum += register_x * (counter - 20);
                }
            }
            Instruction::AddX(x) => {
                counter += 2;
                if counter % 40 == 1 {
                    println!("M{}: {register_x}", counter - 21);
                    sum += register_x * (counter - 21);
                } else if counter % 40 == 0 {
                    println!("M{}: {register_x}", counter - 20);
                    sum += register_x * (counter - 20);
                }
                register_x += x;
            }
        }
    }
    sum
}

fn algo1_part2(input: &'static str) {
    const WIDTH: usize = 40;
    const HEIGHT: usize = 6;
    let mut display = [false; WIDTH * HEIGHT];

    let mut register_x = 1isize;

    let mut instructions = parse_input(input);
    let mut current_instruction = (0isize, Instruction::Noop);
    for i in 0..240 {
        if i == current_instruction.0 {
            match current_instruction.1 {
                Instruction::Noop => {}
                Instruction::AddX(x) => register_x += x,
            }
        }
        if i >= current_instruction.0 {
            let Some(instr) = instructions.next() else {break};
            current_instruction = match instr {
                Instruction::Noop => (i + 1, instr),
                Instruction::AddX(_) => (i + 2, instr),
            }
        }
        let px = i % WIDTH as isize;
        if px >= register_x - 1 && px <= register_x + 1 {
            display[i as usize] = true;
        }
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if display[y * WIDTH + x] {'#'}else {'.'});
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(1736, algo1_part1(INPUT));
    }


    #[bench]
    fn bench_solution_part1(b: &mut Bencher) {
        b.iter(|| algo1_part1(INPUT));
    }

    #[bench]
    fn bench_solution_part2(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }
}
