use rand::{self, Rng};
use std::collections::VecDeque;
use std::fmt::Display;

use crate::utils;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RockShape {
    points: Vec<Point>,
    max_x: i32,
    max_y: i32,
}

impl RockShape {
    pub fn new(points: Vec<Point>) -> RockShape {
        let mut max_x = 0;
        let mut max_y = 0;
        for point in points.iter() {
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }
        RockShape {
            points,
            max_x,
            max_y,
        }
    }
}

pub fn make_rock_shapes() -> [RockShape; 5] {
    [
        // - shape
        RockShape::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ]),
        // + shape
        RockShape::new(vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ]),
        // J shape
        RockShape::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ]),
        // I shape
        RockShape::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ]),
        // square shape
        RockShape::new(vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ]),
    ]
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RockBlock {
    pub shape: RockShape,
    // absolute position of the block in the chamber
    pub position: Point,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Chamber {
    // absolute height of the highest point in the chamber (only fully dropped blocks count)
    pub top_height: i32,
    // block that is currently falling (position of the block is absolute from the bottom of the chamber)
    pub current_block: Option<RockBlock>,
    pub grid: VecDeque<[bool; 7]>,
    pub shapes: [RockShape; 5],
    pub shapes_index: usize,
    pub jet_patterns: Vec<i32>,
    pub jet_index: usize,
    pub height_shift: i32,
}

impl Chamber {
    pub fn new(grid_size: usize, jet_patterns: Vec<i32>) -> Chamber {
        let mut grid = VecDeque::new();
        grid.push_back([true; 7]);
        for _ in 0..(grid_size - 1) {
            grid.push_back([false; 7]);
        }

        Chamber {
            top_height: 0,
            current_block: None,
            grid,
            shapes: make_rock_shapes(),
            shapes_index: 0,
            jet_patterns,
            jet_index: 0,
            height_shift: -1,
        }
    }

    pub fn run_to_sync(&mut self) -> i32 {
        let mut block_counter = 0;
        loop {
            for _ in 0..5 {
                self.drop_new_block();
            }
            block_counter += 5;
            if self.jet_index == 0 {
                break;
            }
        }
        block_counter
    }

    pub fn drop_new_block(&mut self) {
        self.add_block();
        while self.current_block.is_some() {
            self.shift_block();
            self.drop_block();
        }
    }

    pub fn drop_new_block_set(&mut self) {
        for _ in 0..5 {
            self.add_block();
            while self.current_block.is_some() {
                self.shift_block();
                self.drop_block();
            }
        }
    }

    fn add_block(&mut self) {
        let shape = &self.shapes[self.shapes_index];
        self.shapes_index = (self.shapes_index + 1) % self.shapes.len();

        while self.grid.len() <= (self.top_height + 3 + shape.max_y - self.height_shift) as usize {
            // slide the grid to make room for the new block
            self.grid.pop_front().unwrap();
            self.grid.push_back([false; 7]);
            self.height_shift += 1;
        }

        self.current_block = Some(RockBlock {
            shape: shape.clone(),
            position: Point {
                x: 2,
                y: self.top_height + 3,
            },
        });
    }

    fn shift_block(&mut self) {
        let direction = self.jet_patterns[self.jet_index];
        self.jet_index = (self.jet_index + 1) % self.jet_patterns.len();

        if let Some(ref mut block) = self.current_block {
            if block.position.x + direction < 0
                || block.position.x + direction + block.shape.max_x >= 7
            {
                // block would hit a wall
                return;
            }
            for point in block.shape.points.iter() {
                if self.grid[(block.position.y + point.y - self.height_shift) as usize]
                    [(block.position.x + point.x + direction) as usize]
                {
                    // block would hit another block
                    return;
                }
            }
            block.position.x += direction;
        }
    }

    fn drop_block(&mut self) {
        if let Some(ref mut block) = self.current_block {
            if block.position.y <= self.height_shift {
                panic!("block fell below the grid window");
            }
            let mut can_drop = true;
            for point in block.shape.points.iter() {
                if self.grid[(block.position.y + point.y - self.height_shift - 1) as usize]
                    [(block.position.x + point.x) as usize]
                {
                    // block would hit another block
                    can_drop = false;
                    break;
                }
            }
            if can_drop {
                block.position.y -= 1;
            } else {
                // freeze the block
                for point in block.shape.points.iter() {
                    self.grid[(block.position.y + point.y - self.height_shift) as usize]
                        [(point.x + block.position.x) as usize] = true;
                }
                self.top_height = self
                    .top_height
                    .max(block.position.y + block.shape.max_y + 1);
                self.current_block = None;
            }
        }
    }
}

fn make_random_jet_patterns(n: i32) -> Vec<i32> {
    let mut jet_patterns = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        jet_patterns.push(2 * rng.gen_range(0..=1) - 1);
    }
    jet_patterns
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_copy = self.grid.clone();
        if let Some(block) = &self.current_block {
            for point in block.shape.points.iter() {
                grid_copy[(block.position.y + point.y - self.height_shift) as usize]
                    [(point.x + block.position.x) as usize] = true;
            }
        }
        for row in grid_copy.iter().rev() {
            for cell in row.iter() {
                if *cell {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn parse_input(filename: &str) -> Vec<i32> {
    let input = utils::read_input(filename).unwrap();
    let mut jet_patterns = Vec::new();
    for c in input.chars() {
        match c {
            '<' => jet_patterns.push(-1),
            '>' => jet_patterns.push(1),
            _ => (),
        }
    }
    jet_patterns
}

pub fn solve1() -> i32 {
    let jet_patterns = parse_input("src/day17/input.txt");
    let mut chamber = Chamber::new(200, jet_patterns);

    for _ in 0..2022 {
        chamber.drop_new_block();
    }
    // println!("{}", chamber);
    chamber.top_height
}

pub fn solve2() -> i64 {
    let jet_patterns = parse_input("src/day17/input.txt");
    // let jet_patterns = make_random_jet_patterns(10091);
    let mut chamber1 = Chamber::new(200, jet_patterns.clone());
    let mut chamber2 = Chamber::new(200, jet_patterns.clone());

    let mut steps_difference = 0;
    loop {
        chamber1.drop_new_block_set();
        chamber2.drop_new_block_set();
        chamber2.drop_new_block_set();
        steps_difference += 1;
        if chamber1.jet_index == chamber2.jet_index && chamber1.grid == chamber2.grid {
            break;
        }
    }
    println!("steps_difference: {}", steps_difference);

    let height1 = chamber1.top_height as i64;
    let height2 = chamber2.top_height as i64;
    let nb_steps: i64 = 1000000000000 / 5;
    let nb_cycles = nb_steps / steps_difference as i64 - 1;
    let extra_steps = nb_steps % steps_difference as i64;
    for _ in 0..extra_steps {
        chamber2.drop_new_block_set();
    }
    let height3 = chamber2.top_height as i64;

    height1 + (height2 - height1) * nb_cycles + (height3 - height2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 3109);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1541449275365);
    }
}
