use crate::utils;

fn to_priorities(items: &str) -> Vec<i32> {
    let mut priorities = Vec::new();
    for item in items.chars() {
        match item {
            'a'..='z' => priorities.push(item as i32 - 'a' as i32 + 1),
            'A'..='Z' => priorities.push(item as i32 - 'A' as i32 + 27),
            _ => {}
        }
    }
    priorities
}

pub fn solve1() -> i32 {
    let input = utils::read_input("src/day03/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let first = &line[..line.len() / 2];
        let second = &line[line.len() / 2..];
        let first_priority = to_priorities(first);
        let second_priority = to_priorities(second);
        'outer: for i in 0..first_priority.len() {
            for j in 0..second_priority.len() {
                if first_priority[i] == second_priority[j] {
                    total += first_priority[i];
                    break 'outer;
                }
            }
        }
    }
    total
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/day03/input.txt").unwrap();
    let mut lines = input.lines();
    let mut total = 0;

    loop {
        let mut group = Vec::new();
        let first = lines.next();
        if first == None {
            break;
        }
        group.push(to_priorities(first.unwrap()));
        group.push(to_priorities(lines.next().unwrap()));
        group.push(to_priorities(lines.next().unwrap()));
        'outer: for i in &group[0] {
            for j in &group[1] {
                if i == j {
                    for k in &group[2] {
                        if i == k {
                            total += i;
                            break 'outer;
                        }
                    }
                }
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
        assert_eq!(solution, 8243);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 2631);
    }
}
