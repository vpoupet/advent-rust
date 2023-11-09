use crate::utils;

struct Map {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(filename: &str) -> Self {
        let input = utils::read_input(filename).unwrap();
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            grid.push(row);
        }
        Map {
            height: grid.len(),
            width: grid[0].len(),
            grid,
        }
    }

    fn get(&self, i: usize, j: usize) -> bool {
        self.grid[i][j % self.width]
    }

    fn check_slope(&self, di: usize, dj: usize) -> i32 {
        let mut total = 0;
        let (mut i, mut j) = (0, 0);

        while i < self.height {
            if self.get(i, j) {
                total += 1;
            }
            i += di;
            j += dj;
        }

        total
    }
}

pub fn solve1() -> i32 {
    let map = Map::new("src/year2020/day03/input.txt");
    
    map.check_slope(1, 3)
}

pub fn solve2() -> i64 {
    let map = Map::new("src/year2020/day03/input.txt");
    let mut total = 1;
    for (di, dj) in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        total *= map.check_slope(di, dj) as i64;
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
        assert_eq!(solution, 198);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 5140884672);
    }
}
