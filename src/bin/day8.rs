use std::iter::successors;

use adventofcode_2024::{runner, utils::Loc};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<char, Vec<Loc>>, (usize, usize)) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, ch)| {
                if ch.is_ascii_alphanumeric() {
                    Some((ch, Loc(i as isize, j as isize)))
                } else {
                    None
                }
            })
        })
        .into_group_map()
        .into_iter()
        .collect();

    (antennas, (rows, cols))
}

fn part1(input: &str) {
    let (antennas, bounds) = parse_input(input);

    let mut antinodes: HashSet<Loc> = HashSet::new();
    antennas.iter().for_each(|(_, nodes)| {
        nodes.iter().tuple_combinations().for_each(|(&a, &b)| {
            let delta = b - a;
            if (a - delta).in_bounds(bounds) {
                antinodes.insert(a - delta);
            }
            if (b + delta).in_bounds(bounds) {
                antinodes.insert(b + delta);
            }
        });
    });

    println!("Day 8 Part 1: {}", antinodes.len());
}

fn part2(input: &str) {
    let (antennas, bounds) = parse_input(input);

    let mut antinodes: HashSet<Loc> = HashSet::new();
    antennas.iter().for_each(|(_, nodes)| {
        nodes.iter().tuple_combinations().for_each(|(&a, &b)| {
            let delta = b - a;
            antinodes.extend(
                successors(Some(a), |&l| Some(l - delta)).take_while(|&l| l.in_bounds(bounds)),
            );
            antinodes.extend(
                successors(Some(b), |&l| Some(l + delta)).take_while(|&l| l.in_bounds(bounds)),
            );
        });
    });

    println!("Day 8 Part 2: {}", antinodes.len());
}

fn main() {
    runner(part1);
    runner(part2);
}
