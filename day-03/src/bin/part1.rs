fn main() {
    let input = include_str!("./input.txt");
    dbg!(part1(input));
}

fn check_touches_special(grid: &Vec<Vec<char>>, current: (usize, usize)) -> bool {
    let y_end = grid.len() - 1;
    let x_end = grid[0].len() - 1;
    let s_cords = get_surrounding_coordinates(current.0 as isize, current.1 as isize, y_end as isize, x_end as isize);
    let mut any = false;

    for (x, y) in s_cords {
        let cell = grid[y][x];
        if !cell.is_alphanumeric() && cell != '.' {
            any = true;
        };
    }

    any
}

fn get_surrounding_coordinates(x: isize, y: isize, y_end: isize, x_end: isize) -> Vec<(usize, usize)> {
    let mut surrounding_coordinates = Vec::new();

    for i in (x - 1).max(0)..=(x + 1).min(x_end) {
        for j in (y - 1).max(0)..=(y + 1).min(y_end) {
            // Exclude the center point (x, y)
            if i != x || j != y {
                surrounding_coordinates.push((i as usize, j as usize));
            }
        }
    }

    surrounding_coordinates
}

fn part1(input: &str) -> u32 {
    // Parse the string into a 2D vector
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Print the parsed grid
    for row in &grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    // Initialize counters
    // let mut x_skip_count = 0;
    let mut numbers: Vec<u32> = Vec::new();
    let mut current_number: Vec<char> = Vec::new();
    let mut current_touches: bool = false;

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {

            if cell.is_numeric() {
                current_number.push(cell);
                if check_touches_special(&grid, (x, y)) {
                    current_touches = true;
                }
            };

            if !next_cell_is_numeric(&grid, (y, x)) && current_touches {
                let cn: String = current_number.iter().collect();
                numbers.push(cn.parse::<u32>().unwrap());
                current_number = Vec::new();
                current_touches = false;
            } else if !next_cell_is_numeric(&grid, (y, x)) {
                current_number = Vec::new();
                current_touches = false;
            }
        }
    };

    dbg!(numbers).iter().sum()
}

fn next_cell_is_numeric(grid: &Vec<Vec<char>>, current: (usize, usize)) -> bool {
    if current.1 + 1 < grid[0].len() {
        grid[current.0][current.1+1].is_numeric()
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::part1;
    use indoc::indoc;
    use crate::get_surrounding_coordinates;

    #[test]
    fn get_surrounding_coordinates_works() {
        let given_coordinate = (1, 2);
        let result = get_surrounding_coordinates(given_coordinate.0, given_coordinate.1, 2, 2);
        println!("s cords res: {:?}", result);
    }

    #[test]
    fn it_works() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "};

        assert_eq!(part1(input), 4361)
    }
}
