use nom::{
    bytes::complete::tag,
    character::complete::space1,
    multi::separated_list1,
    sequence::{pair, preceded},
};

use crate::utils;

fn parse_input() -> Vec<(i32, i32)> {
    let input = utils::read_input("src/year2023/day06/input.txt").unwrap();
    let mut lines = input.lines();

    // get times from first line
    let line = lines.next().unwrap();
    let (_, times) = preceded(
        pair(tag("Time:"), space1),
        separated_list1(space1, utils::parse_int),
    )(line)
    .unwrap();

    // get distances from second line
    let line = lines.next().unwrap();
    let (_, distances) = preceded(
        pair(tag("Distance:"), space1),
        separated_list1(space1, utils::parse_int),
    )(line)
    .unwrap();

    // zip times and distances into a vector of pairs
    times.into_iter().zip(distances.into_iter()).collect()
}

fn get_nb_solutions(t: f64, d: f64) -> i64 {
    let sq_delta = (t * t - 4. * d).sqrt();
    let x1 = (t - sq_delta) / 2.;
    let x2 = (t + sq_delta) / 2.;
    // y1 is the least integer > x1
    let y1 = if x1 == x1.ceil() {
        x1.ceil() + 1.
    } else {
        x1.ceil()
    } as i64;
    // y2 is the greatest integer < x2
    let y2 = if x2 == x2.floor() {
        x2.floor() - 1.
    } else {
        x2.floor()
    } as i64;
    y2 - y1 + 1
}

pub fn solve1() -> i64 {
    let stats = parse_input();
    let mut total = 1;

    for (t, d) in stats {
        total *= get_nb_solutions(t as f64, d as f64);
    }
    total
}

pub fn solve2() -> i64 {
    let stats = parse_input();
    // concatenate times and distances
    let mut time_str = String::new();
    let mut distance_str = String::new();
    for (t, d) in stats {
        time_str.push_str(&t.to_string());
        distance_str.push_str(&d.to_string());
    }

    get_nb_solutions(time_str.parse().unwrap(), distance_str.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 608902);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 46173809);
    }
}
