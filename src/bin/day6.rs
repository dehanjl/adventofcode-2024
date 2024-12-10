use adventofcode_2024::{
    runner,
    utils::{Dir, Loc},
};
use fnv::FnvHashSet;
use grid::Grid;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

trait DirUse {
    fn from_char(ch: char) -> Self;
    fn turn_right(&self) -> Self;
}

impl DirUse for Dir {
    fn from_char(ch: char) -> Self {
        match ch {
            '^' => Dir::North,
            'v' => Dir::South,
            '>' => Dir::East,
            '<' => Dir::West,
            _ => unreachable!(),
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
            _ => unreachable!(),
        }
    }
}

fn cast_ray(grid: &Grid<char>, loc: &Loc, dir: &Dir) -> (Vec<Loc>, bool) {
    // TODO: pre-compute the rays for each direction, or do recursively
    let mut loc = *loc;
    let mut ray = Vec::with_capacity(grid.cols() + 1);
    let mut out_of_bounds = false;
    ray.push(loc);
    loop {
        loc = loc + *dir;
        match grid.get(loc.0, loc.1) {
            Some('#') | Some('O') => break,
            Some(_) => {
                ray.push(loc);
            }
            None => {
                out_of_bounds = true;
                break;
            }
        }
    }
    (ray, out_of_bounds)
}

fn walk(grid: &Grid<char>, starting_pos: &Loc) -> FnvHashSet<Loc> {
    let mut walked_locs =
        FnvHashSet::with_capacity_and_hasher(grid.cols() * grid.rows() / 2, Default::default());
    let (mut loc, mut dir) = (*starting_pos, Dir::from_char(grid[(*starting_pos).into()]));

    loop {
        let (ray, out_of_bounds) = cast_ray(grid, &loc, &dir);

        walked_locs.extend(ray.iter());
        if out_of_bounds {
            break;
        }

        loc = *ray.last().unwrap();
        dir = dir.turn_right();
    }

    walked_locs
}

fn parse_input(input: &str) -> (Grid<char>, Loc) {
    let cols = input.lines().next().unwrap().len();
    let chars = input.lines().flat_map(|line| line.chars()).collect_vec();
    let loc = chars
        .iter()
        .enumerate()
        .find_map(|(i, &ch)| {
            if ch == '^' || ch == 'v' || ch == '<' || ch == '>' {
                Some((i / cols, i % cols))
            } else {
                None
            }
        })
        .unwrap()
        .into();
    (Grid::from_vec(chars, cols), loc)
}

fn part1(input: &str) {
    let (grid, starting_pos) = parse_input(input);
    let walked_locs = walk(&grid, &starting_pos);

    println!("Day 6 Part 1: {}", walked_locs.len());
}

fn part2(input: &str) {
    let (grid, starting_pos) = parse_input(input);
    let mut walked_locs: FnvHashSet<Loc> = walk(&grid, &starting_pos);

    walked_locs.remove(&starting_pos);

    let cycle_count = walked_locs
        .iter()
        .par_bridge()
        .filter_map(|&pos| {
            let mut new_grid = grid.clone();
            new_grid[pos.into()] = 'O';
            let mut corners: FnvHashSet<(Loc, Dir)> = FnvHashSet::with_capacity_and_hasher(
                grid.cols() * grid.rows() / 4,
                Default::default(),
            );
            let mut inner_walked_locs: FnvHashSet<Loc> = FnvHashSet::with_capacity_and_hasher(
                grid.cols() * grid.rows() / 2,
                Default::default(),
            );

            let (mut loc, mut dir) = (starting_pos, Dir::from_char(grid[starting_pos.into()]));

            loop {
                let (ray, out_of_bounds) = cast_ray(&new_grid, &loc, &dir);

                inner_walked_locs.extend(ray.iter());
                if out_of_bounds {
                    break;
                }

                loc = *ray.last().unwrap();
                dir = dir.turn_right();
                // if we were already at this corner, we must be in a loop
                if !corners.insert((loc, dir)) {
                    return Some(());
                }
            }
            None
        })
        .count();

    println!("Day 6 Part 2: {}", cycle_count);
}

fn main() {
    runner(part1);
    runner(part2);
}
