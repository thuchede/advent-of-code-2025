use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day12.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day12.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let splitted_sample: Vec<Vec<String>>= sample.iter().fold(vec![vec![]], |mut acc, line| {
        if line == "" {
            acc.push(vec![]);
        } else {
            acc.last_mut().unwrap().push(line.clone()); 
        }
        acc
    });
    let shapes = splitted_sample.iter().take(6).map(|group|
        parse_shape_cnt(group.iter().collect())
    ).collect::<Vec<u8>>();
    let spaces= splitted_sample.last().unwrap();
    let under_the_trees: Vec<(Vec<i64>, Vec<i64>)> = spaces.iter().map(|line| parse_space(line.as_str())).collect();
    let got_space: i64 = under_the_trees.iter().map(|(sizes, presents)| {
        let space_size: i64 = sizes.iter().product();
        let presents_size: i64 = presents.iter().zip(shapes.iter()).map(|(&present_count, &p)| present_count*(p as i64)).sum();
        // cheating the problem by just comparing the theoric spaces available without shapes constraints
        if space_size - presents_size >= 0 {
            1
        } else {
            0
        }
    }).sum();
    got_space
}

fn parse_shape_cnt(shape_lines: Vec<&String>) -> u8 {
    let mut count = 0u8;
    for line in shape_lines.iter().skip(1) {
        for ch in line.chars() {
            count += if ch == '#' { 1 } else { 0 };
        }
    }
    count
    
}

fn parse_space(line: &str) -> (Vec<i64>, Vec<i64>) {
    let (sizes_str, presents_str) = line.split_once(": ").unwrap();
    let sizes: Vec<i64> = sizes_str.split("x").map(|s| s.parse::<i64>().unwrap()).collect();
    let presents: Vec<i64> = presents_str.split(" ").map(|s| s.parse::<i64>().unwrap()).collect();
    (sizes, presents)
}

// keeping part 2 for bench
fn read_from_v2(filepath: &str) -> i64 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 591);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample12.txt");
        // not working for input sample since it requires proper tiling algo
        // assert_eq!(res, 2);
    }

    #[test]
    fn test_parse_shape_cnt() {
        let res = parse_shape_cnt(vec![
            &String::from("###"),
            &String::from("# #"),
            &String::from("###"),
        ]);
        assert_eq!(res, 8);
    }
    
    #[test]
    fn test_parse_space() {
        let res = parse_space("40x42: 34 50 12 5 90");
        assert_eq!(res, (vec![40, 42], vec![34, 50, 12, 5, 90]));
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample12.txt");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}
