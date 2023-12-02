use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

enum Cube {
    Red{value: u32},
    Green{value:u32},
    Blue{value:u32},
}

fn name_me(parsed_input: &mut impl Iterator<Item = Cube>) -> bool{
    parsed_input.all(|cube| {
        match cube {
            Cube::Red{value} => value <= 12,
            Cube::Green{value} => value <= 13,
            Cube::Blue{value} => value <= 14
        }
    })
}

fn part1(input: &str) -> u32 {
    lazy_static! {
        static ref CUBES_REGEX: Regex = Regex::new(r"(\d+) (red|blue|green)").unwrap();
        static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+)").unwrap();
    }

    input.lines().fold(0, |sum, line| {
        let game = dbg!(GAME_REGEX
            .captures(line)
            .expect("Didn't find game")
            .get(1)
            .expect("Didn't find game 2")
            .as_str())
            .parse::<u32>()
            .expect("Couldn't parse game");

        let mut input_parsed =
            CUBES_REGEX
                .captures_iter(line)
                .map(|mat| match mat.get(2).unwrap().as_str() {
                    "red" => Cube::Red{value : mat.get(1).unwrap().as_str().parse::<u32>().unwrap()},
                    "green" => Cube::Green{value: mat.get(1).unwrap().as_str().parse::<u32>().unwrap()},
                    "blue" => Cube::Blue{value: mat.get(1).unwrap().as_str().parse::<u32>().unwrap()},
                    _ => panic!("Unknown colour"),
                });

        if dbg!(name_me(&mut input_parsed)) {
            sum + game
        } else {
            sum
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    #[test]
    fn it_works() {
        let input = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        assert_eq!(part1(input), 8)
    }
}
