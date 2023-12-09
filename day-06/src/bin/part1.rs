use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, multispace0},
    combinator::map_res,
    multi::{many1, separated_list},
    IResult, sequence::tuple,
};

fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> u32 {
    let games = parse_input(input).unwrap();
    let mut total_wins: Vec<u32> = Vec::new();
    for game in games.1 {
        let mut successful_combos = 0;
        for speed in 1..game.0 {
            let travel_time = game.0 - speed;
            let distance_travelled = travel_time * speed;
            if distance_travelled > game.1 {
                successful_combos = successful_combos + 1;
            }
        }
        total_wins.push(dbg!(successful_combos));
    }
    total_wins.iter().fold(1, |sum, win| sum * win)
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, times) = tuple((
        multispace0,
        tag("Time:"),
        multispace0,
        separated_list(multispace0, parse_number),
    ))(input)
    .map(|(rest, (_, _, _, times))| (rest, times))?;
    let (input, _) = many1(newline)(input)?;
    let (input, distances) = tuple((
        multispace0,
        tag("Distance:"),
        multispace0,
        separated_list(multispace0, parse_number),
    ))(input)
    .map(|(rest, (_, _, _, distances))| (rest, distances))?;
    Ok((input, times.into_iter().zip(distances).collect::<Vec<_>>()))
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use crate::{part1, parse_input};

    #[test]
    fn parse_correctly() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        assert_eq!(parse_input(input).unwrap().1, vec![(7, 9), (15, 40), (30, 200)])
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(part1(input), 288)
    }
}
