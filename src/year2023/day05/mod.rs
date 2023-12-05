use crate::utils;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Interval {
    start: i64,
    end: i64,
}
impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: i64) -> bool {
        self.start <= value && value < self.end
    }
}

#[derive(Debug)]
struct IntervalMap {
    source: Interval,
    destination: Interval,
}
impl IntervalMap {
    fn new(source: Interval, destination: Interval) -> Self {
        Self {
            source,
            destination,
        }
    }

    fn convert(&self, value: i64) -> i64 {
        self.destination.start + value - self.source.start
    }

    fn convert_interval(&self, other: &Interval) -> (Vec<Interval>, Vec<Interval>) {
        let mut converted_intervals = Vec::new();
        let mut remaining_intervals = Vec::new();

        if other.start < self.source.start {
            // segment of interval before source is unchanged
            remaining_intervals.push(Interval::new(other.start, other.end.min(self.source.start)));
        }
        if other.end > self.source.end {
            // segment of interval after source is unchanged
            remaining_intervals.push(Interval::new(other.start.max(self.source.end), other.end));
        }
        if self.source.start < other.end && other.start < self.source.end {
            // the overlapping interval is converted
            converted_intervals.push(Interval::new(
                self.convert(other.start.max(self.source.start)),
                self.convert(other.end.min(self.source.end)),
            ));
        }

        (converted_intervals, remaining_intervals)
    }
}

struct Map {
    interval_maps: Vec<IntervalMap>,
}
impl Map {
    fn new(interval_maps: Vec<IntervalMap>) -> Self {
        Self { interval_maps }
    }

    fn convert(&self, value: i64) -> i64 {
        for interval_map in &self.interval_maps {
            if interval_map.source.contains(value) {
                return interval_map.convert(value);
            }
        }
        value
    }

    fn convert_intervals(&self, intervals: Vec<Interval>) -> Vec<Interval> {
        let mut converted_intervals = Vec::new();
        let mut remaining_intervals = intervals;
        for interval_map in &self.interval_maps {
            let mut new_remaining_intervals = Vec::new();
            for i in remaining_intervals {
                let (mut converted, mut remaining) = interval_map.convert_interval(&i);
                converted_intervals.append(&mut converted);
                new_remaining_intervals.append(&mut remaining);
            }
            remaining_intervals = new_remaining_intervals;
        }

        converted_intervals.append(&mut remaining_intervals);
        converted_intervals
    }
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    preceded(tag("seeds: "), separated_list1(tag(" "), utils::parse_int))(input)
}

fn parse_map_interval(input: &str) -> IResult<&str, IntervalMap> {
    map(
        tuple((
            utils::parse_int,
            preceded(tag(" "), utils::parse_int),
            preceded(tag(" "), utils::parse_int::<i64>),
        )),
        |(dest, source, len)| {
            IntervalMap::new(
                Interval::new(source, source + len),
                Interval::new(dest, dest + len),
            )
        },
    )(input)
}

fn make_seeds() -> Vec<i64> {
    let input = utils::read_input("src/year2023/day05/seeds.txt").unwrap();
    let (_, seeds) = parse_seeds(&input).unwrap();
    seeds
}

fn make_maps() -> Vec<Map> {
    let input = utils::read_input("src/year2023/day05/maps.txt").unwrap();
    let mut maps = Vec::new();
    let mut current_intervals = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            maps.push(Map::new(current_intervals));
            current_intervals = Vec::new();
            continue;
        }
        if let Ok((_, interval)) = parse_map_interval(&line) {
            current_intervals.push(interval);
        }
    }
    maps.push(Map::new(current_intervals));
    maps
}

pub fn solve1() -> i64 {
    let seeds = make_seeds();
    let maps = make_maps();
    let mut positions = Vec::new();

    for seed in seeds {
        let mut x = seed;
        for map in &maps {
            x = map.convert(x);
        }
        positions.push(x);
    }
    positions.sort();
    positions[0]
}

pub fn solve2() -> i64 {
    let seeds = make_seeds();
    let mut intervals = Vec::new();
    for i in 0..(seeds.len() / 2) {
        intervals.push(Interval::new(seeds[2 * i], seeds[2 * i] + seeds[2 * i + 1]));
    }

    let maps = make_maps();
    for map in maps {
        intervals = map.convert_intervals(intervals);
    }

    intervals.sort_by_key(|i| i.start);
    intervals[0].start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 227653707);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        assert_eq!(solution, 78775051);
    }
}
