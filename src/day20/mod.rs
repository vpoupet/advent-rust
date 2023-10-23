use crate::utils;

fn mix(mut values: Vec<i64>, nb_times: usize) -> Vec<i64> {
    let n = values.len();
    // indexes[i] is the current index in values of the value that was initially at index i
    let mut indexes = (0..n).collect::<Vec<usize>>();

    for _ in 0..nb_times {
        for i in 0..n {
            let start_index = indexes[i];
            let value = values[start_index];
            values.remove(start_index);
            let end_index = (start_index as i64 + value).rem_euclid(n as i64 - 1) as usize;
            values.insert(end_index, value);

            if start_index < end_index {
                for j in 0..n {
                    if start_index < indexes[j] && indexes[j] <= end_index {
                        indexes[j] -= 1;
                    }
                }
            } else if end_index < start_index {
                for j in 0..n {
                    if end_index <= indexes[j] && indexes[j] < start_index {
                        indexes[j] += 1;
                    }
                }
            }

            indexes[i] = end_index;
        }
    }

    values
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/day20/input.txt").unwrap();
    let mut values = Vec::new();
    for line in input.lines() {
        values.push(line.parse::<i64>().unwrap());
    }

    let values = mix(values, 1);
    let n = values.len();

    let zero_index = values.iter().position(|&x| x == 0).unwrap();

    values[(zero_index + 1000) % n]
        + values[(zero_index + 2000) % n]
        + values[(zero_index + 3000) % n]
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/day20/input.txt").unwrap();
    let mut values = Vec::new();
    for line in input.lines() {
        values.push(line.parse::<i64>().unwrap() * 811589153);
    }
    
    let values = mix(values, 10);
    let n = values.len();

    let zero_index = values.iter().position(|&x| x == 0).unwrap();

    values[(zero_index + 1000) % n]
        + values[(zero_index + 2000) % n]
        + values[(zero_index + 3000) % n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 13967);
    }

    #[test]
    #[ignore = "long test (7s)"]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 1790365671518);
    }
}
