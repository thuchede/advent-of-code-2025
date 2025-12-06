use crate::helpers;

#[allow(dead_code)]
pub fn part_1() -> i64 {
    read_from("src/input/day06.txt")
}

#[allow(dead_code)]
pub fn part_2() -> i64 {
    read_from_v2("src/input/day06.txt")
}


fn read_from(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (last, rest) = sample.split_last().unwrap();
    let operators = parse_operator(last.as_str());
    let numbers = rest.iter().map(|line| parse(line)).collect::<Vec<Vec<i64>>>();
    let mut total = 0i64;
    for cols in 0..numbers[0].len() {
        let operator = &operators[cols];
        let mut col_total = if *operator == Operator::Add { 0i64 } else { 1i64 };
        for rows in 0..numbers.len() {
            col_total = match operator {
                Operator::Add => col_total + numbers[rows][cols],
                Operator::Multiply => col_total * numbers[rows][cols],
            }
        }
        total = total + col_total;
    }
    total
}

fn parse(line: &str) -> Vec<i64> {
    line
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Operator {
    Add,
    Multiply
}

fn parse_operator(line: &str) -> Vec<Operator> {
    line
        .split_whitespace()
        .map(|op| if op == "+" { Operator::Add } else { Operator::Multiply })
        .collect()
}

fn read_from_v2(filepath: &str) -> i64 {
    let sample = helpers::read(filepath).unwrap();
    let (last, rest) = sample.split_last().unwrap();
    let operators = parse_operator(last);
    let mut total = 0i64;
    let mut sheet: Vec<Vec<i64>> = vec![];
    let mut numbers: Vec<i64> = vec![];
    for cols in 0..rest[0].len() {
        let mut current_num = 0i64;
        for rows in 0..rest.len() {
            let digit = rest[rows].chars().nth(cols).unwrap();
            if digit == ' ' {
                continue;
            }
            let digit_value = digit.to_digit(10).unwrap() as i64;
            current_num = current_num * 10 + digit_value;
        }
        if current_num == 0 {
            sheet.push(numbers);
            numbers = vec![];
            continue;
        }
        numbers.push(current_num);
    }
    sheet.push(numbers);

    for (i, rows) in sheet.iter().enumerate() {
        match operators[i] {
            Operator::Add => {
                let sum: i64 = rows.iter().sum();
                total = total + sum;
            },
            Operator::Multiply => {
                let product: i64 = rows.iter().product();
                total = total + product;
            },
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = part_1();
        assert_eq!(res, 6605396225322);
    }

    #[test]
    fn test_read_from() {
        let res = read_from("src/input/sample06.txt");
        assert_eq!(res, 4277556);
    }

    #[test]
    fn test_parse() {
        let res = parse("5   567 13");
        assert_eq!(res, vec![5, 567, 13]);
    }

    #[test]
    fn test_parse_operator() {
        let res = parse_operator("*   + * ");
        assert_eq!(res, vec![Operator::Multiply, Operator::Add, Operator::Multiply]);
    }

    #[test]
    fn test_read_from_v2() {
        let res = read_from_v2("src/input/sample06.txt");
        assert_eq!(res, 3263827);
    }

    #[test]
    fn test_part_2() {
        let res = part_2();
        assert_eq!(res, 11052310600986);
    }
}
