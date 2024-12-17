use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, opt};
use nom::sequence::pair;
use nom::IResult;
use std::fs::File;
use std::io::Read;
use std::ops::Neg;
use std::str::FromStr;

pub fn read_input(filename: &str) -> Result<String, std::io::Error> {
    // open file given as input and returns its content as a String
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_unsigned_int<T: FromStr>(input: &str) -> IResult<&str, T> {
    map(digit1, |number: &str| {
        let value = match number.parse::<T>() {
            Ok(v) => v,
            Err(_) => panic!("Could not parse int"),
        };
        return value;
    })(input)
}

pub fn parse_int<T: FromStr + Neg<Output = T>>(input: &str) -> IResult<&str, T> {
    map(
        pair(opt(alt((char('-'), char('+')))), digit1),
        |(sign, number): (Option<char>, &str)| {
            let value = match number.parse::<T>() {
                Ok(v) => v,
                Err(_) => panic!("Could not parse int"),
            };
            if sign == Some('-') {
                return -value;
            }
            return value;
        },
    )(input)
}

/// Divides a by b and rounds up
/// 
/// # Arguments
/// 
/// * `a` - The dividend
/// * `b` - The divisor
/// 
/// # Returns
/// 
/// The result of the integer division rounded up
pub fn div_up(a: i32, b: i32) -> i32 {
    (a + (b - 1)) / b
}

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn bezout(a: i32, b: i32) -> (i32, i32, i32) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);
    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    (r.0, s.0, t.0)
}