use std::collections::HashMap;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day11.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day11.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let connections = sample.iter().map(|line| parse_row(line.as_str())).collect::<Vec<(&str, Vec<&str>)>>();
    let hash_map: std::collections::HashMap<_, _> = connections.iter().cloned().collect();
    let res = traverse("you", &hash_map);
    res
}

fn traverse(start_node: &str, hash_map: &std::collections::HashMap<&str, Vec<&str>>) -> i64 {
    if start_node == "out" {
        return 1;
    }
    hash_map.get(start_node).unwrap().iter().map(|&node| traverse(node, hash_map)).sum()
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample: Vec<String> = helpers::read(filepath).unwrap();
    let connections = sample.iter().map(| line| parse_row(line.as_str())).collect::<Vec<(&str, Vec<&str>)>>();
    let hash_map: HashMap<_, _> = connections.iter().cloned().collect();
    let mut memo: HashMap<(String, u8), i64> = HashMap::new();
    let res = traverse_v2("svr", &hash_map, 0b0, &mut memo);
    res
}

fn traverse_v2(start_node: & str, hash_map: &HashMap<&str, Vec<&str>>, dac_fft: u8, memo: &mut HashMap<(String, u8), i64>) -> i64 {
    if start_node == "out" {
        let res = if dac_fft == 0b11 { 1 } else { 0 };
        memo.insert((start_node.to_string(), dac_fft), res);
        return res;
    }
    let flag = if start_node == "dac" { dac_fft | 0b10 } else if start_node == "fft" { dac_fft | 0b01 } else { dac_fft };
    if memo.get(&(start_node.to_string(), flag)).is_some() {
        return *memo.get(&(start_node.to_string(), flag)).unwrap();
    }
    let res = hash_map.get(start_node).unwrap().iter().map(|&node| traverse_v2(node, hash_map, flag, memo)).sum();
    memo.insert((start_node.to_string(), flag), res);
    res
}

fn parse_row(line: &str) -> (&str, Vec<&str>) {
    let (input, output) = line.split_once(": ").unwrap();
    let output_list = output.split(" ").collect::<Vec<&str>>();
    (input, output_list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 585);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample11.txt");
        assert_eq!(res, 5);
    }

    #[test]
    fn test_parse_row() {
        let res = parse_row("fxp: udl vii hgb qmy");
        assert_eq!(res.0, "fxp");
        assert_eq!(res.1, vec!["udl","vii","hgb","qmy" ]);
    }

    #[test]
    fn test_or() {
        assert_eq!(0b10, 0b00 | 0b10);
        assert_eq!(0b10, 0b10 | 0b10);
        assert_eq!(0b11, 0b01 | 0b10);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample11a.txt");
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 349322478796032);
    }
}
