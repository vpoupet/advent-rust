use crate::utils;

fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let input = utils::read_input(filename).unwrap();
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }
    result
}

fn roll_north(rocks: &mut Vec<Vec<char>>) {
    for i in 0..rocks.len() {
        for j in 0..rocks[0].len() {
            if rocks[i][j] == 'O' {
                let mut i2 = i;
                while i2 > 0 && rocks[i2 - 1][j] == '.' {
                    i2 -= 1;
                }
                if i2 != i {
                    rocks[i][j] = '.';
                    rocks[i2][j] = 'O';
                }
            }
        }
    }
}

fn roll_south(rocks: &mut Vec<Vec<char>>) {
    for i in (0..rocks.len()).rev() {
        for j in 0..rocks[0].len() {
            if rocks[i][j] == 'O' {
                let mut i2 = i;
                while i2 + 1 < rocks.len() && rocks[i2 + 1][j] == '.' {
                    i2 += 1;
                }
                if i2 != i {
                    rocks[i][j] = '.';
                    rocks[i2][j] = 'O';
                }
            }
        }
    }
}

fn roll_west(rocks: &mut Vec<Vec<char>>) {
    for j in 0..rocks[0].len() {
        for i in (0..rocks.len()).rev() {
            if rocks[i][j] == 'O' {
                let mut j2 = j;
                while j2 > 0 && rocks[i][j2 - 1] == '.' {
                    j2 -= 1;
                }
                if j2 != j {
                    rocks[i][j] = '.';
                    rocks[i][j2] = 'O';
                }
            }
        }
    }
}

fn roll_east(rocks: &mut Vec<Vec<char>>) {
    for j in (0..rocks[0].len()).rev() {
        for i in (0..rocks.len()).rev() {
            if rocks[i][j] == 'O' {
                let mut j2 = j;
                while j2 + 1 < rocks[0].len() && rocks[i][j2 + 1] == '.' {
                    j2 += 1;
                }
                if j2 != j {
                    rocks[i][j] = '.';
                    rocks[i][j2] = 'O';
                }
            }
        }
    }
}

fn spin_cycle(rocks: &mut Vec<Vec<char>>, nb_loops: usize) {
    for _ in 0..nb_loops {
        roll_north(rocks);
        roll_west(rocks);
        roll_south(rocks);
        roll_east(rocks);
    }
}

fn get_total_load(rocks: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let n = rocks.len();
    for i in 0..rocks.len() {
        for j in 0..rocks[0].len() {
            if rocks[i][j] == 'O' {
                result += n - i;
            }
        }
    }
    result
}

pub fn solve1() -> usize {
    let mut rocks = parse_input("src/year2023/day14/input.txt");
    roll_north(&mut rocks);
    get_total_load(&rocks)
}

pub fn solve2() -> usize {
    let mut rocks = parse_input("src/year2023/day14/input.txt");
    let mut rocks2 = rocks.clone();
    let mut counter = 0;

    loop {
        spin_cycle(&mut rocks, 1);
        spin_cycle(&mut rocks2, 2);
        counter += 1;
        if rocks == rocks2 {
            break;
        }
    }

    spin_cycle(&mut rocks, 1000000000 % counter);
    get_total_load(&rocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 113078);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 94255);
    }
}
