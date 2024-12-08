use std::collections::{ HashMap, HashSet };

use crate::utils;

fn get_antennas(input: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push((j, i));
            }
        }
    }
    antennas
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2024/day08/input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let antennas: HashMap<char, Vec<(usize, usize)>> = get_antennas(&input);
    
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x1, y1) = (x1 as i32, y1 as i32);
                let (x2, y2) = positions[j];
                let (x2, y2) = (x2 as i32, y2 as i32);
                let (x, y) = (2 * x1 - x2, 2 * y1 - y2);
                if x >= 0 && x < (height as i32) && y >= 0 && y < (width as i32) {
                    antinodes.insert((x as usize, y as usize));
                }
                let (x, y) = (2 * x2 - x1, 2 * y2 - y1);
                if x >= 0 && x < (height as i32) && y >= 0 && y < (width as i32) {
                    antinodes.insert((x as usize, y as usize));
                }
            }
        }
    }
    antinodes.len() as i32
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2024/day08/input.txt").unwrap();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let antennas = get_antennas(&input);
    
    let mut antinodes = HashSet::new();
    for (_, positions) in antennas {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let (x1, y1) = positions[i];
                let (x1, y1) = (x1 as i32, y1 as i32);
                let (x2, y2) = positions[j];
                let (x2, y2) = (x2 as i32, y2 as i32);
                let mut k;
                k = 0;
                loop {
                    let (x, y) = ((k + 1) * x1 - k * x2, (k + 1) * y1 - k * y2);
                    if x < 0 || x >= (height as i32) || y < 0 || y >= (width as i32) {
                        break;
                    }
                    antinodes.insert((x as usize, y as usize));
                    k += 1;
                }
                k = 0;
                loop {
                    let (x, y) = ((k + 1) * x2 - k * x1, (k + 1) * y2 - k * y1);
                    if x < 0 || x >= (height as i32) || y < 0 || y >= (width as i32) {
                        break;
                    }
                    antinodes.insert((x as usize, y as usize));
                    k += 1;
                }
            }
        }
    }
    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 252);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 839);
    }
}
