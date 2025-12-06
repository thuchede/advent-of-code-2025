use fancy_regex::Regex;
use lazy_static::lazy_static;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day02.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2_modulo("src/input/day02.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let ranges = sample.iter()
        .map(|line| parse_row(line.as_str()))
        .flatten()
        .map(|pair| find_invalid_ids(pair))
        .flatten()
        .sum();
    ranges
}

fn parse_row(input: &str) -> Vec<(i64, i64)> {
    input
        .split(",")
        .map(|pair| {
            let mut nums = pair.split('-').map(|n| n.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect()
}

fn find_invalid_ids((start, end): (i64, i64)) -> Vec<i64> {
    let mut res = vec![];

    for id in start..=end {
        let id_str = id.to_string();
        if id_str.len() % 2 != 0 {
            continue;
        }
        let (left, right) = id_str.split_at(id_str.len()/2);
        if left == right {
            res.push(id);
        }
    }
    res
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)\1+$").unwrap();
}

#[allow(dead_code)]
fn find_all_invalid_ids((start, end): (i64, i64)) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    for id in start..=end {
        if RE.is_match(id.to_string().as_str()).unwrap() {
            res.push(id)
        }
    }
    res
}

// testing a solution with modulo checks
fn find_repeat_pattern((start, end): (i64, i64)) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    for id in start..=end {
        let digits = id.to_string().len();
        let ok = match digits {
            1 => false,
            2 => id % 11 == 0,
            3 => id % 111 == 0,
            4 => id % 1_111 == 0 || id % 101 == 0,
            5 => id % 11_111 == 0,
            6 => id % 111_111 == 0 || id % 10_101 == 0 || id % 1_001 == 0,
            7 => id % 1_111_111 == 0,
            8 => id % 11_111_111 == 0 || id % 1_010_101 == 0 || id % 10_001 == 0,
            9 => id % 111_111_111 == 0 || id % 1_001_001 == 0,
            10 => id % 1_111_111_111 == 0 || id % 101_010_101 == 0 || id % 100_001 == 0,
            _ => false,
        };
        if ok {
            res.push(id)
        }
    }
    res
}

#[allow(dead_code)]
fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let ranges = sample.iter()
        .map(|line| parse_row(line.as_str()))
        .flatten()
        .map(|pair| find_all_invalid_ids(pair))
        .flatten()
        .sum();
    ranges
}

fn read_from_v2_modulo(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let ranges = sample.iter()
        .map(|line| parse_row(line.as_str()))
        .flatten()
        .map(|pair| find_repeat_pattern(pair))
        .flatten()
        .sum();
    ranges
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 19219508902);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample02.txt");
        assert_eq!(res, 1227775554);
    }

    #[test]
    fn test_parse_row() {
        let res = parse_row("11-22,33-44");
        assert_eq!(res, vec![(11, 22), (33, 44)]);
    }

    #[test]
    fn test_find_invalid_ids() {
        let res = find_invalid_ids((11,1213));
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212]);
    }


    #[test]
    fn test_find_all_invalid_ids() {
        let res = find_all_invalid_ids((11,1213));
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222, 333, 444, 555, 666, 777, 888, 999, 1010, 1111, 1212]);
    }


    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample02.txt");
        assert_eq!(res, 4174379265);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 27180728081);
    }

    #[test]
    fn test_find_match() {
        let res = find_repeat_pattern((11, 1213));
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 111, 222, 333, 444, 555, 666, 777, 888, 999, 1010, 1111, 1212]);
    }

    #[test]
    fn test_read_from_v3() {
        let res = read_from_v2_modulo("src/input/day02.txt");
        assert_eq!(res, 27180728081);
    }
}
