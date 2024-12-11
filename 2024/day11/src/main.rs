use std::io::{self, Read};

fn build(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect()
}

#[allow(clippy::maybe_infinite_iter)]
fn digits_count(s: u64) -> usize {
    (0..).take_while(|i| 10u64.pow(*i) <= s).count()
}

fn split(mut s: u64, digits_count: usize) -> Vec<u64> {
    let half_digits_count = digits_count / 2;

    let get_half = |s: &mut u64| -> u64 {
        (0..half_digits_count)
            .map(|p| {
                let d = *s % 10;
                *s /= 10;
                d * 10u64.pow(u32::try_from(p).unwrap())
            })
            .sum()
    };

    let right = get_half(&mut s);
    let left = get_half(&mut s);
    vec![left, right]
}

fn blink(stones: &[u64]) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|s| {
            let digits_count = digits_count(*s);
            if *s == 0 {
                vec![1]
            } else if digits_count % 2 == 0 {
                split(*s, digits_count)
            } else {
                vec![s * 2024]
            }
        })
        .collect()
}

fn stones_count(stones: &[u64], blink_count: usize) -> usize {
    let mut stones = stones.to_vec();
    for _ in 0..blink_count {
        stones = blink(&stones);
    }
    stones.len()
}

fn part2(stones: &[u64]) -> usize {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let stones = build(&input);

    println!("Part 1: {}", stones_count(&stones, 25));
    println!("Part 2: {}", part2(&stones));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str = include_str!("../resources/input_test_1");

    #[test]
    fn test_split() {
        assert_eq!(split(1234, 4), [12, 34]);
        assert_eq!(split(1000, 4), [10, 0]);
    }

    #[test]
    fn test_blink() {
        let stones = build("0 1 10 99 999");
        assert_eq!(blink(&stones), build("1 2024 1 0 9 9 2021976"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(stones_count(&build(INPUT_TEST), 6), 22);
        assert_eq!(stones_count(&build(INPUT_TEST), 25), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&build(INPUT_TEST)), 0);
    }
}
