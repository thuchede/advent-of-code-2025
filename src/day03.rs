use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day03.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day03.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    sample.iter()
        .map(|line| parse_row(line.as_str()))
        .map(|v| find_largest(v))
        .sum::<i64>()
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    sample.iter()
        .map(|line| parse_row(line.as_str()))
        .map(|v| find_largest_n_digit(v, 12))
        .sum::<i64>()
}
fn parse_row(line: &str) -> Vec<i64> {
    line
        .chars()
        .map(|c| {
            c.to_digit(10).unwrap() as i64
        })
        .collect()
}

fn find_largest(v: Vec<i64>) -> i64 {
    let max = v.iter().enumerate().rev()
        .max_by_key(|&(_index, value)| value)
        .map(|(index, _value)| index).unwrap();
    if max == v.len() - 1 {
        let second = v.iter().take(v.len() - 1).enumerate().rev()
            .max_by_key(|&(_index, value)| value)
            .map(|(index, _value)| index).unwrap();
        return v.get(second).unwrap() * 10 + v.get(max).unwrap()
    }
    let second = v.iter().skip(max + 1).enumerate().rev()
        .max_by_key(|&(_index, value)| value)
        .map(|(index, _value)| index).unwrap();
    // max + 1 + second to account for the offset of the position since wwe skipped max + 1 items
    v.get(max).unwrap() * 10 + v.get(max + 1 + second).unwrap()
}

fn find_largest_n_digit(v: Vec<i64>, n: usize) -> i64 {
    if n == 1 {
        let max = v.iter().enumerate().rev()
            .max_by_key(|&(_index, value)| value)
            .map(|(index, _value)| index).unwrap();
        return *v.get(max).unwrap();
    }
    // get the largest in v[0..v.len()-n]
    // get the n-1 digit in ...rest
    let size = v.len();
    let slice = v[0..=(size - n)].to_vec();
    let max = slice.iter().enumerate().rev()
        .max_by_key(|&(_index, value)| value)
        .map(|(index, _value)| index).unwrap();
    let res=  *v.get(max).unwrap() * 10i64.pow((n - 1) as u32) + find_largest_n_digit(v.split_at(max+1).1.to_vec(), n - 1);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 17193);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample03.txt");
        assert_eq!(res, 357);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample03.txt");
        assert_eq!(res, 3121910778619);
    }

    #[test]
    fn test_parse_row() {
        let res = parse_row("987654321");
        assert_eq!(res, vec![9,8,7,6,5,4,3,2,1]);
    }

    #[test]
    fn test_find_largest() {
        let res = find_largest(vec![9,8,7,6,5,4,3,2,1]);
        assert_eq!(res, 98);
        let res1 = find_largest(vec![8,7,6,5,4,3,2,1,9]);
        assert_eq!(res1, 89);
        let res2 = find_largest(vec![8,7,6,5,9,3,2,1]);
        assert_eq!(res2, 93);
    }

    #[test]
    fn test_find_largest_n() {
        let res = find_largest_n_digit(vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1], 12);
        assert_eq!(res, 987654321111);
        let res1 = find_largest_n_digit(vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9], 12);
        assert_eq!(res1, 811111111119);
        let res2 = find_largest_n_digit(vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8], 12);
        assert_eq!(res2, 434234234278);
        let res2 = find_largest_n_digit(vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12);
        assert_eq!(res2, 888911112111);
    }

    #[test]
    fn test_find_largest_complete() {
        assert_eq!(find_largest(vec![7,3,6,4,3,2,4,2,4,1,2,2,5,4,3,3,4,4,5,4,2,2,2,3,2,3,2,2,2,3,3,4,3,4,2,2,4,8,3,5,3,2,1,2,5,3,3,3,4,5,3,2,3,3,3,3,2,3,1,6,6,2,2,7,6,7,5,3,2,1,3,2,2,5,3,3,3,3,2,5,2,2,4,2,2,4,4,6,2,3,3,3,2,4,2,3,2,4,3,4]), 87);
        assert_eq!(find_largest(vec![4,4,4,6,3,2,2,8,3,3,4,4,4,6,5,1,6,2,4,6,2,3,3,3,3,4,3,3,5,5,7,4,4,3,3,4,5,3,5,3,5,3,3,3,3,5,1,5,7,3,2,3,3,3,5,5,4,8,6,5,1,3,6,3,3,8,5,6,4,4,7,5,5,5,2,3,6,3,7,3,2,2,4,3,6,4,7,3,3,3,5,3,5,6,3,7,3,3,4,5]), 88);
        assert_eq!(find_largest(vec![5,3,1,3,2,3,4,3,2,1,2,2,2,2,3,1,2,2,3,2,2,3,3,2,2,4,5,1,3,2,3,8,5,3,1,2,2,2,4,4,3,4,3,3,3,3,2,2,3,4,2,2,2,2,3,1,2,3,2,1,3,2,2,3,3,4,1,4,3,2,2,2,1,3,3,7,3,3,2,4,2,3,3,3,3,2,2,3,1,3,8,3,2,3,1,2,2,5,2,2]), 88);
        assert_eq!(find_largest(vec![2,3,2,2,2,2,2,3,2,3,2,2,2,2,2,2,4,2,3,2,1,1,2,3,2,2,3,1,2,1,2,2,6,1,2,2,2,2,2,3,1,2,2,2,3,4,3,2,2,2,5,3,4,2,2,4,2,3,2,3,2,3,3,2,2,2,3,4,4,2,3,2,2,1,2,1,2,3,2,2,4,3,1,2,3,3,6,1,1,2,2,3,2,2,2,2,2,1,2,2]), 66);
        assert_eq!(find_largest(vec![4,3,2,1,1,2,2,2,3,4,4,3,2,4,2,2,2,2,2,1,2,1,2,3,2,2,2,2,2,2,2,2,3,2,2,3,3,6,4,4,4,2,2,5,4,4,1,4,2,2,2,3,3,1,2,2,3,2,2,2,2,4,2,2,3,2,1,2,2,2,2,2,2,3,4,4,2,4,1,3,2,3,2,2,4,2,4,2,4,5,2,1,3,4,1,2,4,3,4,5]), 65);
        assert_eq!(find_largest(vec![5,5,5,5,6,5,5,5,5,6,6,6,2,5,7,6,6,6,5,4,5,6,5,7,4,6,7,5,3,5,6,9,6,9,4,1,8,6,5,5,6,6,6,7,3,5,9,7,5,7,4,6,6,5,2,5,8,7,5,5,7,3,6,5,7,6,7,4,6,5,1,5,6,6,4,5,5,9,7,7,9,7,6,9,6,9,6,3,7,6,7,6,5,6,6,5,9,5,9,5]), 99);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 171297349921310);
    }
}
