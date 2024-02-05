use std::io::{self, Read};

fn build(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn is_opposite_polarity(s: &[char]) -> bool {
    const DIFF: u8 = b'a' - b'A';
    (s[0] as u8).abs_diff(s[1] as u8) == DIFF
}

fn scan(polymer: &[char]) -> Vec<char> {
    let mut polymer = polymer.to_vec();
    // chunks doesn't work as we don't get the proper index, which we need to remove the chars
    while let Some(i) = polymer
        .iter()
        .enumerate()
        .position(|(i, _)| i < polymer.len() - 1 && is_opposite_polarity(&polymer[i..=i + 1]))
    {
        polymer.drain(i..=i + 1);
        // println!("{}: {}", i, polymer.iter().map(char::to_string).collect::<Vec<_>>().join(""));
    }
    polymer
}

fn remaining_units_count(polymer: &[char]) -> usize {
    scan(polymer).len()
}

// unit is the lowercase char
fn remove_unit(polymer: &[char], unit: char) -> Vec<char> {
    let upper_unit = unit.to_ascii_uppercase();
    let mut polymer = polymer.to_vec();
    polymer.retain(|&v| v != unit && v != upper_unit);
    polymer
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn shortest_polymer(polymer: &[char]) -> usize {
    ASCII_LOWER
        .iter()
        .map(|unit| {
            let polymer = remove_unit(polymer, *unit);
            remaining_units_count(&polymer)
        })
        .min()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let polymer = build(&input);

    println!("Part 1: {}", remaining_units_count(&polymer));
    println!("Part 2: {}", shortest_polymer(&polymer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_opposite_polarity() {
        assert!(is_opposite_polarity(&['b', 'B']));
        assert!(is_opposite_polarity(&['C', 'c']));
        assert!(!is_opposite_polarity(&['b', 'A']));
        assert!(!is_opposite_polarity(&['C', 'C']));
        assert!(!is_opposite_polarity(&['E', 'C']));
        assert!(!is_opposite_polarity(&['r', 'r']));
    }

    #[test]
    fn test_remaining_units() {
        assert_eq!(scan(&build("aA")), vec![]);
        assert_eq!(scan(&build("abBA")), vec![]);
        assert_eq!(scan(&build("abAB")), vec!['a', 'b', 'A', 'B']);
        assert_eq!(scan(&build("aabAAB")), vec!['a', 'a', 'b', 'A', 'A', 'B']);
    }

    #[test]
    fn test_part1() {
        assert_eq!(remaining_units_count(&build("dabAcCaCBAcCcaDA")), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(shortest_polymer(&build("dabAcCaCBAcCcaDA")), 4);
    }
}