use adventofcode_2024::runner;
use grid::Grid;
use itertools::Itertools;

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input(input: &str) -> Grid<char> {
    let cols = input.lines().next().unwrap().len();
    let chars = input.lines().flat_map(|line| line.chars()).collect_vec();
    Grid::from_vec(chars, cols)
}

fn get_words(grid: &Grid<char>, r: usize, c: usize) -> Vec<String> {
    let mut words = vec![];
    'outer: for (dr, dc) in DIRS.iter() {
        let mut word = String::new();
        word.push(grid[(r, c)]);

        for i in 1..=3 {
            let (ri, ci) = (r as isize + dr * i, c as isize + dc * i);
            if let Some(&ch) = grid.get(ri, ci) {
                word.push(ch);
            } else {
                continue 'outer;
            }
        }
        words.push(word);
    }
    words
}

fn part1(input: &str) {
    let grid = parse_input(input);
    let mut total_xmas = 0;
    for (r, c) in (0..grid.rows()).cartesian_product(0..grid.cols()) {
        if grid[(r, c)] != 'X' {
            continue;
        }
        total_xmas += get_words(&grid, r, c)
            .iter()
            .filter(|word| *word == "XMAS")
            .count();
    }

    println!("Day 1 Part 1: {:?}", total_xmas);
}

fn check_diagonal_sam(chars: [Option<&char>; 3]) -> bool {
    let word = chars.iter().filter_map(|&ch| ch).collect::<String>();
    word == "MAS" || word == "SAM"
}

fn part2(input: &str) {
    let grid = parse_input(input);
    let mut total_cross_mas = 0;

    for (r, c) in (0..grid.rows()).cartesian_product(0..grid.cols()) {
        if grid[(r, c)] != 'A' {
            continue;
        }

        let nw_se = [
            grid.get(r - 1, c - 1),
            grid.get(r, c),
            grid.get(r + 1, c + 1),
        ];

        let ne_sw = [
            grid.get(r - 1, c + 1),
            grid.get(r, c),
            grid.get(r + 1, c - 1),
        ];

        if check_diagonal_sam(nw_se) && check_diagonal_sam(ne_sw) {
            total_cross_mas += 1;
        }
    }

    println!("Day 1 Part 2: {}", total_cross_mas);
}

fn main() {
    runner(part1);
    runner(part2);
}
