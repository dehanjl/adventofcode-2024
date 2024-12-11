use adventofcode_2024::runner;
use fnv::FnvHashMap;
use itertools::Itertools;

type StoneId = u64;
type Count = usize;

trait IntUtils {
    fn n_digits(self) -> u32;
    fn split_half(self) -> (Self, Self)
    where
        Self: std::marker::Sized;
}

impl IntUtils for StoneId {
    fn n_digits(self) -> u32 {
        self.ilog10() + 1
    }

    fn split_half(self) -> (Self, Self) {
        assert!(self.n_digits() % 2 == 0);
        let mask = 10u64.pow(self.n_digits() / 2);
        (self / mask, self % mask)
    }
}

fn parse_input(input: &str) -> FnvHashMap<StoneId, Count> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .counts()
        .into_iter()
        .collect()
}

fn apply_rules(stone: StoneId) -> (StoneId, Option<StoneId>) {
    match stone {
        0 => (1, None),
        val if stone.n_digits() % 2 == 0 => (val.split_half().0, Some(val.split_half().1)),
        _ => (stone * 2024, None),
    }
}

fn compute(n_steps: usize, mut stones: FnvHashMap<StoneId, Count>) -> Count {
    let mut new_stones = FnvHashMap::default();
    for _ in 0..n_steps {
        new_stones.clear();
        // reserve capacity to avoid reallocations
        new_stones.reserve(stones.len() * 2);

        for (&stone, &count) in &stones {
            let (a, b) = apply_rules(stone);
            *new_stones.entry(a).or_default() += count;
            if let Some(b) = b {
                *new_stones.entry(b).or_default() += count;
            }
        }

        // swap so that `stones` becomes the new map without allocating again
        std::mem::swap(&mut stones, &mut new_stones);
    }

    stones.values().sum()
}

fn part1(input: &str) {
    let total = compute(25, parse_input(input));
    println!("Day 11 Part 1: {}", total);
}

fn part2(input: &str) {
    let total = compute(75, parse_input(input));
    println!("Day 11 Part 2: {}", total);
}

fn main() {
    runner(part1);
    runner(part2);
}
