use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    println!("{}", part2(input));
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<HandBid> = parse_input(input);
    println!("Parsed hands: {:?}", hands);
    hands.sort();
    println!("Sorted hands: {:?}", hands);
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i + 1);
    }
    total
}

fn parse_input(input: &str) -> Vec<HandBid> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let (hand, bid) = (split.next().unwrap(), split.next().unwrap());
            let hand_enums = hand
                .chars()
                .map(|char| match char {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => Card::J,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    _ => panic!("Unexpected card"),
                })
                .collect();
            HandBid {
                hand: hand_enums,
                bid: bid.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug)]
struct HandBid {
    hand: Vec<Card>,
    bid: usize,
}

fn count_duplicates(vec: &Vec<Card>) -> HashMap<&Card, usize> {
    let mut count_map: HashMap<&Card, usize> = HashMap::new();

    for item in vec {
        let counter = count_map.entry(item).or_insert(0);
        *counter += 1;
    }

    count_map.retain(|card, count| (count > &mut 1) || **card == Card::J);

    count_map
}

impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut my_dupes = count_duplicates(dbg!(&self.hand));
        dbg!(mutate_jokers(&mut my_dupes));
        let mut other_dupes = count_duplicates(dbg!(&other.hand));
        dbg!(mutate_jokers(&mut other_dupes));
        let my_max = dbg!(my_dupes.values().max().unwrap_or(&0));
        let other_max = dbg!(other_dupes.values().max().unwrap_or(&0));
        if *my_max == 3 && *other_max == 3 {
            // check for full house
            let my_dupe_counts: Vec<&usize> = my_dupes.values().collect();
            let other_dupe_counts: Vec<&usize> = other_dupes.values().collect();
            if my_dupe_counts.contains(&&2) && other_dupe_counts.contains(&&2) {
                // both full house
                println!("Both full house");
                self.fun_name(other)
            } else if my_dupe_counts.contains(&&2) {
                println!(
                    "Full house: Self greater than other: \n{:?},\n{:?}",
                    self, other
                );
                println!("Counts: \n{:?},\n{:?}", my_dupes, other_dupes);
                Ordering::Greater
            } else if other_dupe_counts.contains(&&2) {
                println!(
                    "Full house: Self less than other: \n{:?},\n{:?}",
                    self, other
                );
                Ordering::Less
            } else {
                println!("Full house: Else...");
                self.fun_name(other)
            }
        } else if *my_max == 2 && *other_max == 2 {
            // check for two pair
            let my_pairs_count = my_dupes
                .values()
                .fold(0, |sum, v| if v == &2 { sum + 1 } else { sum });

            let other_pairs_count =
                other_dupes
                    .values()
                    .fold(0, |sum, v| if v == &2 { sum + 1 } else { sum });

            if my_pairs_count == 2 && other_pairs_count == 2 {
                println!("Both two pairs");
                self.fun_name(other)
            } else if my_pairs_count == 2 {
                println!(
                    "Two pair: Self greater than other: \n{:?},\n{:?}",
                    self, other
                );
                println!("Counts: \n{:?},\n{:?}", my_dupes, other_dupes);
                Ordering::Greater
            } else if other_pairs_count == 2 {
                println!("Two pair: Self less than other: \n{:?},\n{:?}", self, other);
                Ordering::Less
            } else {
                println!("Two pair: Else...");
                self.fun_name(other)
            }
        } else if my_max > other_max {
            println!("Self greater than other: \n{:?},\n{:?}", self, other);
            println!("Counts: \n{:?},\n{:?}", my_dupes, other_dupes);
            Ordering::Greater
        } else if my_max < other_max {
            println!("Self less than other: \n{:?},\n{:?}", self, other);
            println!("Counts: \n{:?},\n{:?}", my_dupes, other_dupes);
            Ordering::Less
        } else {
            println!("Else...");
            self.fun_name(other)
        }
    }
}

// if 5 jokers, do nothing
// if 4 jokers, add 1 to joker count
// if 3 jokers
    // if pair, make 5 of kind
    // if high card, make 4 of kind
// if 2 jokers
    // if 3 of kind, make 5 of kind
    // if 2 pair, make 4 of kind
    // if high card, make 3 of kind
// if 1 joker
    // if 4 of kind, make 5 of kind
    // if 3 of kind, make 4 of kind
    // if 2 pairs, make full house
    // if pair, make 3 of kind
    // if high card, make pair
fn mutate_jokers(dupes: &mut HashMap<&Card, usize>) -> () {
    let joker_count = dupes.get(&Card::J).unwrap_or(&0);
    match joker_count {
        4 => {
            dupes.insert(&Card::J, 5);
        }
        3 => {
            if dupes.values().collect::<Vec<_>>().contains(&&2) {
                dupes.insert(&Card::J, 5);
            } else {
                dupes.insert(&Card::J, 4);
            }
        }
        2 => {
            if dupes.values().collect::<Vec<_>>().contains(&&3) {
                dupes.insert(&Card::J, 5);
            } else {
                let pairs_count = dupes
                .values()
                .fold(0, |sum, v| if v == &2 { sum + 1 } else { sum });

                if pairs_count == 2 {
                    dupes.insert(&Card::J, 4);
                } else {
                    dupes.insert(&Card::J, 3);
                }
            }
        }
        1 => {
            if dupes.values().collect::<Vec<_>>().contains(&&4) {
                dupes.insert(&Card::J, 5);
            } else if dupes.values().collect::<Vec<_>>().contains(&&3) {
                dupes.insert(&Card::J, 4);
            } else {
                let pairs_count = dupes
                .values()
                .fold(0, |sum, v| if v == &2 { sum + 1 } else { sum });

                if pairs_count == 2 {
                    let pair = dupes.iter().find(|item| item.1 == &2).unwrap();
                    dupes.insert(pair.0, 3);
                } else if pairs_count == 1 {
                    dupes.insert(&Card::J, 3);
                } else {
                    dupes.insert(&Card::J, 2);
                }
            }
        }
        x => println!("Nothing to do with {}", x),
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HandBid {
    fn fun_name(&self, other: &HandBid) -> Ordering {
        let mut ordering = Ordering::Equal;
        for i in 0..self.hand.len() {
            match self.hand.get(i).unwrap().cmp(other.hand.get(i).unwrap()) {
                Ordering::Equal => ordering = Ordering::Equal,
                to_return => {
                    ordering = to_return;
                    break;
                }
            }
        }
        println!(
            "Ordering on high card: {:?} for \n{:?}\n{:?}",
            ordering, self, other
        );
        ordering
    }
}

#[derive(PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part2, Card, HandBid};
    use indoc::indoc;

    #[test]
    fn parse_correctly() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
        "};
        assert_eq!(
            parse_input(input),
            vec![
                HandBid {
                    hand: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::K],
                    bid: 765
                },
                HandBid {
                    hand: vec![Card::Ten, Card::Five, Card::Five, Card::J, Card::Five],
                    bid: 684
                },
                HandBid {
                    hand: vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
                    bid: 28
                }
            ]
        )
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "};

        assert_eq!(part2(input), 5905)
    }

    #[test]
    fn it_works2() {
        let input = indoc! {"
            2345A 1
            Q2KJJ 13
            Q2Q2Q 19
            T3T3J 17
            T3Q33 11
            2345J 3
            J345A 2
            32T3K 5
            T55J5 29
            KK677 7
            KTJJT 34
            QQQJA 31
            JJJJJ 37
            JAAAA 43
            AAAAJ 59
            AAAAA 61
            2AAAA 23
            2JJJJ 53
            JJJJ2 41
        "};

        assert_eq!(part2(input), 6839)
    }

    #[test]
    fn it_works3() {
        let input = indoc! {"
            2345A 1
            Q2KJJ 13
            Q2Q2Q 19
            T3T3J 17
            T3Q33 11
            2345J 3
            J345A 2
            32T3K 5
            T55J5 29
            KK677 7
            KTJJT 34
            QQQJA 31
            JJJJJ 37
            JAAAA 43
            AAAAJ 59
            AAAAA 61
            2AAAA 23
            2JJJJ 53
            JKQKK 21
            JJJJ2 41
        "};

        assert_eq!(part2(input), 7460)
    }
}
