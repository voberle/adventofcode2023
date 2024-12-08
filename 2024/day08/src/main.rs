use std::io::{self, Read};

use fxhash::FxHashSet;
use itertools::Itertools;

struct Grid {
    values: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn build(input: &str) -> Self {
        let mut rows = 0;
        let values: Vec<_> = input
            .lines()
            .flat_map(|l| {
                rows += 1;
                l.chars().collect::<Vec<_>>()
            })
            .collect();
        assert_eq!(values.len() % rows, 0);
        let cols = values.len() / rows;
        Self { values, rows, cols }
    }

    fn print(&self, positions: &[usize]) {
        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";
        for row in 0..self.rows {
            for p in row * self.cols..(row + 1) * self.cols {
                let c = self.values[p];
                if positions.contains(&p) {
                    if c == '.' {
                        print!("{RED}#{RESET}");
                    } else {
                        print!("{RED}{c}{RESET}");
                    }
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    fn pos(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn col(&self, index: usize) -> usize {
        index % self.cols
    }

    fn row(&self, index: usize) -> usize {
        index / self.cols
    }
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
fn antinode_positions(map: &Grid, f1: usize, f2: usize) -> (Option<usize>, Option<usize>) {
    let f1_row = map.row(f1);
    let f2_row = map.row(f2);
    let diff_row = f1_row.abs_diff(f2_row) as isize;
    let min_row = f1_row.min(f2_row);
    let max_row = f1_row.max(f2_row);
    let up_row = ((min_row % map.rows) as isize - diff_row) as usize;
    let down_row = ((max_row % map.rows) as isize + diff_row) as usize;

    let f1_col = map.col(f1);
    let f2_col = map.col(f2);
    let diff_col = f1_col.abs_diff(f2_col) as isize;
    let min_col = f1_col.min(f2_col);
    let max_col = f1_col.max(f2_col);
    let left_col = ((min_col % map.cols) as isize - diff_col) as usize;
    let right_col = ((max_col % map.cols) as isize + diff_col) as usize;

    if f1_col < f2_col {
        (
            if up_row < map.rows && left_col < map.cols {
                Some(map.pos(up_row, left_col))
            } else {
                None
            },
            if down_row < map.rows && right_col < map.cols {
                Some(map.pos(down_row, right_col))
            } else {
                None
            },
        )
    } else {
        (
            if up_row < map.rows && right_col < map.cols {
                Some(map.pos(up_row, right_col))
            } else {
                None
            },
            if down_row < map.rows && left_col < map.cols {
                Some(map.pos(down_row, left_col))
            } else {
                None
            },
        )
    }
}

fn unique_antinode_locations(map: &Grid) -> usize {
    // Find all different frequencies and their occurences count.
    let mut frequencies: FxHashSet<char> = FxHashSet::default();
    for f in map.values.iter().filter(|&&c| c != '.') {
        frequencies.insert(*f);
    }

    // For each, create all pair permutations and get the anti-node positions.
    let mut antinode_locations: FxHashSet<usize> = FxHashSet::default();
    for f in frequencies {
        for pair in map
            .values
            .iter()
            .enumerate()
            .filter_map(|(pos, c)| if *c == f { Some(pos) } else { None })
            .combinations(2)
        {
            let (a1, a2) = antinode_positions(map, pair[0], pair[1]);
            if let Some(a_loc) = a1 {
                antinode_locations.insert(a_loc);
            }
            if let Some(a_loc) = a2 {
                antinode_locations.insert(a_loc);
            }
        }
    }

    map.print(&antinode_locations.iter().copied().collect::<Vec<usize>>());

    antinode_locations.len()
}

fn part2(map: &Grid) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let map = Grid::build(&input);

    println!("Part 1: {}", unique_antinode_locations(&map));
    println!("Part 2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_part1() {
        assert_eq!(unique_antinode_locations(&Grid::build(INPUT_TEST)), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&Grid::build(INPUT_TEST)), 0);
    }
}
