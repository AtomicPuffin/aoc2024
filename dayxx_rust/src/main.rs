use itertools::Itertools;
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
    let mut sum = 0;
    let lists = parse_lists(input);
    let (mut list_a, mut list_b) = lists;
    list_a.sort();
    list_b.sort();
    while let Some(a) = list_a.pop() {
        let b = list_b.pop().unwrap();
        sum += (a - b).abs();
    }
    sum
}

fn part_2(input: &str) -> i64 {
    let mut sum = 0;
    sum
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list_a = Vec::new();
    let mut list_b = Vec::new();
    for line in input.lines() {
        let parts = line.split("   ").collect::<Vec<&str>>();
        let a = parts[0].parse::<i64>().unwrap();
        let b = parts[1].parse::<i64>().unwrap();
        list_a.push(a);
        list_b.push(b);
    }
    (list_a, list_b)
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
        assert_eq!(part_1(&read_file("example.txt")), 0);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 0);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 0);
    }
}
