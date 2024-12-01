use adventofcode_2024::runner;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn part1(input: &str) {
    let (l, r) = parse_input(input);

    let total_distance = l
        .into_iter()
        .sorted()
        .zip(r.into_iter().sorted())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Day 1 Part 1: {}", total_distance);
}

fn part2(input: &str) {
    let (l, r) = parse_input(input);
    let r_counts = r.into_iter().counts();

    let similarity_score: i32 = l
        .into_iter()
        .filter_map(|x| r_counts.get(&x).map(|&count| count as i32 * x))
        .sum();

    println!("Day 1 Part 2: {}", similarity_score);
}

fn main() {
    runner(part1);
    runner(part2);
}
