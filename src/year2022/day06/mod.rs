use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/year2022/day06/input.txt").unwrap();
    let mut buffer = Vec::new();
    
    let mut chars = input.chars();
    let mut counter = 0;
    for _ in 0..4 {
        buffer.push(chars.next().unwrap());
        counter += 1;
    }

    loop {
        if buffer[0] != buffer[1] && buffer[0] != buffer[2] && buffer[0] != buffer[3]
        && buffer[1] != buffer[2] && buffer[1] != buffer[3] && buffer[2] != buffer[3] {
            return counter;
        }
        buffer.remove(0);
        buffer.push(chars.next().unwrap());
        counter += 1;
    }
}

fn are_all_different(buffer: &Vec<char>) -> bool {
    for i in 0..buffer.len() {
        for j in i+1..buffer.len() {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    true
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/year2022/day06/input.txt").unwrap();
    let mut buffer = Vec::new();
    
    let mut chars = input.chars();
    let mut counter = 0;
    for _ in 0..14 {
        buffer.push(chars.next().unwrap());
        counter += 1;
    }

    loop {
        if are_all_different(&buffer) {
            return counter;
        }
        buffer.remove(0);
        buffer.push(chars.next().unwrap());
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 1623);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 3774);
    }
}
