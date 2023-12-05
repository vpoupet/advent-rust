use crate::utils;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, one_of},
    combinator::map,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Job {
    Number(i64),
    Operation(String, char, String),
    Unknown,
}

fn parse_job(input: &str) -> IResult<&str, Job> {
    alt((
        map(utils::parse_int, |x| Job::Number(x)),
        map(
            tuple((
                alpha1,
                delimited(tag(" "), one_of("+*-/"), tag(" ")),
                alpha1,
            )),
            |x| Job::Operation(String::from(x.0), x.1, String::from(x.2)),
        ),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Job)> {
    separated_pair(alpha1, tag(": "), parse_job)(input)
}

fn make_jobs(filename: &str) -> HashMap<String, Job> {
    let input = utils::read_input(filename).unwrap();
    let mut jobs = HashMap::new();

    for line in input.lines() {
        let (_, (label, job)) = parse_line(line).unwrap();
        jobs.insert(label.to_string(), job);
    }

    jobs
}

fn eval(label: &str, mut jobs: HashMap<String, Job>) -> (Option<i64>, HashMap<String, Job>) {
    let job = jobs.get(label).unwrap().clone();

    match job {
        Job::Number(x) => return (Some(x), jobs),
        Job::Operation(label1, op, label2) => {
            let (val1, mut jobs) = eval(&label1, jobs);
            let (val2, mut jobs) = eval(&label2, jobs);
            match (val1, val2) {
                (Some(val1), Some(val2)) => {
                    let value = match op {
                        '+' => val1 + val2,
                        '*' => val1 * val2,
                        '-' => val1 - val2,
                        '/' => val1 / val2,
                        _ => panic!("Unknown operator"),
                    };
                    jobs.insert(label.to_string(), Job::Number(value));
                    return (Some(value), jobs);
                }
                _ => return (None, jobs),
            }
        }
        Job::Unknown => return (None, jobs),
    }
}

fn set_value(label: &str, value: i64, mut jobs: HashMap<String, Job>) -> HashMap<String, Job> {
    let job = jobs.get(label).unwrap().clone();
    match job {
        Job::Unknown => {
            jobs.insert(label.to_string(), Job::Number(value));
        }
        Job::Operation(label1, op, label2) => {
            let (val1, j) = eval(&label1, jobs);
            jobs = j;
            let (val2, j) = eval(&label2, jobs);
            jobs = j;
            match (val1, val2) {
                (Some(val1), None) => match op {
                    '+' => jobs = set_value(&label2, value - val1, jobs),
                    '*' => jobs = set_value(&label2, value / val1, jobs),
                    '-' => jobs = set_value(&label2, val1 - value, jobs),
                    '/' => jobs = set_value(&label2, val1 / value, jobs),
                    _ => panic!("Unknown operator"),
                },
                (None, Some(val2)) => match op {
                    '+' => jobs = set_value(&label1, value - val2, jobs),
                    '*' => jobs = set_value(&label1, value / val2, jobs),
                    '-' => jobs = set_value(&label1, value + val2, jobs),
                    '/' => jobs = set_value(&label1, value * val2, jobs),
                    _ => panic!("Unknown operator"),
                },
                _ => {
                    panic!("Cannot set value");
                }
            }
        }
        _ => panic!("Cannot set value"),
    }
    jobs
}

pub fn solve1() -> i64 {
    let jobs = make_jobs("src/year2022/day21/input.txt");
    let (result, _) = eval("root", jobs);
    result.unwrap()
}

pub fn solve2() -> i64 {
    let mut jobs = make_jobs("src/year2022/day21/input.txt");
    let label1;
    let label2;
    // get two labels that should be equal from root
    let root_job = jobs.get("root").unwrap();
    match root_job {
        Job::Operation(a, _, b) => {
            label1 = a.clone();
            label2 = b.clone();
        }
        _ => panic!("Root job is not an operation"),
    }
    jobs.remove("root");
    // set job "humn" as Unknown
    jobs.insert(String::from("humn"), Job::Unknown);

    let (val1, j) = eval(&label1, jobs);
    jobs = j;
    let (val2, j) = eval(&label2, jobs);
    jobs = j;
    match (val1, val2) {
        (Some(val1), None) => {
            jobs = set_value(&label2, val1, jobs);
        }
        (None, Some(val2)) => {
            jobs = set_value(&label1, val2, jobs);
        }
        _ => panic!("No value to set"),
    }

    let (val, _) = eval("humn", jobs);
    val.unwrap()
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
        assert_eq!(solution, 3916936880448);
    }
}
