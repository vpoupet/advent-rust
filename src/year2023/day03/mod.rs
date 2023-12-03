use crate::utils;

#[derive(Debug)]
struct Number {
    value: i32,
    row: usize,
    column: usize,
    length: usize,
}
impl Number {
    fn new(value: i32, row: usize, column: usize, length: usize) -> Number {
        Number {
            value,
            row,
            column,
            length,
        }
    }

    fn add_digit(&mut self, digit: i32) {
        self.value = self.value * 10 + digit;
        self.length += 1;
    }

    fn is_adjacent_to(&self, row: i32, column: i32) -> bool {
        let r = self.row as i32;
        let c = self.column as i32;
        if -1 <= row - r && row - r <= 1 && -1 <= column - c && column - c <= self.length as i32 {
            return true;
        }
        false
    }
}

struct Schematic {
    grid: Vec<Vec<char>>,
    numbers: Vec<Number>,
}
impl Schematic {
    fn from_file(filename: &str) -> Schematic {
        let input = utils::read_input(filename).unwrap();
        // make the grid
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }

        // find the numbers
        let mut numbers = Vec::new();
        let mut current_number = None;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j].is_digit(10) {
                    let d = grid[i][j].to_digit(10).unwrap() as i32;
                    if current_number.is_none() {
                        current_number = Some(Number::new(d, i, j, 1));
                    } else {
                        current_number.as_mut().unwrap().add_digit(d);
                    }
                } else {
                    if current_number.is_some() {
                        numbers.push(current_number.unwrap());
                        current_number = None;
                    }
                }
            }
            if current_number.is_some() {
                numbers.push(current_number.unwrap());
                current_number = None;
            }
        }

        // return object
        Schematic { grid, numbers }
    }

    fn get_symbol(&self, row: i32, column: i32) -> Option<char> {
        if row < 0 || row >= self.grid.len() as i32 {
            return None;
        }
        let row = row as usize;
        if column < 0 || column >= self.grid[row].len() as i32 {
            return None;
        }
        let column = column as usize;
        if self.grid[row][column] == '.' || self.grid[row][column].is_digit(10) {
            return None;
        }
        Some(self.grid[row][column])
    }

    fn is_engine_part(&self, number: &Number) -> bool {
        let row = number.row as i32;
        let column = number.column as i32;

        for i in -1..=1 {
            if self.get_symbol(row + i, column - 1).is_some() {
                return true;
            }
            if self
                .get_symbol(row + i, column + number.length as i32)
                .is_some()
            {
                return true;
            }
        }

        for j in -1..=number.length as i32 {
            if self.get_symbol(row - 1, column + j).is_some() {
                return true;
            }
            if self.get_symbol(row + 1, column + j).is_some() {
                return true;
            }
        }
        false
    }
}

pub fn solve1() -> i32 {
    let schematic = Schematic::from_file("src/year2023/day03/input.txt");
    let mut total = 0;
    for number in &schematic.numbers {
        if schematic.is_engine_part(&number) {
            total += number.value;
        }
    }
    total
}

pub fn solve2() -> i32 {
    let schematic = Schematic::from_file("src/year2023/day03/input.txt");
    let mut total = 0;

    for i in 0..schematic.grid.len() {
        for j in 0..schematic.grid[0].len() {
            if schematic.grid[i][j] == '*' {
                let mut adjacent_numbers = Vec::new();
                for number in &schematic.numbers {
                    if number.is_adjacent_to(i as i32, j as i32) {
                        adjacent_numbers.push(number);
                    }
                }
                if adjacent_numbers.len() == 2 {
                    total += adjacent_numbers[0].value * adjacent_numbers[1].value;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 535351);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 87287096);
    }
}
