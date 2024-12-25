//#![allow(unused_mut)]
//use itertools::Itertools;
//use std::collections::HashMap;
//use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let (keys, locks) = parse_input(input);
    let mut counter = 0;
    for key in keys {
        for lock in &locks {
            let mut overlap = false;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                counter += 1;
            }
        }
    }
    counter as i64
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for item in input.split("\n\n").collect::<Vec<&str>>() {
        if item.starts_with("#") {
            let mut lock = vec![0; 5];
            for (i, line) in item.lines().enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '#' {
                        lock[j] = i;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = vec![0; 5];
            for (i, line) in item.lines().enumerate() {
                for (j, c) in line.chars().enumerate() {
                    if c == '.' {
                        key[j] = 5 - i;
                    }
                }
            }
            keys.push(key);
        }
    }

    (keys, locks)
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
        assert_eq!(part_1(&read_file("example.txt")), 3);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 3057);
    }
}
