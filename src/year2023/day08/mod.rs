use nom::{bytes::complete::tag, multi::separated_list1};

use crate::utils;

fn get_input_sequences() -> Vec<Vec<i64>> {
    let input = utils::read_input("src/year2023/day08/input.txt").unwrap();
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let (_, values) = separated_list1(tag(" "), utils::parse_int)(line).unwrap();
        sequences.push(values);
    }
    sequences
}

fn extrapolate(sequence: Vec<i64>) -> (i64, i64) {
    if sequence.iter().all(|&x| x == sequence[0]) {
        return (sequence[0], sequence[0]);
    }
    let differences: Vec<i64> = sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
    let (d1, d2) = extrapolate(differences);
    (sequence[0] - d1, sequence[sequence.len() - 1] + d2)
}

pub fn solve1() -> i64 {
    let sequences = get_input_sequences();
    let mut total = 0;
    for seq in sequences {
        total += extrapolate(seq).1;
    }
    total
}

pub fn solve2() -> i64 {
    let sequences = get_input_sequences();
    let mut total = 0;
    for seq in sequences {
        total += extrapolate(seq).0;
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
        assert_eq!(solution, 1702218515);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 925);
    }
}
