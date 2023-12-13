use std::collections::HashSet;

use crate::utils;

fn make_galaxies() -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let input = utils::read_input("src/year2023/day11/input.txt").unwrap();
    for (i, line) in input.lines().enumerate() {
        for (j, chars) in line.chars().enumerate() {
            if chars == '#' {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn get_total_distance(galaxies: &Vec<(usize, usize)>, expansion: i64) -> i64 {
    let rows = galaxies.iter().map(|(i, _)| i).collect::<HashSet<_>>();
    let cols = galaxies.iter().map(|(_, j)| j).collect::<HashSet<_>>();

    let mut total = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let (i1, j1) = galaxies[i];
            let (i2, j2) = galaxies[j];
            let mut dist = 0;
            for row in i1.min(i2)..i1.max(i2) {
                if rows.contains(&row) {
                    dist += 1;
                } else {
                    dist += expansion;
                }
            }
            for col in j1.min(j2)..j1.max(j2) {
                if cols.contains(&col) {
                    dist += 1;
                } else {
                    dist += expansion;
                }
            }
            total += dist;
        }
    }
    total
}

pub fn solve1() -> i64 {
    let galaxies = make_galaxies();
    get_total_distance(&galaxies, 2)
}

pub fn solve2() -> i64 {
    let galaxies = make_galaxies();
    get_total_distance(&galaxies, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 9521550);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 298932923702);
    }
}
