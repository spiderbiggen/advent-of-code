#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;
use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");
#[derive(Debug, Clone)]
enum File<'a> {
    Dir(HashMap<&'a str, Self>),
    File(usize),
}

impl<'a> File<'a> {
    fn child(&mut self, dir: &str) -> Option<&mut Self> {
        match self {
            Self::Dir(files) => files.get_mut(dir),
            Self::File(_) => None,
        }
    }

    fn space(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Dir(files) => files.iter().map(|(_, f)| f.space()).sum(),
        }
    }

    fn add_children(&mut self, m: HashMap<&'a str, Self>) {
        match self {
            Self::File(_) => (),
            Self::Dir(files) => {
                files.extend(m);
            }
        }
    }

    fn find(&mut self, path: &Path) -> Option<&mut Self> {
        let mut current = self;
        for part in path.components() {
            current = match part {
                Component::RootDir => current,
                Component::Normal(s) => current.child(s.to_str()?)?,
                _ => return None,
            }
        }
        Some(current)
    }
}

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example: {result}");
    let result = algo1_part2(INPUT_EXAMPLE);
    println!("Example2: {result}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result}");
}

fn parse_commands<'a>(input: &'a str) -> File<'a> {
    let mut root: File = File::Dir(HashMap::new());
    let mut path = PathBuf::from("/");
    for cmd in input.split('$').map(|s| s.trim_start()) {
        let mut lines = cmd.lines();
        let Some(line) = lines.next().map(|s| s.trim()) else {continue};

        if let Some(("cd", dir)) = line.split_once(' ') {
            match dir {
                "/" => {
                    path.clear();
                    path.push("/");
                }
                ".." => {
                    path.pop();
                }
                c => path.push(c),
            }
        } else if "ls" == line {
            let Some(current) = root.find(&path) else {continue};
            let mut map: HashMap<&'a str, File> = HashMap::with_capacity(lines.size_hint().0);
            for (a, path) in lines.filter_map(|s| s.split_once(' ')) {
                match a {
                    "dir" => {
                        if map.get(path).is_none() {
                            map.insert(path, File::Dir(HashMap::new()));
                        }
                    }
                    digits => {
                        digits
                            .parse::<usize>()
                            .map(|size| map.insert(path, File::File(size)))
                            .unwrap();
                    }
                }
            }
            current.add_children(map)
        }
    }
    root
}

fn algo1_part1(input: &str) -> usize {
    let tree = parse_commands(input);
    find_nested_dirs("/", tree)
        .iter()
        .map(|(_, d)| d.space())
        .filter(|&a| a <= 100_000)
        .sum()
}

fn find_nested_dirs<'a>(p: &'a str, file: File<'a>) -> Vec<(&'a str, File<'a>)> {
    match file {
        File::File(_) => vec![],
        File::Dir(f) => {
            let mut a = vec![(p, File::Dir(f.clone()))];
            a.extend(f.into_iter().flat_map(|(p, f)| find_nested_dirs(p, f)));
            a
        }
    }
}

fn algo1_part2(input: &str) -> usize {
    const DISK_SIZE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;
    let tree = parse_commands(input);
    let remove_space = REQUIRED_SPACE - (DISK_SIZE - tree.space());
    find_nested_dirs("/", tree)
        .iter()
        .map(|(_, d)| d.space())
        .filter(|&a| a >= remove_space)
        .min()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_solution1_part1() {
        assert_eq!(2104783, algo1_part1(INPUT));
    }
    #[test]
    fn test_solution1_part2() {
        assert_eq!(5883165, algo1_part2(INPUT));
    }

    #[bench]
    fn bench_solution1(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }
}
