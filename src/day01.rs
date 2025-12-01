use nom::bytes::complete::tag;
use nom::character::complete;
use nom::{IResult, Parser};
use nom::sequence::{pair};
use crate::helpers;
use nom::branch::alt;
use nom::combinator::map;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day01.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day01.txt")
}

// dial start at 50
const INITIAL_DIAL_POS: i64 = 50;
const DIAL_SIZE: i64 = 100;

fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let count: &mut i64 = &mut 0;
    let res: (_, i64) = sample.iter()
        .map(|line| parse_row(line.as_str()).unwrap().1)
        .fold((count, INITIAL_DIAL_POS), |(cnt, acc), value| {
            let new_value = (acc + value) % DIAL_SIZE;
            if new_value == 0 {
                *cnt = *cnt + 1
            }
            (cnt, new_value)
        });
    *res.0
}

fn parse_row(line: &str) -> IResult<&str, i64> {
    let (_, pair) = pair(
        alt((
            map(tag("R"), |_| 1),
            map(tag("L"), |_| -1),
        )), complete::i64).parse(line)?;
    Ok((line, pair.0 * pair.1))
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let count_past_zero: &mut i64 = &mut 0;
    let res: (_, i64) = sample.iter()
        .map(|line| parse_row(line.as_str()).unwrap().1)
        .fold((count_past_zero, INITIAL_DIAL_POS), |(count, start_pos), value| {
            let mut res: i64 = 0;
            let new_pos: i64;
            if value < 0 {
                if start_pos + value <= 0 {
                    res = if start_pos == 0 {res} else {res + 1};
                    let remaining = start_pos + value;
                    let cycles = remaining.abs().div_euclid(DIAL_SIZE);
                    res = res + cycles;
                    new_pos = (DIAL_SIZE + remaining).rem_euclid(DIAL_SIZE);
                } else {
                    new_pos = start_pos + value;
                }
            } else {
                if start_pos + value >= DIAL_SIZE {
                    res = res + 1;
                    let remaining = value - (DIAL_SIZE - start_pos);
                    let cycles = remaining.abs().div_euclid(DIAL_SIZE);
                    res = res + cycles;
                    new_pos = remaining.rem_euclid(DIAL_SIZE);
                } else {
                    new_pos = start_pos + value;
                }
            }
            *count = *count + res;

            (count, new_pos)
        });
    *res.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 1118);
    }

    #[test]
    fn test_sample_1() {
        let res = read_from("src/input/sample01.txt");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_parse() {
        let right = parse_row("R34").unwrap().1;
        assert_eq!(right, 34);
        let left = parse_row("L67").unwrap().1;
        assert_eq!(left, -67);
    }

    #[test]
    fn test_sample_v2() {
        let res = read_from_v2("src/input/sample01.txt");
        assert_eq!(res, 6);
    }

    #[test]
    fn test_sample_v2a() {
        let res = read_from_v2("src/input/sample01a.txt");
        assert_eq!(res, 39);
    }

    #[test]
    fn test_sample_v2b() {
        let res = read_from_v2("src/input/sample01b.txt");
        assert_eq!(res, 226); // 200 lines
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 6289);
    }
}