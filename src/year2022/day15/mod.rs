use std::{cmp::Ordering, collections::HashSet};

use crate::utils;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

impl Sensor {
    fn radius(&self) -> i32 {
        (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs()
    }

    fn get_diamond(&self) -> Diamond {
        let r = self.radius();
        Diamond {
            diag_interval: Interval {
                start: self.x - self.y - r,
                end: self.x - self.y + r + 1,
            },
            anti_interval: Interval {
                start: self.x + self.y - r,
                end: self.x + self.y + r + 1,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    fn sub(&self, other: &Interval) -> Vec<Interval> {
        let mut intervals = Vec::new();
        if self.start < other.start {
            intervals.push(Interval {
                start: self.start,
                end: self.end.min(other.start),
            });
        }
        if self.end > other.end {
            intervals.push(Interval {
                start: self.start.max(other.end),
                end: self.end,
            });
        }
        intervals
    }

    fn intersect(&self, other: &Interval) -> Option<Interval> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(Interval { start, end })
        } else {
            None
        }
    }

    fn size(&self) -> i32 {
        self.end - self.start
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

#[derive(Debug, Copy, Clone)]
struct Diamond {
    // diagonal: x - y = cst
    diag_interval: Interval,
    // anti-diagonal: x + y = cst
    anti_interval: Interval,
}

impl Diamond {
    fn sub(&self, other: &Diamond) -> Vec<Diamond> {
        let mut result = Vec::new();
        for i in self.diag_interval.sub(&other.diag_interval) {
            result.push(Diamond {
                diag_interval: i,
                anti_interval: self.anti_interval,
            });
        }
        for i in self.anti_interval.sub(&other.anti_interval) {
            if let Some(j) = self.diag_interval.intersect(&other.diag_interval) {
                result.push(Diamond {
                    diag_interval: j,
                    anti_interval: i,
                });
            }
        }
        result
    }
}

fn parse_line(input: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            preceded(tag("Sensor at x="), utils::parse_int),
            preceded(tag(", y="), utils::parse_int),
            preceded(tag(": closest beacon is at x="), utils::parse_int),
            preceded(tag(", y="), utils::parse_int),
        )),
        |(x, y, beacon_x, beacon_y)| Sensor {
            x,
            y,
            beacon_x,
            beacon_y,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(tag("\n"), parse_line)(input)
}

pub fn solve1() -> i32 {
    // read input and make list of sensors data
    let input = utils::read_input("src/year2022/day15/input.txt").unwrap();
    let (_, sensors) = parse_input(input.as_str()).unwrap();
    let mut intervals = Vec::new();

    // line on which we count the number of positions that cannot contain a beacon
    let line_number = 2000000;

    // for each sensor, compute the interval of positions that are covered by the sensor
    for s in &sensors {
        let r = s.radius();
        if (s.y - line_number).abs() <= r {
            let d = r - (s.y - line_number).abs();
            intervals.push(Interval {
                start: s.x - d,
                end: s.x + d + 1,
            });
        }
    }

    // merge the overlapping intervals
    intervals.sort();
    let mut merged = Vec::new();
    for i in intervals {
        if merged.is_empty() {
            merged.push(i);
        } else {
            let last = merged.last_mut().unwrap();
            if last.end >= i.start {
                last.end = last.end.max(i.end);
            } else {
                merged.push(i);
            }
        }
    }
    // count total length of intervals
    let mut total = 0;
    for i in merged {
        total += i.size();
    }

    // count the number of beacons that are on the line
    let mut beacons_on_line = HashSet::new();
    for s in &sensors {
        if s.beacon_y == line_number {
            beacons_on_line.insert(s.beacon_x);
        }
    }

    total - beacons_on_line.len() as i32
}

pub fn solve2() -> i64 {
    let input = utils::read_input("src/year2022/day15/input.txt").unwrap();
    let (_, sensors) = parse_input(input.as_str()).unwrap();

    // make initial search space containing the [0, 4M] x [0, 4M] area
    let min_diag = -4_000_000;
    let max_diag = 4_000_000;
    let min_anti = 0;
    let max_anti = 8_000_000;
    let mut search_space = Vec::new();
    search_space.push(Diamond {
        diag_interval: Interval {
            start: min_diag,
            end: max_diag + 1,
        },
        anti_interval: Interval {
            start: min_anti,
            end: max_anti + 1,
        },
    });

    // remove the diamond covered by each sensor from the search space
    for sensor in &sensors {
        let sensor_diamond = sensor.get_diamond();
        let mut new_search_space = Vec::new();
        for space_diamond in &search_space {
            new_search_space.append(&mut space_diamond.sub(&sensor_diamond));
        }
        search_space = new_search_space;
    }

    // look for a diamond in the search space that isn't limited by one of the extremal diagonals (~ finite diamonds)
    for diamond in &search_space {
        if diamond.diag_interval.start > min_diag
            && diamond.diag_interval.end < max_diag
            && diamond.anti_interval.start > min_anti
            && diamond.anti_interval.end < max_anti
        {
            // return value for first point in the diamond
            let diagonal = diamond.diag_interval.start;
            let anti_diagonal = diamond.anti_interval.start;
            let x = (diagonal + anti_diagonal) as i64 / 2;
            let y = (anti_diagonal - diagonal) as i64 / 2;
            return 4_000_000 * x + y;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 6275922);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 11747175442119);
    }
}
