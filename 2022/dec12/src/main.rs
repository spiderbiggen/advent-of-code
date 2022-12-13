#![feature(iter_next_chunk)]
#![feature(test)]
extern crate test;

use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT: &str = include_str!("../input");
const INPUT_EXAMPLE: &str = include_str!("../input_example");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(i16, i16);

#[derive(Debug, Clone)]
struct Field {
    size: usize,
    height_map: Vec<u8>,
    start: Coordinate,
    end: Coordinate,
}

impl Field {
    fn get_height(&self, c: Coordinate) -> u8 {
        self.height_map[c.1 as usize * self.size + c.0 as usize]
    }

    fn is_reachable_from_height(&self, height: u8, other: Coordinate, inverse: bool) -> bool {
        if inverse {
            self.get_height(other) >= height - 1
        } else {
            self.get_height(other) <= height + 1
        }
    }

    fn get_reachable_neigbors(&self, c: Coordinate, inverse: bool) -> Vec<Coordinate> {
        let mut v = Vec::with_capacity(4);
        let height = self.get_height(c);
        if c.0 - 1 >= 0 {
            let other = Coordinate(c.0 - 1, c.1);
            if self.is_reachable_from_height(height, other, inverse) {
                v.push(other);
            }
        }
        if c.0 + 1 < self.size as i16 {
            let other = Coordinate(c.0 + 1, c.1);
            if self.is_reachable_from_height(height, other, inverse) {
                v.push(other);
            }
        }
        if c.1 - 1 >= 0 {
            let other = Coordinate(c.0, c.1 - 1);
            if self.is_reachable_from_height(height, other, inverse) {
                v.push(other);
            }
        }
        if c.1 + 1 < (self.height_map.len() / self.size) as i16 {
            let other = Coordinate(c.0, c.1 + 1);
            if self.is_reachable_from_height(height, other, inverse) {
                v.push(other);
            }
        }
        v
    }
}

fn main() {
    let result = algo1_part1(INPUT_EXAMPLE);
    println!("Example1: {result:?}");
    let result = algo1_part2(INPUT_EXAMPLE);
    println!("Example2: {result:?}");
    let result = algo1_part1(INPUT);
    println!("Part1: {result:?}");
    let result = algo1_part2(INPUT);
    println!("Part2: {result:?}");
}

fn parse_input(input: &'static str) -> Result<Field> {
    let mut start: Option<Coordinate> = None;
    let mut end: Option<Coordinate> = None;
    let mut iter = input.lines().enumerate().peekable();
    let size = iter
        .peek()
        .map(|(_, line)| line.len())
        .ok_or(anyhow!("invalid first line"))?;
    let mut coordinates: Vec<u8> = Vec::with_capacity((size * size) as usize);
    for (y, line) in iter {
        for (x, b) in line.bytes().enumerate() {
            match b {
                b'S' => {
                    start.get_or_insert(Coordinate(x as i16, y as i16));
                    coordinates.push(b'a');
                }
                b'E' => {
                    end.get_or_insert(Coordinate(x as i16, y as i16));
                    coordinates.push(b'z');
                }
                b @ b'a'..=b'z' => coordinates.push(b),
                _ => bail!("invalid char input {b}"),
            }
        }
    }

    //    for i in 0..(coordinates.len() / size) {
    //        println!("{:?}", &coordinates[i * size..(i + 1) * size])
    //    }

    Ok(Field {
        height_map: coordinates,
        size,
        start: start.ok_or(anyhow!("no start point"))?,
        end: end.ok_or(anyhow!("no end point"))?,
    })
}

fn algo1_part1(input: &'static str) -> Result<usize> {
    let field = parse_input(input)?;
    a_star(&field, field.start)
}

fn algo1_part2(input: &'static str) -> Result<usize> {
    let field = parse_input(input)?;
    let min_dist = field
        .height_map
        .iter()
        .enumerate()
        .filter(|(_, &b)| b == b'a')
        .filter_map(|(i, _)| {
            let start = Coordinate((i % field.size) as i16, (i / field.size) as i16);
            a_star(&field, start).ok()
        })
        .min()
        .ok_or(anyhow!("empty map?"))?;
    Ok(min_dist)
}

fn algo2_part2(input: &'static str) -> Result<usize> {
    let field = parse_input(input)?;
    dijkstra(field)
}

fn a_star(field: &Field, start: Coordinate) -> Result<usize> {
    fn heuristic(field: &Field, c: Coordinate) -> usize {
        ((field.end.0 - c.0).abs() + (field.end.1 - c.1).abs()) as usize
    }

    fn reconstruct(
        came_from: HashMap<Coordinate, Coordinate>,
        current: Coordinate,
    ) -> Vec<Coordinate> {
        let mut v = vec![];
        let mut last = current;
        while let Some(&next) = came_from.get(&last) {
            v.push(next);
            last = next;
        }
        v.reverse();
        v
    }

    let mut open_set = HashSet::<Coordinate>::new();
    let mut came_from = HashMap::<Coordinate, Coordinate>::new();
    let mut g_score = HashMap::<Coordinate, usize>::new();
    let mut f_score = HashMap::<Coordinate, usize>::new();

    open_set.insert(start);
    g_score.insert(start, 0);
    f_score.insert(start, heuristic(&field, start));

    while !open_set.is_empty() {
        let Some(&current) = open_set.iter().min_by_key(|&c| *f_score.get(c).unwrap_or(&usize::MAX)) else {bail!("empty set ???")};
        if current == field.end {
            return Ok(reconstruct(came_from, current).len());
        }
        open_set.remove(&current);

        let t_score = g_score[&current] + 1;
        for neigbor in field.get_reachable_neigbors(current, false) {
            if g_score.get(&neigbor).map_or(true, |n| t_score < *n) {
                came_from.insert(neigbor, current);
                g_score.insert(neigbor, t_score);
                f_score.insert(neigbor, t_score + heuristic(&field, neigbor));
                open_set.insert(neigbor);
            }
        }
    }
    bail!(
        "failed to find a path from {:?} to {:?}",
        field.start,
        field.end
    )
}

fn dijkstra(field: Field) -> Result<usize> {
    let mut open_set = HashSet::<Coordinate>::new();
    let mut came_from = HashMap::<Coordinate, Coordinate>::new();
    let mut g_score = HashMap::<Coordinate, usize>::new();

    for i in 0..field.height_map.len() {
        open_set.insert(Coordinate((i % field.size) as i16, (i / field.size) as i16));
    }
    g_score.insert(field.end, 0);

    while !open_set.is_empty() {
        let Some(&current) = open_set.iter().min_by_key(|&c| *g_score.get(c).unwrap_or(&usize::MAX)) else {bail!("empty set ???")};
        open_set.remove(&current);

        let Some(t_score) = g_score.get(&current).map(|t| t + 1) else {continue;};
        for neigbor in field.get_reachable_neigbors(current, true) {
            if !open_set.contains(&neigbor) {
                continue;
            }
            if g_score.get(&neigbor).map_or(true, |n| t_score < *n) {
                came_from.insert(neigbor, current);
                g_score.insert(neigbor, t_score);
            }
        }
    }
    let min_dist = g_score
        .into_iter()
        .filter(|(c, _)| field.get_height(*c) == b'a')
        .min_by_key(|a| a.1)
        .ok_or(anyhow!("empty map?"))?
        .1;
    Ok(min_dist)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_solution1_part2(b: &mut Bencher) {
        b.iter(|| algo1_part2(INPUT));
    }

    #[bench]
    fn bench_solution2_part2(b: &mut Bencher) {
        b.iter(|| algo2_part2(INPUT));
    }
}
