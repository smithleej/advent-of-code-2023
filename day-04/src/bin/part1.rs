use nom::{
    character::complete::{char, digit1, multispace1, space1},
    combinator::map_res,
    multi::separated_list,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    println!("Answer {}", part1(input));
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_card(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (input, _) = tuple((
        char('C'),
        char('a'),
        char('r'),
        char('d'),
        space1,
        digit1,
        char(':'),
        space1
    ))(input)?;

    let (input, first_list) = separated_list(space1, parse_number)(input)?;

    let (input, _) = multispace1(input)?;

    let (input, _) = char('|')(input)?;

    let (input, _) = multispace1(input)?;

    let (input, second_list) = separated_list(space1, parse_number)(input)?;

    Ok((input, (first_list, second_list)))
}

fn part1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let (_, (winners, guesses)) = parse_card(line).unwrap();
        let correct_guesses: Vec<&u32> = guesses.iter().filter(|&guess| winners.contains(guess)).collect();
        total = total + correct_guesses.iter().fold(0, |sum, _| {
            if sum == 0 {
                1
            } else {
                sum * 2
            }
        })
    };
    total
}

#[cfg(test)]

mod tests {

    use crate::{parse_card, part1};

    use indoc::indoc;

    #[test]

    fn test_parsing() {
        let res = parse_card("Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53").unwrap();

        assert_eq!(
            res.1,
            (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
        )
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
    Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3: 1 21 53 59 44 | 69 82 63 72 16 21 14 1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58 5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};
        assert_eq!(part1(input), 13)
    }
}
