use fancy_regex::Regex;
use lazy_static::lazy_static;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day02.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day02.txt")
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
            // println!("matching {:?} with pattern {}", id, left);
            res.push(id);
        }
    }
    res
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)\1+$").unwrap();
}

fn find_all_invalid_ids((start, end): (i64, i64)) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    for id in start..=end {
        if RE.is_match(id.to_string().as_str()).unwrap() {
            res.push(id)
        }
    }
    res
}

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
    fn test_parse_row2() {
        let splt = "121212".as_bytes()
            .chunks(2)
            .map(|byte_slice| {
                std::str::from_utf8(byte_slice).unwrap()
            })
            .collect::<Vec<&str>>();
        println!("splt {splt:?}");
        let res= splt.into_iter()
            .all(|sub| {
            println!("sub {sub:?}");
            sub == "12"
        });
        assert_eq!(res, true);
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
}
