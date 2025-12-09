use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day09.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day09.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let coordinates: Vec<(i64, i64)> = sample.iter().map(|s| parse_row(s.as_str())).collect();
    let mut max = 0i64;
    for i in 0..=coordinates.len() - 2 {
        for j in i+1..=coordinates.len() - 1 {
            let &(x1, y1) = &coordinates[i];
            let &(x2, y2) = &coordinates[j];
            let area = ((x2 - x1).abs() +1) * ((y2 - y1).abs()+1);
            // print!("x: ({}, {}), y: ({}, {}), area: {area}\n", x1, y1, x2, y2);
            if area > max {
                max = area;
            }
        }
    }
    max
}

fn parse_row(line: &str) -> (i64, i64) {
    let parts = line.split(",").collect::<Vec<&str>>();
    let x = parts[0].parse::<i64>().unwrap();
    let y = parts[1].parse::<i64>().unwrap();
    (x, y)
}   


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 4746238001);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample09.txt");
        assert_eq!(res, 50);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample09.txt");
        assert_eq!(res, 0);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 0);
    }
}
