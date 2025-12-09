use crate::helpers;
use geo::{Covers, coord, Coord, Intersects, LineString, Polygon, Rect};

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day09.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day09.txt") as i64
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

fn parse_row_v2(line: &str) -> Coord<f64> {
    let parts = line.split(",").collect::<Vec<&str>>();
    let x = parts[0].parse::<f64>().unwrap();
    let y = parts[1].parse::<f64>().unwrap();
    Coord {x, y}
}   


fn read_from_v2(filepath: &str) -> f64 {
    let sample = helpers::read(filepath).unwrap();
    let coordinates: Vec<Coord<f64>>  = sample.iter().map(|s| parse_row_v2(s.as_str())).collect();
    let mut coords: Vec<Coord<f64>> = coordinates.clone();

    // close loop
    if coords.first() != coords.last() {
        if let Some(first) = coords.first().cloned() {
            coords.push(first);
        }
    }
    let line_string = LineString::new(coords);
    let polygon = Polygon::new(line_string, vec![]);
    let mut max = 0f64;
    for i in 0..=coordinates.len() - 2 {
        for j in i+1..=coordinates.len() - 1 {
            let &(x1, y1) = &(coordinates[i].x, coordinates[i].y);
            let &(x2, y2) = &(coordinates[j].x, coordinates[j].y);
            let area = ((x2 - x1).abs() + 1f64) * ((y2 - y1).abs() + 1f64);
            let rect = Rect::new(
                coord! { x: x1 as f64, y: y1 as f64 },
                coord! { x: x2 as f64, y: y2 as f64 },
            );
            if !polygon.intersects(&coord!{x: x1, y: y2}) || !polygon.intersects(&coord!{x: x2, y: y1}) {
                continue;
            }
            let in_polygon = polygon.covers(&rect);
            if in_polygon && area > max  {
                max = area;
            }
        }
    }
    max
}


#[cfg(test)]
mod tests {
    use geo::line_string;
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
    fn test_contains() {
        let line_string = line_string![
            (x: 0., y: 0.),
            (x: 2., y: 0.),
            (x: 2., y: 2.),
            (x: 0., y: 2.),
            (x: 0., y: 0.),
        ];

        let polygon = Polygon::new(line_string.clone(), vec![]);
        assert!(polygon.covers(&coord!(x: 0., y: 1.)));
        assert!(polygon.intersects(&coord!(x: 0., y: 1.)));
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample09.txt");
        assert_eq!(res, 24f64);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 1552139370i64);
    }
}
