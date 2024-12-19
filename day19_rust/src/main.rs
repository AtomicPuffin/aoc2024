//#![allow(unused_mut)]
//use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let (towels, patterns, max) = parse_input(input);
    let mut sum = 0;
    let mut cache = HashMap::new();
    for pattern in patterns {
        let combos = find_combinations(&towels, &pattern, max, &mut cache);
        if combos > 0 {
            sum += 1;
        }
    }
    sum
}

fn find_combinations(
    towels: &HashSet<&str>,
    pattern: &str,
    max: usize,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if cache.contains_key(pattern) {
        return cache[pattern];
    }
    let mut sum = 0;
    for n in 1..=max {
        if pattern.len() < n {
            break;
        }
        let search_pattern = &pattern[..n];
        if towels.contains(search_pattern) {
            let new_pattern = &pattern[n..];
            if new_pattern.is_empty() {
                sum += 1;
            } else {
                sum += find_combinations(towels, &new_pattern, max, cache);
            }
        }
    }
    cache.insert(pattern.to_string(), sum);
    sum
}

fn parse_input(input: &str) -> (HashSet<&str>, Vec<&str>, usize) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let patterns: Vec<&str> = patterns.lines().collect();
    let towels = towels.split(", ").collect::<HashSet<&str>>();
    let max = towels.iter().max_by_key(|s| s.len()).unwrap().len();
    (towels, patterns, max)
}

fn part_2(input: &str) -> i64 {
    let (towels, patterns, max) = parse_input(input);
    let mut sum = 0;
    let mut cache = HashMap::new();
    for pattern in patterns {
        sum += find_combinations(&towels, &pattern, max, &mut cache);
    }
    sum
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), 6);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 311);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 16);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 616234236468263);
    }
}
