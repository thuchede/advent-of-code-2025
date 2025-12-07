use std::collections::{HashMap};
use itertools::Itertools;
use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day07.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day07.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (first, rest) = sample.split_first().unwrap();
    let start_beam = parse_start(first);
    let splitters_row: Vec<Vec<(i64, i64)>> = rest.iter().enumerate().map(|(row, line)| parse_splitters(line, (row + 1) as i64)).collect();
    let mut current_beams = vec![start_beam];
    let mut total_splits = 0i64;
    for splitters in splitters_row.iter() {
        let (split_count, new_beams) = split_beams(&current_beams, &splitters);
        current_beams = new_beams;
        total_splits = total_splits + split_count;
    }
    total_splits
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (first, rest) = sample.split_first().unwrap();
    let start_beam = parse_start(first);
    let splitters_row: Vec<Vec<(i64, i64)>> = rest.iter().enumerate().map(|(row, line)| parse_splitters(line, (row + 1) as i64)).collect();
    let mut current_beams = HashMap::from([(start_beam, 1i64)]);
    for splitters in splitters_row.iter() {
        let new_beams = count_universe(&current_beams, &splitters);
        current_beams = new_beams;
    }
    current_beams.values().sum()
}

fn parse_start(line: &str) -> (i64, i64) {
    let (pos, _) = line.chars().find_position(|&c| c == 'S').unwrap();
    (0, pos as i64)
}

fn parse_splitters(line: &str, row: i64) -> Vec<(i64, i64)> {
    let positions: Vec<(i64, i64)> = line.chars().enumerate().filter_map(|(i, c)| if c == '^' {Some((row ,i as i64))} else {None}).collect::<Vec<(i64, i64)>>();
    positions
}

fn split_beams(beams: &Vec<(i64, i64)>, splitters: &Vec<(i64, i64)>) -> (i64, Vec<(i64, i64)>) {
    let mut beams_split_count: i64= 0;
    let mut new_beams: Vec<(i64,i64)> = vec![];
    for beam in beams.iter() {
        let mut split_occurred = false;
        for splitter in splitters.iter() {
            if beam.0 + 1 == splitter.0 && beam.1 == splitter.1 {
                split_occurred = true;
                beams_split_count += 1;
                if !new_beams.contains(&(beam.0 + 1, beam.1 - 1)) {
                    new_beams.push((beam.0 + 1, beam.1 - 1)); // Left beam
                }
                if !new_beams.contains(&(beam.0 + 1, beam.1 + 1)) {
                    new_beams.push((beam.0 + 1, beam.1 + 1)); // Right beam
                    break;
                }
            }
        }
        if !split_occurred && !new_beams.contains(&(beam.0 + 1, beam.1)) {
            new_beams.push((beam.0 + 1, beam.1)); // Continue straight
        }
    }
    (beams_split_count, new_beams)
}


fn count_universe(beams: &HashMap<(i64,i64), i64>, splitters: &Vec<(i64, i64)>) -> HashMap<(i64,i64), i64> {
    let mut new_beams: HashMap<(i64,i64), i64> = HashMap::new();
    for (beam, past) in beams.iter() {
        let mut split_occurred = false;
        for splitter in splitters.iter() {
            if beam.0 + 1 == splitter.0 && beam.1 == splitter.1 {
                split_occurred = true;
                if let Some(current_past) = new_beams.get_mut(&(beam.0 + 1, beam.1 - 1)) {
                    *current_past =  *current_past + past;
                } else {
                    new_beams.insert((beam.0 + 1, beam.1 - 1), *past);
                }
                if let Some(current_past) = new_beams.get_mut(&(beam.0 + 1, beam.1 + 1)) {
                    *current_past =  *current_past + past;
                } else {
                    new_beams.insert((beam.0 + 1, beam.1 + 1), *past);
                }
            }
        }
        if !split_occurred {
            if let Some(current_past) = new_beams.get_mut(&(beam.0 + 1, beam.1)) {
                *current_past =  *current_past + past;
            } else {
                new_beams.insert((beam.0 + 1, beam.1), *past);
            }
        }
    }
    new_beams
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 1640);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample07.txt");
        assert_eq!(res, 21);
    }

    #[test]
    fn test_parse_splitters() {
        let res = parse_splitters(".^....^.^......", 4);
        assert_eq!(res, vec![(4, 1), (4, 6), (4, 8)]);
    }
    
    #[test]
    fn test_parse_start() {
        let res = parse_start(".......S.......");
        assert_eq!(res, (0, 7));
    }
    
    #[test]
    fn test_split_beams() {
        let res = split_beams(
            &vec![(3,6),(3,8)], &vec![(4, 1), (4, 6), (4, 8)]);
        assert_eq!(res.0, 2);
        assert_eq!(res.1, vec![(4, 5), (4, 7), (4, 9)]);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample07.txt");
        assert_eq!(res, 40);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 40999072541589);
    }
}
