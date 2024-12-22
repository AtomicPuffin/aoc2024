#![allow(unused_mut)]
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
        part_2(&read_file("example2.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> u64 {
    let mut secrets = parse_input(input);
    let mut sum = 0;
    for mut secret in secrets {
        for _ in 0..2000 {
            secret = calculate_secret(secret);
        }
        sum += secret;
    }
    sum
}

fn calculate_secret(s: u64) -> u64 {
    let mut secret = s;
    secret = (secret ^ secret << 6) % 16777216;
    secret = ((secret >> 5) ^ secret) % 16777216;
    secret = ((secret << 11) ^ secret) % 16777216;
    secret
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn part_2(input: &str) -> u64 {
    let mut secrets = parse_input(input);
    let mut change_vec = Vec::new();
    for mut secret in secrets {
        let mut changes = Vec::new();
        let mut last = (secret % 10) as i64;
        for _ in 0..2000 {
            secret = calculate_secret(secret);
            changes.push((((secret % 10) as i64 - last), (secret % 10) as i64));
            last = (secret % 10) as i64;
        }
        change_vec.push(changes);
    }

    let mut combo_sums = HashMap::new();
    for changes in change_vec.clone() {
        let mut found = HashSet::new();
        for i in 0..changes.len() {
            if i + 3 < changes.len() {
                let combination = (
                    changes[i].0,
                    changes[i + 1].0,
                    changes[i + 2].0,
                    changes[i + 3].0,
                );

                if found.contains(&combination) {
                    continue;
                }
                found.insert(combination);
                let value = changes[i + 3].1;
                *combo_sums.entry(combination.clone()).or_insert(0) += value;
            }
        }
    }
    combo_sums.values().max().unwrap().clone() as u64
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
        assert_eq!(part_1(&read_file("example.txt")), 37327623);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 12979353889);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 23);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1449);
    }
}
