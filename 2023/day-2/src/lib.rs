const INPUT: &str = include_str!("../input_part_1");
const EXAMPLE_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

struct Game {
    id: usize,
    red: usize,
    blue: usize,
    green: usize,
}

fn parse_input(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().map(parse_line)
}

fn parse_line(line: &str) -> Game {
    let line = line.strip_prefix("Game ").unwrap();
    let (id, rem) = line.split_once(": ").unwrap();
    let id = id.parse::<usize>().unwrap();
    let mut game = Game {
        id,
        red: 0,
        blue: 0,
        green: 0,
    };
    let subsets = rem.split(";");
    for subset in subsets {
        let cubes = subset.split(", ");
        for cube in cubes {
            let (num, color) = cube.trim().split_once(" ").unwrap();
            let num = num.parse::<usize>().unwrap();
            match color {
                "red" => game.red = game.red.max(num),
                "blue" => game.blue = game.blue.max(num),
                "green" => game.green = game.green.max(num),
                _ => unreachable!(),
            }
        }
    }
    game
}

mod part_1 {

    fn solution(input: &str) -> usize {
        super::parse_input(input)
            .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
            .map(|g| g.id)
            .sum()
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 8);
    }

    #[test]
    fn test_part_1() {
        eprintln!("Solution Day 2 Part 1: [{}]", solution(super::INPUT));
    }
}

mod part_2 {
    fn solution(input: &str) -> usize {
        super::parse_input(input)
            .map(|g| g.red * g.green * g.blue)
            .sum()
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(solution(super::EXAMPLE_INPUT), 2286);
    }

    #[test]
    fn test_part_2() {
        eprintln!("Solution Day 2 Part 2: [{}]", solution(super::INPUT));
    }
}
