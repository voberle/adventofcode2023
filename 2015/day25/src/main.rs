use std::io::{self, Read};

use regex::Regex;

// Coord systems starts at 1,1.
#[derive(Debug, PartialEq)]
struct Coords {
    row: usize,
    col: usize,
}

impl Coords {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn next(&self) -> Coords {
        if self.row > 1 {
            Coords {
                row: self.row - 1,
                col: self.col + 1,
            }
        } else {
            Coords {
                row: self.col + 1,
                col: 1,
            }
        }
    }
}

fn build(input: &str) -> Coords {
    let re = Regex::new(r"To continue, please consult the code grid in the manual.  Enter the code at row (\d+), column (\d+).").unwrap();
    let parts = re.captures(input).unwrap();
    Coords::new(parts[1].parse().unwrap(), parts[2].parse().unwrap())
}

// After that, each code is generated by taking the previous one, multiplying it by 252533,
// and then keeping the remainder from dividing that value by 33554393.
#[allow(clippy::unreadable_literal)]
fn next_code(c: u64) -> u64 {
    c * 252533 % 33554393
}

#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
const SCRAP: [[u64; 6]; 6] = [
    [ 20151125, 18749137, 17289845, 30943339, 10071777, 33511524 ],
    [ 31916031, 21629792, 16929656,  7726640, 15514188,  4041754 ],
    [ 16080970,  8057251,  1601130,  7981243, 11661866, 16474243 ],
    [ 24592653, 32451966, 21345942,  9380097, 10600672, 31527494 ],
    [    77061, 17552253, 28094349,  6899651,  9250759, 31663883 ],
    [ 33071741,  6796745, 25397450, 24659492,  1534922, 27995004 ],
];

fn get_code_from_machine(target: &Coords) -> u64 {
    let mut pos = Coords::new(1, 1);
    let mut code = SCRAP[0][0];
    while pos != *target {
        code = next_code(code);
        pos = pos.next();
    }
    code
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let coords = build(&input);

    println!("Part 1: {}", get_code_from_machine(&coords));
    // No part 2 to do.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_next() {
        assert_eq!(Coords::new(1, 1).next(), Coords::new(2, 1));
        assert_eq!(Coords::new(2, 1).next(), Coords::new(1, 2));
        assert_eq!(Coords::new(2, 2).next(), Coords::new(1, 3));
    }

    #[test]
    fn test_next_code() {
        assert_eq!(next_code(20151125), 31916031);
    }
}
