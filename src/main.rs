use std::time::{Instant};
use std::vec;
use std::env;
use std::process::exit;

mod day01;
mod helpers;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let run_all = args.get(1).unwrap() ==  "--all";
        if run_all {
            bench();
        }
        exit(0);
    }
    // else run just today's
    let time_for_part1 = Instant::now();
    let part1 = day11::part_1();
    let time1 = time_for_part1.elapsed().as_micros();
    println!("{}", part1);
    println!("Done in {}µs", time1);
    let time_for_part2 = Instant::now();
    let part2 = day11::part_2();
    let time2 = time_for_part2.elapsed().as_micros();
    println!("{}", part2);
    println!("Done in {}µs", time2);
}

// TODO: run this when an arg is passed + format output as a table
#[allow(dead_code)]
fn bench() {
    let v: Vec<(&str, fn()->i64, fn()->i64)> = vec![
        ("day01", day01::part_1, day01::part_2),
        ("day02", day02::part_1, day02::part_2),
        ("day03", day03::part_1, day03::part_2),
        ("day04", day04::part_1, day04::part_2),
        ("day05", day05::part_1, day05::part_2),
        ("day06", day06::part_1, day06::part_2),
        ("day07", day07::part_1, day07::part_2),
        ("day08", day08::part_1, day08::part_2),
        ("day09", day09::part_1, day09::part_2),
        ("day10", day10::part_1, day10::part_2),
        ("day11", day11::part_1, day11::part_2),
        ("day12", day12::part_1, day12::part_2),
    ];
    println!("| day## | part1 | part2 |");
    println!("|:-----:|:-----:|:-----:|");
    for (d, p1, p2) in v {
        let time_for_part1 = Instant::now();
        let _ = p1();
        let time1 = time_for_part1.elapsed().as_millis();
        let formatted_time1 = if time1 == 0 { "<1".to_string() } else { time1.to_string() };
        let time_for_part2 = Instant::now();
        let _ = p2();
        let time2 = time_for_part2.elapsed().as_millis();
        let formatted_time2 = if time2 == 0 { "<1".to_string() } else { time2.to_string() };
        println!("| {d} | {}ms | {}ms |", formatted_time1, formatted_time2);
    }
}