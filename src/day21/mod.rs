use crate::utils;
use lazy_static::lazy_static;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    combinator::map,
    sequence::{separated_pair, tuple, delimited},
    IResult,
};
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref JOBS: Mutex<HashMap<String, Job>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Operation(String, char, String),
    Unknown,
}

fn parse_job(input: &str) -> IResult<&str, Job> {
    alt((
        map(utils::parse_int, |x| Job::Number(x as i64)),
        map(tuple((alpha1, delimited(tag(" "), one_of("+*-/"), tag(" ")), alpha1)), |x| {
            Job::Operation(String::from(x.0), x.1, String::from(x.2))
        }),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Job)> {
    separated_pair(alpha1, tag(": "), parse_job)(input)
}

fn make_jobs(filename: &str) {
    let input = utils::read_input(filename).unwrap();
    let mut jobs = JOBS.lock().unwrap();

    for line in input.lines() {
        let (_, (label, job)) = parse_line(line).unwrap();
        jobs.insert(label.to_string(), job);
    }
}

fn eval(label: &str) -> Option<i64> {
    let job;
    {
        let jobs = JOBS.lock().unwrap();
        job = jobs.get(label).unwrap().clone();
    }
    match job {
        Job::Number(x) => return Some(x),
        Job::Operation(a, op, b) => {
            let x = eval(&a);
            let y = eval(&b);
            match (x, y) {
                (Some(x), Some(y)) => {
                    let value = match op {
                        '+' => x + y,
                        '*' => x * y,
                        '-' => x - y,
                        '/' => x / y,
                        _ => panic!("Unknown operator"),
                    };
                    let mut jobs = JOBS.lock().unwrap();
                    jobs.insert(label.to_string(), Job::Number(value));
                    return Some(value);
                }
                _ => return None,
            }
        }
        Job::Unknown => return None,
    }
}

pub fn solve1() -> i64 {
    make_jobs("src/day21/input.txt");
    eval("root").unwrap()
}

pub fn solve2() -> i32 {
    // let input = utils::read_input("src/dayXX/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, 41857219607906);
    }

    #[test]
    fn test_solve2() {
        let solution = solve2();
        println!("Part Two: {}", solution);
        // assert_eq!(solution, 0);
    }
}
