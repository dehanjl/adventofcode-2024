use adventofcode_2024::runner;
use dashmap::DashMap;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
    Cat,
}

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(':').unwrap();
            let target = parts.0.parse().unwrap();
            let components = parts
                .1
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (target, components)
        })
        .collect()
}

fn generate_op_matrix_2(n: usize) -> Vec<Vec<Op>> {
    itertools::repeat_n([Op::Add, Op::Mul], n)
        .multi_cartesian_product()
        .collect()
}

fn generate_op_matrix_3(n: usize) -> Vec<Vec<Op>> {
    itertools::repeat_n([Op::Add, Op::Mul, Op::Cat], n)
        .multi_cartesian_product()
        .collect()
}

fn compute(vals: &[u64], ops: &[Op]) -> u64 {
    vals[1..]
        .iter()
        .zip_eq(ops.iter())
        .fold(vals[0], |acc, (val, op)| match op {
            Op::Add => acc + val,
            Op::Mul => acc * val,
            Op::Cat => 10_u64.pow(val.ilog10() + 1) * acc + val,
        })
}

fn part1(input: &str) {
    let total: u64 = parse_input(input)
        .par_iter()
        .filter_map(|(target, components)| {
            let ops = generate_op_matrix_2(components.len() - 1);
            ops.iter().find_map(|op| {
                if compute(components, op) == *target {
                    Some(*target)
                } else {
                    None
                }
            })
        })
        .sum();

    println!("Day 7 Part 1: {}", total);
}

fn part2(input: &str) {
    let cached_ops: DashMap<usize, Vec<Vec<Op>>> = DashMap::new();
    let total: u64 = parse_input(input)
        .par_iter()
        .filter_map(|(target, components)| {
            let ops = cached_ops
                .entry(components.len() - 1)
                .or_insert_with(|| generate_op_matrix_3(components.len() - 1));
            ops.iter().find_map(|op| {
                if compute(components, op) == *target {
                    Some(*target)
                } else {
                    None
                }
            })
        })
        .sum();
    println!("Day 7 Part 2: {}", total);
}

fn main() {
    runner(part1);
    runner(part2);
}
