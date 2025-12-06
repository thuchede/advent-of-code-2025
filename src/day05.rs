use itertools::{Itertools};
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day05.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day05.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let mut iter = sample.split(|s| s == "");
    let ranges = iter.next().unwrap().to_vec().iter().map(|r| parse_range(r)).collect::<Vec<(i64, i64)>>();
    let products = iter.next().unwrap().to_vec().iter().map(|p| p.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut count = 0i64;
    for product in products {
        for range in ranges.iter() {
            let (start, end) = range;
            if product >= *start && product <= *end {
                count = count + 1;
                break;
            }
        }
    }
    count
}

fn parse_range(line: &str) -> (i64, i64) {
    let mut parts = line.split('-');
    let start = parts.next().unwrap().parse::<i64>().unwrap();
    let end = parts.next().unwrap().parse::<i64>().unwrap();
    (start, end)
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let mut iter = sample.split(|s| s == "");
    let ranges = iter.next().unwrap().to_vec().iter().map(|r| parse_range(r)).collect::<Vec<(i64, i64)>>();
    let mut res = optimise_ranges(ranges);

    while res.0 != 0 {
        res = optimise_ranges(res.1);
    }
    let optimized_range = res.1;

    optimized_range.iter().sorted_by(|(s, _e), (d, _f)| s.partial_cmp(d).unwrap()).fold(0i64, |sum, (s,e)| {
        sum + (e-s+1)
    })

}

fn optimise_ranges(all_ranges: Vec<(i64, i64)>) -> (i64, Vec<(i64, i64)>){
    let mut count = 0i64;
    let mut optimized_range: Vec<(i64, i64)> = vec![];
    'outer: for (s,e) in all_ranges {
        for (opt_s, opt_e) in optimized_range.iter_mut() {
            if (s <=*opt_e && e >= *opt_s) || (s <= *opt_s && e >= *opt_s) || (s <= *opt_e && e >= *opt_e) {
                if s == *opt_s && e ==*opt_e {
                    continue 'outer;
                }
                *opt_s = std::cmp::min(*opt_s, s);
                *opt_e = std::cmp::max(*opt_e, e);
                count = count + 1;
                continue 'outer;
            }
        }
        optimized_range.push((s, e));
    }
    (count, optimized_range)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 737);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample05.txt");
        assert_eq!(res, 3);
    }

    #[test]
    fn test_parse_range() {
        let res = parse_range("10-20");
        assert_eq!(res, (10, 20));
    }

    #[test]
    fn test_read_from_v2_new() {
        let res = read_from_v2("src/input/sample05.txt");
        assert_eq!(res, 14);
    }

    #[test]
    fn test_read_from_v2_new_a() {
        let res = read_from_v2("src/input/sample05a.txt");
        assert_eq!(res, 215);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 357485433193284);
    }
}
