#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;
const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result}");
    let result = algo1_part2(INPUT_EXAMPLE);
    println!("Example2: {result}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result}");
}

fn parse_input<'a>(input: &'a str) -> (usize, Vec<u8>) {
    let mut lines = input.lines();
    let line = lines.next().unwrap();
    let size = line.len();
    let mut v: Vec<u8> = line.bytes().map(|b| b - 48).collect();
    lines.for_each(|line| v.extend(line.bytes().map(|b| b - 48)));
    (size, v)
}

fn algo1_part1(input: &str) -> usize {
    let (size, v) = parse_input(input);
    find_tall_trees(size, v)
}

fn find_tall_trees(size: usize, v: Vec<u8>) -> usize {
    let max = size - 1;
    let mut grid: Vec<bool> = vec![false; v.len()];
    for (y, _) in v.iter().step_by(size).take(size).enumerate() {
        let mut min_row = 0;
        for (x, &cell) in v.iter().skip(y * size).take(size).enumerate() {
            if x == 0 || x == max || y == 0 || y == max {
                grid[y * size + x] = true;
            }
            if cell > min_row {
                min_row = cell;
                grid[y * size + x] = true;
            }
        }
        min_row = 0;
        for (x, &cell) in v.iter().skip(y * size).take(size).enumerate().rev() {
            if cell > min_row {
                min_row = cell;
                grid[y * size + x] = true;
            }
        }
    }
    for (x, _) in v.iter().enumerate().take(size) {
        let mut min_col = 0;
        for (y, &cell) in v.iter().skip(x).step_by(size).take(size).enumerate() {
            if cell > min_col {
                min_col = cell;
                grid[y * size + x] = true;
            }
        }
        min_col = 0;
        for (y, &cell) in v.iter().skip(x).step_by(size).take(size).enumerate().rev() {
            if cell > min_col {
                min_col = cell;
                grid[y * size + x] = true;
            }
        }
    }

//    grid.chunks(size)
//        .map(|r| {
//            r.iter()
//                .map(|&b| if b { 49 } else { 48 })
//                .collect::<Vec<u8>>()
//        })
//        .map(|v| String::from_utf8(v).unwrap())
//        .for_each(|s| println!("{s}"));
    grid.iter().filter(|&&cell| cell).count()
}

fn algo1_part2(input: &str) -> usize {
    let (size, v) = parse_input(input);
    find_tree_view_distance(size, v)
}

fn find_tree_view_distance(size: usize, v: Vec<u8>) -> usize {
    let mut grid: Vec<usize> = vec![0; v.len()];
    for (i, &tree) in v.iter().enumerate() {
        let y = i / size;
        let x = i - y * size;

        let mut top = 0;
        let mut left = 0;
        let mut right = 0;
        let mut bottom = 0;

        // ^
        for j in (1..size).take_while(|j| i.checked_sub(j * size).is_some()) {
            top += 1;
            if v[i - j * size] >= tree {
                break;
            }
        }
        // v
        for j in (1..size).take_while(|j| i + j * size < v.len()) {
            bottom += 1;
            if v[i + j * size] >= tree {
                break;
            }
        }


        // <-
        for j in (1..size).take_while(|&j| x.checked_sub(j).is_some()) {
            left += 1;
            if v[i - j] >= tree {
                break;
            }
        }
        // ->
        for j in (1..size).take_while(|j| x + j < size) {
            right += 1;
            if v[i + j] >= tree {
                break;
            }
        }

        grid[i] = top * right * left * bottom;
    }

//    grid.chunks(size)
//        .map(|r| {
//            r.iter()
//                .map(|&b| format!("{b:04X}"))
//                .collect::<Vec<String>>()
//                .join(",")
//        })
//        .for_each(|s| println!("{s}"));
    *grid.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(1736, algo1_part1(INPUT));
    }
    #[test]
    fn test_solution1_part2() {
        assert_eq!(268800, algo1_part2(INPUT));
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
