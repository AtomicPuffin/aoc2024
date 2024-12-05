use itertools::Itertools;
//use std::collections::HashMap;
//use std::collections::HashSet;
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
    let (rules, mut updates) = parse_input(input);
    let mut sum = 0;
    updates.iter_mut().for_each(|update| {
        if test_update(update, &rules) {
            sum += update[update.len() / 2];
        }
    });
    sum
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let (rules_raw, updates_raw) = input.split("\n\n").collect_tuple().unwrap();
    let rules = rules_raw
        .lines()
        .map(|x| x.split_once('|').unwrap())
        .into_iter()
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();
    let updates: Vec<Vec<i64>> = updates_raw
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|num| num.trim().parse::<i64>().ok())
                .collect()
        })
        .collect();
    (rules, updates)
}

fn test_update(update: &mut Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    for (a, b) in rules {
        if let Some(pos_a) = update.iter().position(|&x| x == *a) {
            if let Some(pos_b) = update.iter().position(|&x| x == *b) {
                if pos_a > pos_b {
                    return false;
                }
            }
        }
    }
    true
}

fn part_2(input: &str) -> i64 {
    let (rules, mut updates) = parse_input(input);
    let mut sum = 0;
    for update in updates.iter_mut() {
        if !test_update(update, &rules) {
            let new_update = fix_update(update, &rules);
            sum += new_update[new_update.len() / 2];
        }
    }
    sum
}

fn fix_update(update: &mut Vec<i64>, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut fixed = false;
    let mut new_update = update.clone();
    while !fixed {
        for (a, b) in rules {
            if let Some(pos_a) = new_update.iter().position(|&x| x == *a) {
                if let Some(pos_b) = new_update.iter().position(|&x| x == *b) {
                    if pos_a > pos_b {
                        new_update.remove(pos_b);
                        new_update.insert(pos_a, *b);
                    }
                }
            }
        }
        fixed = test_update(&mut new_update.clone(), rules);
    }
    new_update
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
        assert_eq!(part_1(&read_file("example.txt")), 143);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 5275);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 123);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 6191);
    }
}
