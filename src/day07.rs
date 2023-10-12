use crate::utils;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map;
use nom::error::ParseError;
use nom::sequence::{pair, preceded};
use nom::IResult;
use std::rc::Rc;

struct Directory {
    name: String,
    size: usize,
    subdirs: Vec<Rc<Directory>>,
    parent: Option<Rc<Directory>>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<Directory>>) -> Directory {
        Directory {
            name,
            size: 0,
            subdirs: Vec::new(),
            parent,
        }
    }

    fn get_total_size(&self) -> usize {
        let mut total_size = self.size;
        for subdir in &self.subdirs {
            total_size += subdir.get_total_size();
        }
        total_size
    }
}

enum Command {
    CdParent,
    Cd(String),
    Ls,
    Dir(String),
    File(usize, String),
}

fn parse_line(input: &str) -> IResult<&str, Command> {
    let (remaining, command) = alt((
        map(tag("$ cd .."), |_| Command::CdParent),
        map(preceded(tag("$ cd "), alpha1), |s: &str| {
            Command::Cd(s.to_string())
        }),
        map(tag("$ ls"), |_| Command::Ls),
        map(preceded(tag("dir "), alpha1), |s: &str| {
            Command::Dir(s.to_string())
        }),
        map(pair(digit1, alpha1), |(size, name): (&str, &str)| {
            Command::File(size.parse::<usize>().unwrap(), String::from(name))
        }),
    ))(input)?;
    Ok((remaining, command))
}

fn build_root() -> Rc<Directory> {
    let mut root = Rc::new(
        Directory::new(String::from("/"), None)
    );
    let mut current_dir = Rc::clone(&root);

    let input = utils::read_input("src/day07.txt").unwrap();
    for line in input.lines() {
        let (_, command) = parse_line(line).unwrap();
        match command {
            Command::CdParent => {
                if let Some(parent) = &current_dir.parent {
                    current_dir = Rc::clone(parent);
                }
            },
            Command::Cd(dirname) => {
                let mut new_dir = Rc::new(
                    Directory::new(dirname, Some(Rc::clone(&current_dir)))
                );
                current_dir.subdirs.push(new_dir);
                
            },
            _ => {}
        }
    }
    root
}

pub fn solve1() -> usize {
    let input = utils::read_input("src/dayXX.txt").unwrap();
    let root = build_root();
    0
}

pub fn solve2() -> i32 {
    let input = utils::read_input("src/dayXX.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        println!("Part One: {}", solve1());
        assert_eq!(solve1(), 1477771);
    }

    #[test]
    fn test_solve2() {
        println!("Part Two: {}", solve2());
        assert_eq!(solve2(), 3579501);
    }
}
