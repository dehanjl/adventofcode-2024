use adventofcode_2024::{runner, utils::Loc};
use grid::Grid;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn parse_input(input: &str) -> (Grid<char>, HashSet<Loc>) {
    let cols = input.lines().next().unwrap().len();
    let chars = input.lines().flat_map(|line| line.chars()).collect_vec();

    let starts = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().filter_map(move |(col, c)| {
                if c == '0' {
                    Some(Loc::from((row, col)))
                } else {
                    None
                }
            })
        })
        .collect();

    (Grid::from_vec(chars, cols), starts)
}

fn part1(input: &str) {
    println!("Day 10 Part 1: {}", "");
}

fn part2(input: &str) {
    println!("Day 10 Part 2: {}", "");
}

fn main() {
    runner(part1);
    runner(part2);
}
