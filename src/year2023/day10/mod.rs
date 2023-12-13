use crate::utils;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Direction {
    di: i32,
    dj: i32,
}
impl Direction {
    fn north() -> Self {
        Self { di: -1, dj: 0 }
    }
    fn south() -> Self {
        Self { di: 1, dj: 0 }
    }
    fn east() -> Self {
        Self { di: 0, dj: 1 }
    }
    fn west() -> Self {
        Self { di: 0, dj: -1 }
    }
    fn opposite(&self) -> Self {
        Self {
            di: -self.di,
            dj: -self.dj,
        }
    }
}

fn get_directions(symbol: char) -> Vec<Direction> {
    match symbol {
        '|' => vec![Direction::north(), Direction::south()],
        '-' => vec![Direction::west(), Direction::east()],
        'L' => vec![Direction::north(), Direction::east()],
        'J' => vec![Direction::north(), Direction::west()],
        '7' => vec![Direction::south(), Direction::west()],
        'F' => vec![Direction::south(), Direction::east()],
        '.' => Vec::new(),
        'S' => Vec::new(),
        _ => panic!("Unknown symbol: {}", symbol),
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    prev: Direction,
    next: Direction,
}
impl Cell {
    fn new(prev: Direction, next: Direction) -> Self {
        Self { prev, next }
    }
}

struct Grid {
    cells: Vec<Vec<Option<Cell>>>,
    i0: usize,
    j0: usize,
}
impl Grid {
    fn get_loop_cells(&self) -> Vec<(usize, usize)> {
        let mut loop_cells = Vec::new();
        let mut i = self.i0;
        let mut j = self.j0;
        loop {
            loop_cells.push((i, j));
            let next = self.cells[i][j].unwrap().next;
            i = (i as i32 + next.di) as usize;
            j = (j as i32 + next.dj) as usize;
            if (i, j) == (self.i0, self.j0) {
                break;
            }
        }
        loop_cells
    }
}

fn make_grid() -> Grid {
    let input = utils::read_input("src/year2023/day10/input.txt").unwrap();
    let symbols_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut grid = vec![vec![None; symbols_grid[0].len()]; symbols_grid.len()];

    // find starting cell
    let (mut i0, mut j0) = (0, 0);
    'outer: for i in 0..symbols_grid.len() {
        for j in 0..symbols_grid[i].len() {
            if symbols_grid[i][j] == 'S' {
                (i0, j0) = (i, j);
                break 'outer;
            }
        }
    }

    // set directions for starting cell
    let mut starting_directions = Vec::new();
    if get_directions(symbols_grid[i0 - 1][j0]).contains(&Direction::south()) {
        starting_directions.push(Direction::north());
    }
    if get_directions(symbols_grid[i0 + 1][j0]).contains(&Direction::north()) {
        starting_directions.push(Direction::south());
    }
    if get_directions(symbols_grid[i0][j0 - 1]).contains(&Direction::east()) {
        starting_directions.push(Direction::west());
    }
    if get_directions(symbols_grid[i0][j0 + 1]).contains(&Direction::west()) {
        starting_directions.push(Direction::east());
    }

    grid[i0][j0] = Some(Cell::new(starting_directions[0], starting_directions[1]));
    let mut direction = grid[i0][j0].unwrap().next;
    let (mut i, mut j) = (
        (i0 as i32 + direction.di) as usize,
        (j0 as i32 + direction.dj) as usize,
    );
    while (i, j) != (i0, j0) {
        let directions = get_directions(symbols_grid[i][j]);
        let prev = direction.opposite();
        let next = directions.iter().filter(|d| **d != prev).next().unwrap();
        grid[i][j] = Some(Cell::new(prev, *next));
        direction = *next;
        i = (i as i32 + direction.di) as usize;
        j = (j as i32 + direction.dj) as usize;
    }

    Grid {
        cells: grid,
        i0,
        j0,
    }
}

fn paint_areas(grid: &Grid) -> Vec<Vec<i32>> {
    let mut area_map = vec![vec![0; grid.cells[0].len()]; grid.cells.len()];
    for (i, j) in grid.get_loop_cells() {
        area_map[i][j] = 1;
    }
    for (i, j) in grid.get_loop_cells() {
        let prev = grid.cells[i][j].unwrap().prev.opposite();
        paint_area(&mut area_map, i as i32 + prev.dj, j as i32 - prev.di, 2);
        paint_area(&mut area_map, i as i32 - prev.dj, j as i32 + prev.di, 3);
        let next = grid.cells[i][j].unwrap().next;
        paint_area(&mut area_map, i as i32 + next.dj, j as i32 - next.di, 2);
        paint_area(&mut area_map, i as i32 - next.dj, j as i32 + next.di, 3);
    }
    area_map
}

fn paint_area(area_map: &mut Vec<Vec<i32>>, i: i32, j: i32, area: i32) {
    let mut to_do = vec![(i as i32, j as i32)];
    while !to_do.is_empty() {
        let (i, j) = to_do.pop().unwrap();
        if i >= 0
            && i < area_map.len() as i32
            && j >= 0
            && j < area_map[0].len() as i32
            && area_map[i as usize][j as usize] == 0
        {
            area_map[i as usize][j as usize] = area;
            to_do.push((i - 1, j));
            to_do.push((i + 1, j));
            to_do.push((i, j - 1));
            to_do.push((i, j + 1));
        }
    }
}

pub fn solve1() -> usize {
    let grid = make_grid();
    grid.get_loop_cells().len() / 2
}

pub fn solve2() -> i32 {
    let grid = make_grid();
    let area_map = paint_areas(&grid);
    let mut count = vec![0, 0, 0, 0];
    for i in 0..area_map.len() {
        for j in 0..area_map[i].len() {
            count[area_map[i][j] as usize] += 1;
        }
    }

    if area_map[0][0] == 2 {
        return count[3];
    } else if area_map[0][0] == 3 {
        return count[2];
    }
    panic!("No area found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 6773);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 493);
    }
}
