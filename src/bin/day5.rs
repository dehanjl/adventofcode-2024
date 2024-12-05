use std::cmp::Ordering;

use adventofcode_2024::runner;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules_map = rules
        .lines()
        .map(|line| sscanf::sscanf!(line, "{}|{}", i32, i32).unwrap())
        .into_group_map()
        .iter()
        .map(|(k, v)| (*k, v.into_iter().map(|x| *x).collect::<HashSet<i32>>()))
        .collect();

    let updates_vec = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    (rules_map, updates_vec)
}

fn part1(input: &str) {
    let (rules_map, updates) = parse_input(input);

    let mut valids = vec![];
    'outer: for v in updates.iter() {
        for (i, vv) in v.iter().enumerate() {
            let before_set: HashSet<i32> = v[0..i].into_iter().map(|x| *x).collect();
            if let Some(rule_set) = rules_map.get(vv) {
                if rule_set.intersection(&before_set).count() > 0 {
                    continue 'outer;
                }
            }
        }
        valids.push(v.clone());
    }

    let total: i32 = valids.iter().map(|v| v[v.len() / 2]).sum();

    println!("Day 5 Part 1: {}", total);
}

fn part2(input: &str) {
    let (rules_map, updates) = parse_input(input);

    let mut invalids = vec![];
    'outer: for v in updates.iter() {
        for (i, vv) in v.iter().enumerate() {
            let before_set: HashSet<i32> = v[0..i].into_iter().map(|x| *x).collect();
            if let Some(rule_set) = rules_map.get(vv) {
                if rule_set.intersection(&before_set).count() > 0 {
                    invalids.push(v.clone());
                    continue 'outer;
                }
            }
        }
    }

    let mut new_invalids = vec![];
    for mut v in invalids {
        v.sort_by(|&a, &b| {
            if let Some(rule_set) = rules_map.get(&a) {
                if rule_set.contains(&b) {
                    return Ordering::Less;
                }
            }
            Ordering::Equal
        });
        new_invalids.push(v);
    }

    let total: i32 = new_invalids.iter().map(|v| v[v.len() / 2]).sum();

    println!("Day 5 Part 2: {}", total);
}

fn main() {
    runner(part1);
    runner(part2);
}
