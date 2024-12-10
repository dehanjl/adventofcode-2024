use std::collections::VecDeque;

use adventofcode_2024::{
    runner,
    utils::{GridUtils, Loc, DIR4},
};
use grid::Grid;
use hashbrown::HashSet;
use itertools::Itertools;

fn parse_input(input: &str) -> (Grid<char>, HashSet<Loc>) {
    let grid = Grid::parse(input);
    let trailheads = grid.find_set(|&c| c == '0');

    (grid, trailheads)
}

fn part1(input: &str) {
    let (map, trailheads) = parse_input(input);

    let total_score: i32 = trailheads
        .iter()
        .map(|start| {
            let mut score = 0;
            let mut queue: VecDeque<Loc> = VecDeque::from([(*start)]);
            while let Some(loc) = queue.pop_front() {
                if let Some(&'9') = map.get(loc.0, loc.1) {
                    score += 1;
                }

                let next_locs = DIR4
                    .iter()
                    .map(|dir| loc + *dir)
                    .filter(|&new_loc| {
                        new_loc.in_bounds(map.size())
                            && (map[new_loc.into()] as u8 - map[loc.into()] as u8) == 1
                            && !queue.contains(&new_loc)
                    })
                    .collect_vec();

                queue.extend(next_locs);
            }
            score
        })
        .sum();

    println!("Day 10 Part 1: {}", total_score);
}

fn part2(input: &str) {
    let (map, trailheads) = parse_input(input);

    let total_score: i32 = trailheads
        .iter()
        .map(|start| {
            let mut score = 0;
            let mut queue: VecDeque<Loc> = VecDeque::from([(*start)]);
            while let Some(loc) = queue.pop_front() {
                if let Some(&'9') = map.get(loc.0, loc.1) {
                    score += 1;
                }

                queue.extend(DIR4.iter().map(|dir| loc + *dir).filter(|&new_loc| {
                    new_loc.in_bounds(map.size())
                        && (map[new_loc.into()] as u8 - map[loc.into()] as u8) == 1
                }));
            }
            score
        })
        .sum();

    println!("Day 10 Part 2: {}", total_score);
}

fn main() {
    runner(part1);
    runner(part2);
}
