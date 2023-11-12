use std::collections::HashSet;

use crate::utils;

trait Point
where
    Self: Sized,
    Self: std::hash::Hash,
    Self: std::cmp::Eq,
    Self: std::clone::Clone,
{
    fn get_neighbors(&self) -> Vec<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Point3D {
        Point3D { x, y, z }
    }
}

impl Point for Point3D {
    fn get_neighbors(&self) -> Vec<Point3D> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    neighbors.push(Point3D::new(self.x + dx, self.y + dy, self.z + dz));
                }
            }
        }
        neighbors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point4D {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point4D {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Point4D {
        Point4D { x, y, z, w }
    }
}

impl Point for Point4D {
    fn get_neighbors(&self) -> Vec<Point4D> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                            continue;
                        }
                        neighbors.push(Point4D::new(
                            self.x + dx,
                            self.y + dy,
                            self.z + dz,
                            self.w + dw,
                        ));
                    }
                }
            }
        }
        neighbors
    }
}

struct Configuration<T: Point> {
    cells: HashSet<T>,
}

impl<T: Point> Configuration<T> {
    fn count_active_neighbors(&self, p: &T) -> i32 {
        let mut count = 0;
        for n in p.get_neighbors() {
            if self.cells.contains(&n) {
                count += 1;
            }
        }
        count
    }

    fn next(&self) -> Configuration<T> {
        let mut next_config = HashSet::new();
        for p in &self.cells {
            let count = self.count_active_neighbors(p);
            if count == 2 || count == 3 {
                next_config.insert(p.clone());
            }

            for n in p.get_neighbors() {
                if !self.cells.contains(&n) {
                    let count = self.count_active_neighbors(&n);
                    if count == 3 {
                        next_config.insert(n.clone());
                    }
                }
            }
        }
        Configuration { cells: next_config }
    }
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2020/day17/input.txt").unwrap();
    let mut active_cells = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active_cells.insert(Point3D::new(i as i32, j as i32, 0));
            }
        }
    }
    let mut c = Configuration {
        cells: active_cells,
    };
    for _ in 0..6 {
        c = c.next();
    }
    c.cells.len() as i32
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2020/day17/input.txt").unwrap();
    let mut active_cells = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                active_cells.insert(Point4D::new(i as i32, j as i32, 0, 0));
            }
        }
    }
    let mut c = Configuration {
        cells: active_cells,
    };
    for _ in 0..6 {
        c = c.next();
    }
    c.cells.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 338);
    }

    #[test]
    #[ignore="long test (6s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
