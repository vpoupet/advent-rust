use nom::{ bytes::complete::tag, sequence::separated_pair, IResult };

use crate::utils::{ self, parse_unsigned_int };

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parse_unsigned_int, tag("   "), parse_unsigned_int)(input)
}

fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = utils::read_input(filename).unwrap();
    let mut t1 = Vec::new();
    let mut t2 = Vec::new();
    for line in input.lines() {
        let (_, (a, b)) = parse_line(line).unwrap();
        t1.push(a);
        t2.push(b);
    }
    (t1, t2)
}

pub fn solve1() -> i32 {
    let (mut left, mut right) = parse_input("src/year2024/day01/input.txt");
    left.sort();
    right.sort();
    let mut total = 0;
    for i in 0..left.len() {
        total += (left[i] - right[i]).abs();
    }
    total
}

pub fn solve2() -> i32 {
    let (left, right) = parse_input("src/year2024/day01/input.txt");
    let mut total = 0;
    for x in &left {
        for y in &right {
            if x == y {
                total += x;
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
        assert_eq!(solution, 1879048);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 21024792);
    }
}
