use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, newline, space1},
    combinator::map_res,
    multi::{many1, separated_list},
    sequence::{terminated, tuple},
    IResult,
};
use itertools::Itertools;

#[derive(PartialEq, Debug)]
struct LineMapping {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

#[derive(PartialEq, Debug)]
struct Inputs {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<LineMapping>,
    soil_to_fertilizer: Vec<LineMapping>,
    fertilizer_to_water: Vec<LineMapping>,
    water_to_light: Vec<LineMapping>,
    light_to_temperature: Vec<LineMapping>,
    temperature_to_humidity: Vec<LineMapping>,
    humidity_to_location: Vec<LineMapping>,
}

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn parse_input(input: &str) -> IResult<&str, Inputs> {
    let (input, _) = tuple((tag("seeds"), char(':'), space1))(input)?;

    let (input, seeds) = separated_list(space1, parse_number)(input)?;

    let (input, _) = many1(newline)(input)?;

    let (input, _) = tuple((tag("seed-to-soil map"), char(':'), line_ending))(input)?;
    let (input, seeds_to_soil) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("soil-to-fertilizer map"), char(':'), line_ending))(input)?;
    let (input, soil_to_fertilizer) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("fertilizer-to-water map"), char(':'), line_ending))(input)?;
    let (input, fertilizer_to_water) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("water-to-light map"), char(':'), line_ending))(input)?;
    let (input, water_to_light) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("light-to-temperature map"), char(':'), line_ending))(input)?;
    let (input, light_to_temperature) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("temperature-to-humidity map"), char(':'), line_ending))(input)?;
    let (input, temperature_to_humidity) = parse_line_mappings(input)?;

    let (input, _) = tuple((tag("humidity-to-location map"), char(':'), line_ending))(input)?;
    let (input, humidity_to_location) = parse_line_mappings(input)?;

    Ok((
        input,
        Inputs {
            seeds,
            seeds_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn parse_line_mappings(input: &str) -> Result<(&str, Vec<LineMapping>), nom::Err<(&str, nom::error::ErrorKind)>> {
    let (input, line_mappings) = many1(terminated(
        tuple((parse_number, space1, parse_number, space1, parse_number)),
        many1(newline),
    ))(input)
    .map(|(remaining, vec)| {
        (
            remaining,
            vec.iter()
                .map(
                    |(destination_start, _, source_start, _, range_length)| LineMapping {
                        destination_start: *destination_start,
                        source_start: *source_start,
                        range_length: *range_length,
                    },
                )
                .collect::<Vec<_>>(),
        )
    })?;
    Ok((input, line_mappings))
}

fn part2(input: &str) -> u64 {
    let (_, inputs) = parse_input(input).unwrap();
    println!("Parsed input {:?}", inputs);
    let all_seeds = expand_seeds(inputs.seeds);
    println!("Expanded seeds count {:?}", all_seeds.len());
    let lowest: u64 = all_seeds.iter().map(|seed| {
        let soil_number = get_next_number(&inputs.seeds_to_soil, *seed);
        let fert_number = get_next_number(&inputs.soil_to_fertilizer, soil_number);
        let water_number = get_next_number(&inputs.fertilizer_to_water, fert_number);
        let light_number = get_next_number(&inputs.water_to_light, water_number);
        let temp_number = get_next_number(&inputs.light_to_temperature, light_number);
        let hum_number = get_next_number(&inputs.temperature_to_humidity, temp_number);
        let loc_number = get_next_number(&inputs.humidity_to_location, hum_number);
        loc_number
    }).min().unwrap();
    lowest
}

fn expand_seeds(seeds: Vec<u64>) -> Vec<u64> {
    let mut expanded: Vec<u64> = Vec::new();
    let pairs: Vec<(&u64, &u64)> = seeds.iter().tuples().collect_vec();
    for pair in pairs {
        for count in 0..*pair.1 {
            expanded.push(pair.0 + count);
        }
    }
    expanded
}

fn get_next_number(mappings: &Vec<LineMapping>, seed: u64) -> u64 {
    let number = mappings.iter().find(|mapping| {
        if seed >= mapping.source_start && seed < mapping.source_start + mapping.range_length {
            true
        } else {
            false
        }
    }).map_or(seed,|mapping| mapping.destination_start + (seed - mapping.source_start));
    number
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part2, Inputs, LineMapping};
    use indoc::indoc;

    #[test]
    fn test_parsing() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let expected: Inputs = Inputs {
            seeds: vec![79, 14, 55, 13],
            seeds_to_soil: vec![
                LineMapping {
                    destination_start: 50,
                    source_start: 98,
                    range_length: 2,
                },
                LineMapping {
                    destination_start: 52,
                    source_start: 50,
                    range_length: 48,
                },
            ],
            soil_to_fertilizer: vec![
                LineMapping {
                    destination_start: 0,
                    source_start: 15,
                    range_length: 37,
                },
                LineMapping {
                    destination_start: 37,
                    source_start: 52,
                    range_length: 2,
                },
                LineMapping {
                    destination_start: 39,
                    source_start: 0,
                    range_length: 15,
                },
            ],
            fertilizer_to_water: vec![
                LineMapping {
                    destination_start: 49,
                    source_start: 53,
                    range_length: 8,
                },
                LineMapping {
                    destination_start: 0,
                    source_start: 11,
                    range_length: 42,
                },
                LineMapping {
                    destination_start: 42,
                    source_start: 0,
                    range_length: 7,
                },
                LineMapping {
                    destination_start: 57,
                    source_start: 7,
                    range_length: 4,
                },
            ],
            water_to_light: vec![
                LineMapping {
                    destination_start: 88,
                    source_start: 18,
                    range_length: 7,
                },
                LineMapping {
                    destination_start: 18,
                    source_start: 25,
                    range_length: 70,
                },
            ],
            light_to_temperature: vec![
                LineMapping {
                    destination_start: 45,
                    source_start: 77,
                    range_length: 23,
                },
                LineMapping {
                    destination_start: 81,
                    source_start: 45,
                    range_length: 19,
                },
                LineMapping {
                    destination_start: 68,
                    source_start: 64,
                    range_length: 13,
                },
            ],
            temperature_to_humidity: vec![
                LineMapping {
                    destination_start: 0,
                    source_start: 69,
                    range_length: 1,
                },
                LineMapping {
                    destination_start: 1,
                    source_start: 0,
                    range_length: 69,
                },
            ],
            humidity_to_location: vec![
                LineMapping {
                    destination_start: 60,
                    source_start: 56,
                    range_length: 37,
                },
                LineMapping {
                    destination_start: 56,
                    source_start: 93,
                    range_length: 4,
                },
            ],
        };

        assert_eq!(expected, parse_input(input).unwrap().1)
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        assert_eq!(part2(input), 46)
    }
}
