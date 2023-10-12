use crate::utils;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, alpha1, digit1};
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;

struct Directory {
    _name: String,
    size: usize,
    total_size: Option<usize>,
    subdirs: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn new(_name: String, parent: Option<Rc<RefCell<Directory>>>) -> Directory {
        Directory {
            _name,
            size: 0,
            total_size: None,
            subdirs: Vec::new(),
            parent,
        }
    }

    fn get_total_size(&mut self) -> usize {
        if let Some(size) = self.total_size {
            return size;
        }
        let mut total_size = self.size;
        for subdir in &self.subdirs {
            total_size += subdir.borrow_mut().get_total_size();
        }
        self.total_size = Some(total_size);
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

fn parse_filename(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic() || c == '.')(input)
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
        map(separated_pair(digit1, char(' '), parse_filename), |(size, name): (&str, &str)| {
            Command::File(size.parse::<usize>().unwrap(), String::from(name))
        }),
    ))(input)?;
    Ok((remaining, command))
}

fn build_root() -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory::new(String::from("/"), None)));
    let mut current_dir = Rc::clone(&root);

    let input = utils::read_input("src/day07/input.txt").unwrap();
    for line in input.lines() {
        let (_, command) = parse_line(line).unwrap();
        match command {
            Command::CdParent => {
                let parent = Rc::clone(current_dir.borrow().parent.as_ref().unwrap());
                current_dir = parent;
            }
            Command::Cd(dirname) => {
                let new_dir = Rc::new(RefCell::new(Directory::new(
                    dirname,
                    Some(Rc::clone(&current_dir)),
                )));
                current_dir.borrow_mut().subdirs.push(Rc::clone(&new_dir));
                current_dir = new_dir;
            }
            Command::File(size, _) => {
                current_dir.borrow_mut().size += size;
            }
            _ => {}
        }
    }
    root
}

fn solve1_aux(dir: &mut Directory)-> usize {
    let mut sum_total = 0;
    for subdir in &dir.subdirs {
        sum_total += solve1_aux(&mut subdir.borrow_mut());
    }
    let size = dir.get_total_size();
    if size <= 100000 {
        sum_total += size;
    }
    sum_total
}

pub fn solve1() -> usize {
    let root = build_root();
    return solve1_aux(&mut root.borrow_mut());
}

fn solve2_aux(dir: &mut Directory, required_space: usize)-> usize {
    let mut best_size = 70000000;
    for subdir in &dir.subdirs {
        best_size = cmp::min(best_size, solve2_aux(&mut subdir.borrow_mut(), required_space));
    }
    let size = dir.get_total_size();
    if size >= required_space {
        best_size = cmp::min(best_size, size);
    }
    best_size
}

pub fn solve2() -> usize {
    let root = build_root();
    let used_space = root.borrow_mut().get_total_size();
    let required_space = used_space - 40000000;
    return solve2_aux(&mut root.borrow_mut(), required_space);
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
