use crate::utils;

fn snafu_to_int(snafu: &str) -> i64 {
    let mut total = 0;
    for c in snafu.chars() {
        match c {
            '2' => total = 5 * total + 2,
            '1' => total = 5 * total + 1,
            '0' => total = 5 * total,
            '-' => total = 5 * total - 1,
            '=' => total = 5 * total - 2,
            _ => panic!("Invalid character in snafu: {}", c),
        }
    }
    total
}

fn int_to_snafu(n: i64) -> String {
    let mut n = n;
    let mut result = String::new();
    while n != 0 {
        match n.rem_euclid(5) {
            0 => {
                result.insert(0, '0');
            }
            1 => {
                result.insert(0, '1');
                n -= 1;
            }
            2 => {
                result.insert(0, '2');
                n -= 2;
            }
            3 => {
                result.insert(0, '=');
                n += 2;
            }
            4 => {
                result.insert(0, '-');
                n += 1;
            }
            _ => panic!("Invalid integer: {}", n),
        }
        n /= 5;
    }
    result
}

pub fn solve1() -> String {
    let input = utils::read_input("src/day25/input.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        total += snafu_to_int(line);
    }
    int_to_snafu(total)
}

pub fn solve2() -> i32 {
    let _input = utils::read_input("src/day25/input.txt").unwrap();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_int() {
        assert_eq!(1, snafu_to_int("1"));
        assert_eq!(2, snafu_to_int("2"));
        assert_eq!(3, snafu_to_int("1="));
        assert_eq!(4, snafu_to_int("1-"));
        assert_eq!(5, snafu_to_int("10"));
        assert_eq!(6, snafu_to_int("11"));
        assert_eq!(7, snafu_to_int("12"));
        assert_eq!(8, snafu_to_int("2="));
        assert_eq!(9, snafu_to_int("2-"));
        assert_eq!(10, snafu_to_int("20"));
        assert_eq!(15, snafu_to_int("1=0"));
        assert_eq!(20, snafu_to_int("1-0"));
        assert_eq!(2022, snafu_to_int("1=11-2"));
        assert_eq!(12345, snafu_to_int("1-0---0"));
        assert_eq!(314159265, snafu_to_int("1121-1110-1=0"));
    }

    #[test]
    fn test_int_to_snafu() {
        assert_eq!(int_to_snafu(1), "1");
        assert_eq!(int_to_snafu(2), "2");
        assert_eq!(int_to_snafu(3), "1=");
        assert_eq!(int_to_snafu(4), "1-");
        assert_eq!(int_to_snafu(5), "10");
        assert_eq!(int_to_snafu(6), "11");
        assert_eq!(int_to_snafu(7), "12");
        assert_eq!(int_to_snafu(8), "2=");
        assert_eq!(int_to_snafu(9), "2-");
        assert_eq!(int_to_snafu(10), "20");
        assert_eq!(int_to_snafu(15), "1=0");
        assert_eq!(int_to_snafu(20), "1-0");
        assert_eq!(int_to_snafu(2022), "1=11-2");
        assert_eq!(int_to_snafu(12345), "1-0---0");
        assert_eq!(int_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_solve1() {
        let solution = solve1();
        println!("Part One: {}", solution);
        assert_eq!(solution, "2=0=02-0----2-=02-10");
    }
}
