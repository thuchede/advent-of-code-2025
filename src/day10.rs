use itertools::Itertools;
use nom::character::complete::{char};
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, tuple};
use good_lp::*;
use crate::helpers;


#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day10.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day10.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let machines: Vec<(u16, Vec<u16>, Vec<u16>)> = sample.iter().map(|line| parse_row(line).unwrap().1).collect();
    let mut total = 0i64;
    for (final_state, button_masks, _) in &machines {
        let mut min_solve = u16::MAX;
        'outer: for i in 0..=button_masks.len() {
            let combinations_iterator = button_masks.iter().combinations(i);
            for combination in combinations_iterator {
                let state = combination.iter().fold(0, |acc, &&mask| acc ^ mask);
                if *final_state == state {
                    min_solve = combination.len() as u16;
                    break 'outer;
                }
            }
        }
        total = total + min_solve as i64;
    }
    total
}

fn parse_indicator(line: &str) -> IResult<&str, u16> {
    let (s, res) =
        delimited(
            char('['),
            many1(alt((
                map(char('.'), |_| 0u16),
                map(char('#'), |_| 1)
            ))),
            char(']'),
        ).parse(line)?;
    let mut bin: u16 = 0;
    for (i, r) in res.iter().enumerate() {
        bin = bin + (r << i)
    }
    Ok((s, bin))
}

fn parse_buttons(line: &str) -> IResult<&str, Vec<u16>> {
    let (s, res) =
        separated_list1(
            char(' '),
            delimited(
                char('('),
                separated_list1(
                    char(','),
                    complete::u16
                ),
                char(')'),
            )
        ).parse(line)?;
    let mut masks: Vec<u16> = vec![];
    for button in &res {
        let mut bin: u16 = 0;
        for b in button {
            bin = bin + (1u16<<b);
        }
        masks.push(bin);
    }
    Ok((s, masks))
}

fn parse_buttons_v2(line: &str) -> IResult<&str, Vec<Vec<u16>>> {
    let (s, res) =
        separated_list1(
            char(' '),
            delimited(
                char('('),
                separated_list1(
                    char(','),
                    complete::u16
                ),
                char(')'),
            )
        ).parse(line)?;
    Ok((s, res))
}

fn parse_joltage(line: &str) -> IResult<&str, Vec<u16>> {
    let (s, res) =
            delimited(
                char('{'),
                separated_list1(
                    char(','),
                    complete::u16
                ),
                char('}'),
        ).parse(line)?;
    Ok((s, res))
}

fn parse_row(line: &str) -> IResult<&str, (u16, Vec<u16>, Vec<u16>)> {
    let (s, res) =
            tuple((
                parse_indicator,
                preceded(char(' '), parse_buttons),
                preceded(char(' '), parse_joltage)
        )).parse(line)?;
    Ok((s, res))
}

fn parse_row_v2(line: &str) -> IResult<&str, (u16, Vec<Vec<u16>>, Vec<u16>)> {
    let (s, res) =
            tuple((
                parse_indicator,
                preceded(char(' '), parse_buttons_v2),
                preceded(char(' '), parse_joltage)
        )).parse(line)?;
    Ok((s, res))
}


fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let machines: Vec<(u16, Vec<Vec<u16>>, Vec<u16>)> = sample.iter().map(|line| parse_row_v2(line).unwrap().1).collect();

    let total = machines.iter().map(|(_, buttons, joltages)| {
        let mut variables = ProblemVariables::new();
        let mut press_count = Vec::new();
        for _ in 0..buttons.len() {
            let variable = variables.add(variable().min(0).integer());
            press_count.push(variable);
        }
        let mut problem = good_lp::highs(variables.minimise(press_count.iter().sum::<Expression>()));

        let mut expressions = vec![Expression::with_capacity(buttons.len()); joltages.len()];

        for i in 0..buttons.len() {
            for x in buttons[i].iter() {
                expressions[*x as usize] += press_count[i];
            }
        }
        for (e, j) in expressions.into_iter().zip(joltages.clone()) {
            problem.add_constraint(e.eq(j as f64));
        }
        let solution = problem.solve().unwrap();
        let sol = press_count.iter().map(|&v| {
            solution.value(v).round()
        }).sum::<f64>() as i64;
        sol
    }).sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 500);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample10.txt");
        assert_eq!(res, 7);
    }

    #[test]
    fn test_shift() {
        let res = (0 << 1) + 1;
        assert_eq!(res, 1);
        let res1 = (res << 1) + 1;
        assert_eq!(res1, 3);
    }

    #[test]
    fn test_parse_indicator() {
        let res = parse_indicator("[#]").unwrap();
        assert_eq!(res.1, 0b1);
        assert_eq!(res.1, 1);
        let res1 = parse_indicator("[.#]").unwrap();
        assert_eq!(res1.1, 2);
        let res2 = parse_indicator("[#...#]").unwrap();
        assert_eq!(res2.1, 17);
    }

    #[test]
    fn test_parse_buttons() {
        let res = parse_buttons("(0,1,3) (0,2)").unwrap();
        assert_eq!(res.1[0], 0b1011);
        assert_eq!(res.1[1], 0b101);
        let res1 = parse_buttons("(1,4)").unwrap();
        assert_eq!(res1.1[0], 0b10010);
    }

    #[test]
    fn test_parse_buttons_v2() {
        let res = parse_buttons_v2("(0,1,3) (0,2)").unwrap();
        assert_eq!(res.1, vec![vec![0u16,1,3], vec![0,2]]);
    }

    #[test]
    fn test_parse_joltage() {
        let res = parse_joltage("{14,1,13,1}").unwrap();
        assert_eq!(res.1, vec![14, 1, 13,1]);
    }

    #[test]
    fn test_parse_row() {
        let res = parse_row("[##.#] (0,1,3) (0,2) {14,1,13,1}").unwrap();
        assert_eq!(res.1, (11, vec![11,5], vec![14, 1, 13,1]));
        let res1 = parse_row("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}").unwrap();
        assert_eq!(res1.1, (8, vec![0b11101, 0b1100, 0b10001, 0b111, 0b11110], vec![7, 5, 12, 7, 2]));
    }
    
    #[test]
    fn test_sanity_xor() {
        assert_eq!(0b1000, 8);
        assert_eq!(0b1000, 0 ^ 0b111 ^ 0b1111);
        assert_eq!(0b110, 0 ^ 5 ^ 3);
        assert_eq!(0b110, 0 ^ 0b101 ^ 0b11);
        assert_eq!(0b1000, 0 ^ 0b10001 ^ 0b111 ^ 0b11110);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample10.txt");
        assert_eq!(res, 33);
    }

    #[test]
    fn test_read_from_v2a() {
        let res = read_from_v2("src/input/sample10a.txt");
        assert_eq!(res, 287);
    }

    #[test]
    fn test_read_from_v2b() {
        let res = read_from_v2("src/input/sample10b.txt");
        assert_eq!(res, 231);
    }

    #[test]
    fn test_read_from_v2c() {
        let res = read_from_v2("src/input/sample10c.txt");
        assert_eq!(res, 115);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 19763);
    }
}
