fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

struct NumberString {
    name: &'static str,
    value: &'static char
}

fn is_number<'a>(numbers: &'a Vec<NumberString>, input: &'a str) -> Option<&'a NumberString> {
    numbers.iter().find(|item| input.contains(item.name))
}

fn part1(input: &str) -> u32 {
    let numbers = vec![
        NumberString{name:"one", value:&'1'}, 
        NumberString{name:"two", value:&'2'}, 
        NumberString{name:"three", value:&'3'},
        NumberString{name:"four", value:&'4'},
        NumberString{name:"five", value:&'5'},
        NumberString{name:"six", value:&'6'},
        NumberString{name:"seven", value:&'7'}, 
        NumberString{name:"eight", value:&'8'},
        NumberString{name:"nine", value:&'9'}
        ];
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut first_buffer: String = String::new();
        let mut first = 'a';
        for charr in line.chars() {
            first_buffer.push(charr);
            if dbg!(charr).is_numeric() {
                first = charr;
                break;
            } else if is_number(&numbers, dbg!(&first_buffer)).is_some() {
                first = *is_number(&numbers, &first_buffer).unwrap().value;
                break;
            }
        }
        
        let mut second_buffer: String = String::new();
        let mut second = 'b';
        for charr in line.chars().rev() {
            second_buffer.insert(0, charr);
            if charr.is_numeric() {
                second = charr;
                break;
            } else if is_number(&numbers, dbg!(&second_buffer)).is_some() {
                second = *is_number(&numbers, &second_buffer).unwrap().value;
                break;
            }
        }

        let mut combined = String::from(first);
        combined.push(second);
        match dbg!(combined).parse::<u32>() {
            Ok(num) => {
                total = total + dbg!(num);
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
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        
        assert_eq!(part1(input), 281)
    }
}
