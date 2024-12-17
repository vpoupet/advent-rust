use nom::{ bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple };
use crate::utils::{ self, parse_int };

struct Machine {
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_target: i64,
    y_target: i64,
}

impl Machine {
    fn solve(&self) -> Option<(i64, i64)> {
        let delta = self.x_a * self.y_b - self.x_b * self.y_a;
        if (self.y_b * self.x_target - self.x_b * self.y_target) % delta != 0 {
            return None;
        }
        if (self.x_a * self.y_target - self.y_a * self.x_target) % delta != 0 {
            return None;
        }
        return Some((
            (self.y_b * self.x_target - self.x_b * self.y_target) / delta,
            (self.x_a * self.y_target - self.y_a * self.x_target) / delta,
        ));
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let (_, machines) = separated_list1(
        tag("\n\n"),
        map(
            tuple((
                tuple((
                    tag("Button A: X+"),
                    parse_int::<i64>,
                    tag(", Y+"),
                    parse_int::<i64>,
                    tag("\n"),
                )),
                tuple((
                    tag("Button B: X+"),
                    parse_int::<i64>,
                    tag(", Y+"),
                    parse_int::<i64>,
                    tag("\n"),
                )),
                tuple((tag("Prize: X="), parse_int::<i64>, tag(", Y="), parse_int::<i64>)),
            )),
            |(button_a, button_b, target)| Machine {
                x_a: button_a.1,
                y_a: button_a.3,
                x_b: button_b.1,
                y_b: button_b.3,
                x_target: target.1,
                y_target: target.3,
            }
        )
    )(input).unwrap();
    machines
}

pub fn solve1() -> i64 {
    let input = utils::read_input("src/year2024/day13/input.txt").unwrap();
    let machines = parse_input(&input);

    let mut total = 0;
    for machine in machines {
        if let Some((i, j)) = machine.solve() {
            total += 3 * i + j;
        }
    }
    total
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2024/day13/input.txt").unwrap();
    let machines = parse_input(&input);

    let mut total = 0;
    for mut machine in machines {
        machine.x_target += 10000000000000;
        machine.y_target += 10000000000000;
        if let Some((i, j)) = machine.solve() {
            total += 3 * i + j;
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
        assert_eq!(solution, 39748);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 74478585072604);
    }
}
