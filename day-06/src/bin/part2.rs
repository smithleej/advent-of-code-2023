fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> u32 {
    let game = parse_input(input);
    let mut successful_combos = 0;
    for speed in 1..game.0 {
        let travel_time = game.0 - speed;
        let distance_travelled = travel_time * speed;
        if distance_travelled > game.1 {
            successful_combos = successful_combos + 1;
        }
    }
    successful_combos
}

fn parse_input(input: &str) -> (u64, u64) {
    let mut split = input.split('\n');
    match (split.next(), split.next()) {
        (Some(time), Some(distance)) => (
            time.trim_start_matches("Time:")
                .replace(" ", "")
                .parse::<u64>()
                .unwrap(),
            distance
                .trim_start_matches("Distance:")
                .replace(" ", "")
                .parse::<u64>()
                .unwrap(),
        ),
        (_, _) => {
            panic!("Input couldn't be split into two lines")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part1};
    use indoc::indoc;

    #[test]
    fn parse_correctly() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};
        assert_eq!(parse_input(input), (71530, 940200))
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
            Time:      7  15   30
            Distance:  9  40  200
        "};

        assert_eq!(part1(input), 71503)
    }
}
