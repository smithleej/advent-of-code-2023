fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;

    for line in input.lines() {
        let first_index = line.find(char::is_numeric).unwrap();
        let first = line.chars().nth(first_index).unwrap();
        let second_index = line.rfind(char::is_numeric).unwrap();
        let second = line.chars().nth(second_index).unwrap();
        let mut combined = String::from(first);
        combined.push(second);
        match combined.parse::<u32>() {
            Ok(num) => {
                total = total + num;
            },
            _ => panic!("Couldn't parse String into number")
        }
    };

    total
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;

    #[test]
    fn it_works() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "};
        
        assert_eq!(part1(input), 142)
    }
}
