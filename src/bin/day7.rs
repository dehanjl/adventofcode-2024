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

impl Op {
    fn invoke(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Cat => 10_u64.pow(b.ilog10() + 1) * a + b,
        }
    }
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
        .fold(vals[0], |acc, (val, op)| op.invoke(acc, *val))
}

fn compute_recurse(vals: &[u64], op_options: &[Op], target: u64) -> Option<u64> {
    fn recurse(acc: u64, vals: &[u64], op: Op, op_options: &[Op], target: u64) -> Option<u64> {
        if acc > target {
            None
        } else if vals.is_empty() {
            (acc == target).then_some(acc)
        } else {
            op_options.iter().find_map(|op_inner| {
                recurse(
                    op.invoke(acc, vals[0]),
                    &vals[1..],
                    *op_inner,
                    op_options,
                    target,
                )
            })
        }
    }

    op_options
        .iter()
        .find_map(|op| recurse(vals[0], &vals[1..], *op, op_options, target))
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

fn part1_recurse(input: &str) {
    let total: u64 = parse_input(input)
        .par_iter()
        .filter_map(|(target, components)| {
            compute_recurse(components, &[Op::Add, Op::Mul], *target)
        })
        .sum();

    println!("(recursive) Day 7 Part 1: {}", total);
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

fn part2_recurse(input: &str) {
    let total: u64 = parse_input(input)
        .par_iter()
        .filter_map(|(target, components)| {
            compute_recurse(components, &[Op::Add, Op::Mul, Op::Cat], *target)
        })
        .sum();

    println!("(recursive) Day 7 Part 2: {}", total);
}

fn main() {
    runner(part1);
    runner(part1_recurse);
    runner(part2);
    runner(part2_recurse);
}
