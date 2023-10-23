use crate::utils;

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day20/example.txt").unwrap();
    let mut values = Vec::new();

    for line in input.lines() {
        values.push(line.parse::<i32>().unwrap());
    }
    let n = values.len() as i32;

    println!("{:?}", values);
    for value in values.clone().iter() {
        let index = values.iter().position(|&x| x == *value).unwrap();
        values.remove(index);
        let new_index = ((index as i32 + *value + n) % n) as usize;
        if new_index > index {
            values.insert(new_index + 1, *value);
        } else {
            values.insert(new_index, *value);
        }
        println!("{:?}", values);
    }
    0
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/dayXX/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        // assert_eq!(solution, 0);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
