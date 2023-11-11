use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::pair;
use nom::IResult;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};

pub fn read_input(filename: &str) -> Result<String, std::io::Error> {
    // open file given as input and returns its content as a String
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_int(input: &str) -> IResult<&str, i32> {
    map(
        pair(opt(alt((char('-'), char('+')))), digit1),
        |(sign, number): (Option<char>, &str)| {
            let value = number.parse::<i32>().unwrap();
            if sign == Some('-') {
                return -value;
            }
            return value;
        },
    )(input)
}

pub fn div_up(a: i32, b: i32) -> i32 {
    (a + (b - 1)) / b
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
