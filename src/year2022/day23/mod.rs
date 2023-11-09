use std::collections::HashSet;

use crate::utils;

static DIRECTIONS: [Vec2; 4] = [
    Vec2 { x: 0, y: 1 },  // North
    Vec2 { x: 0, y: -1 }, // South
    Vec2 { x: -1, y: 0 }, // West
    Vec2 { x: 1, y: 0 },  // East
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    fn add(&self, other: &Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    fn orth(&self) -> Vec2 {
        Vec2::new(-self.y, self.x)
    }

    fn neg(&self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

fn get_proposition(config: &HashSet<Vec2>, p: &Vec2, dir: usize) -> Option<Vec2> {
    let mut neighbors = [false; 4];
    let mut has_neighbors = false;
    for i in 0..4 {
        let direction = DIRECTIONS[(i + dir) % 4];
        let target = p.add(&direction);
        if config.contains(&target) {
            neighbors[i] = true;
            has_neighbors = true;
        }
        if config.contains(&target.add(&direction.orth())) {
            neighbors[i] = true;
            has_neighbors = true;
        }
        if config.contains(&target.add(&direction.orth().neg())) {
            neighbors[i] = true;
            has_neighbors = true;
        }
    }

    if has_neighbors {
        for i in 0..4 {
            if !neighbors[i] {
                return Some(p.add(&DIRECTIONS[(i + dir) % 4]));
            }
        }
    }

    None
}

fn next(config: &HashSet<Vec2>, dir: usize) -> (HashSet<Vec2>, bool) {
    let mut new_config = HashSet::new();
    let mut did_move = false;
    for p in config {
        match get_proposition(config, p, dir) {
            Some(prop) => {
                let mut can_move = true;
                for neighbor_direction in &DIRECTIONS {
                    let neighbor = prop.add(neighbor_direction);
                    if neighbor != *p
                        && config.contains(&neighbor)
                        && get_proposition(config, &neighbor, dir) == Some(prop)
                    {
                        can_move = false;
                        break;
                    }
                }
                if can_move {
                    new_config.insert(prop);
                    did_move = true;
                } else {
                    new_config.insert(*p);
                }
            }
            None => {
                new_config.insert(*p);
            }
        }
    }
    (new_config, did_move)
}

pub fn solve1() -> i32 {
    let mut config = HashSet::new();

    let input = utils::read_input("src/year2022/day23/input.txt").unwrap();
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                config.insert(Vec2::new(x as i32, y as i32));
            }
        }
    }

    let mut dir = 0;

    for _ in 0..10 {
        (config, _) = next(&config, dir);
        dir = (dir + 1) % 4;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for p in &config {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }

    (max_x - min_x + 1) * (max_y - min_y + 1) - config.len() as i32
}

pub fn solve2() -> i32 {
    let mut config = HashSet::new();

    let input = utils::read_input("src/year2022/day23/input.txt").unwrap();
    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                config.insert(Vec2::new(x as i32, y as i32));
            }
        }
    }

    let mut dir = 0;
    let mut counter = 1;
    loop {
        let (new_config, did_move) = next(&config, dir);
        config = new_config;
        if !did_move {
            break;
        }
        dir = (dir + 1) % 4;
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 4034);
    }

    #[test]
    #[ignore = "long test (9s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 960);
    }
}
