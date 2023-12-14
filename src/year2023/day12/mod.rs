use nom::{
    bytes::complete::{is_a, tag},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::utils;

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<usize>)> {
    separated_pair(
        is_a(".#?"),
        tag(" "),
        separated_list1(tag(","), utils::parse_unsigned_int),
    )(input)
}

fn get_lengths(input: &Vec<char>) -> Vec<i32> {
    let mut lengths: Vec<i32> = Vec::new();
    let mut counter = 0;
    for c in input {
        if *c == '#' {
            counter += 1;
        } else {
            if counter > 0 {
                lengths.push(counter);
                counter = 0;
            }
        }
    }
    if counter > 0 {
        lengths.push(counter);
    }
    lengths
}

fn count(springs: &[char], segments: &[usize]) -> i32 {
    match springs.get(0) {
        None => {
            if segments.len() == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        Some('.') => {
            return count(&springs[1..], segments);
        }
        Some('#') => {
            if segments.len() == 0 {
                return 0;
            }
            let n = segments[0];
            if springs.len() < n {
                return 0;
            } else if springs.len() == n && springs.iter().all(|c| *c != '.'){
                if segments.len() == 1 {
                    return 1;
                } else {
                    return 0;
                }
            } else if springs[..n].iter().all(|c| *c != '.')
                && (springs[n] != '#')
            {
                return count(&springs[n+1..], &segments[1..]);
            } else {
                return 0;
            }
        }
        Some('?') => {
            let c = count(&springs[1..], segments);
            let mut new_springs = springs.to_vec();
            new_springs[0] = '#';
            return c + count(&new_springs, segments);
        }
        _ => panic!("Invalid input"),
    }
}

fn count_solutions(springs: &str, lengths: Vec<i32>) -> i32 {
    let mut chars = Vec::new();
    let mut unknown_positions = Vec::new();
    let mut counter = 0;

    for (i, c) in springs.chars().enumerate() {
        if c == '?' {
            unknown_positions.push(i);
        }
        chars.push(c);
    }

    for n in 0..1 << unknown_positions.len() {
        for (i, p) in unknown_positions.iter().enumerate() {
            if n & (1 << i) != 0 {
                chars[*p] = '#';
            } else {
                chars[*p] = '.';
            }
        }
        if get_lengths(&chars) == lengths {
            counter += 1;
        }
    }
    counter
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2023/day12/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (springs, segments)) = parse_line(line).unwrap();
        let springs = springs.chars().collect::<Vec<char>>();
        total += count(&springs, &segments);
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2023/day12/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let (_, (root_springs, root_segments)) = parse_line(line).unwrap();
        let root_springs = root_springs.chars().collect::<Vec<char>>();
        println!("{:?} {:?}", root_springs, root_segments);

        let mut springs = root_springs.clone();
        for _ in 0..4 {
            springs.push('?');
            springs.append(&mut root_springs.clone());
        }

        let mut segments = root_segments.clone();
        for _ in 0..4 {
            segments.append(&mut root_segments.clone());
        }

        println!("{:?} {:?}", springs, segments);
        total += count(&springs, &segments);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(count_solutions("???.###", vec![1, 1, 3]), 1);
        assert_eq!(count_solutions(".??..??...?##.", vec![1, 1, 3]), 4);
    }

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        // assert_eq!(solution, 7017);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
