use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

struct RGB {
    red: Cube,
    green: Cube,
    blue: Cube,
}

struct Cube {
    colour: CubeColour,
    value: u32,
}

enum CubeColour {
    Red,
    Green,
    Blue,
}

fn find_highest_rgb(parsed_input: &mut impl Iterator<Item = Cube>) -> RGB {
    let init: RGB = RGB {
        red: Cube {
            colour: CubeColour::Red,
            value: 0,
        },
        green: Cube {
            colour: CubeColour::Green,
            value: 0,
        },
        blue: Cube {
            colour: CubeColour::Blue,
            value: 0,
        },
    };
    parsed_input.fold(init, |sum, cube| match cube.colour {
        CubeColour::Red => {
            if cube.value > sum.red.value {
                RGB {
                    red: cube,
                    green: sum.green,
                    blue: sum.blue,
                }
            } else {
                sum
            }
        }
        CubeColour::Green => {
            if cube.value > sum.green.value {
                RGB {
                    red: sum.red,
                    green: cube,
                    blue: sum.blue,
                }
            } else {
                sum
            }
        }
        CubeColour::Blue => {
            if cube.value > sum.blue.value {
                RGB {
                    red: sum.red,
                    green: sum.green,
                    blue: cube,
                }
            } else {
                sum
            }
        }
    })
}

fn part1(input: &str) -> u32 {
    lazy_static! {
        static ref CUBES_REGEX: Regex = Regex::new(r"(\d+) (red|blue|green)").unwrap();
        static ref GAME_REGEX: Regex = Regex::new(r"Game (\d+)").unwrap();
    }

    input.lines().fold(0, |sum, line| {
        let mut input_parsed =
            CUBES_REGEX
                .captures_iter(line)
                .map(|mat| match mat.get(2).unwrap().as_str() {
                    "red" => Cube {
                        colour: CubeColour::Red,
                        value: mat.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    },
                    "green" => Cube {
                        colour: CubeColour::Green,
                        value: mat.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    },
                    "blue" => Cube {
                        colour: CubeColour::Blue,
                        value: mat.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    },
                    _ => panic!("Unknown colour"),
                });

        let rgb = find_highest_rgb(&mut input_parsed);
        sum + dbg!(rgb.red.value * rgb.green.value * rgb.blue.value)
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

        assert_eq!(part1(input), 2286)
    }
}
