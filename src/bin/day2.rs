use adventofcode_2024::runner;
use itertools::{any, Itertools};

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn safe_check(line: &[i32]) -> bool {
    let diffs = line
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect_vec();

    diffs.iter().all(|&x| (1..=3).contains(&x)) || diffs.iter().all(|&x| (-3..=-1).contains(&x))
}

fn part1(input: &str) {
    let safe_count = parse_input(input)
        .iter()
        .filter(|line| safe_check(line))
        .count();

    println!("Day 1 Part 1: {}", safe_count);
}

fn part2(input: &str) {
    let safe_count = parse_input(input)
        .iter()
        .filter(|line| {
            safe_check(line)
                || any(
                    (0..line.len()).map(|i| [&line[..i], &line[i + 1..]].concat()),
                    |iter| safe_check(&iter),
                )
        })
        .count();

    println!("Day 1 Part 2: {}", safe_count);
}

fn main() {
    runner(part1);
    runner(part2);
}
