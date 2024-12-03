use adventofcode_2024::runner;
use regex::Regex;

fn part1(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total = re
        .captures_iter(input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .fold(0, |acc, (l, r)| acc + l * r);

    println!("Day 1 Part 1: {}", total);
}

fn part2(input: &str) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't|do").unwrap();
    let mut enabled = true;
    let total = re
        .captures_iter(input)
        .filter_map(|cap| match (&cap[0], enabled) {
            ("don't", _) => {
                enabled = false;
                None
            }
            ("do", _) => {
                enabled = true;
                None
            }
            (_, true) => Some((
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )),
            _ => None,
        })
        .fold(0, |acc, (l, r)| acc + l * r);

    println!("Day 1 Part 2: {}", total);
}

fn main() {
    runner(part1);
    runner(part2);
}
