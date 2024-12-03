use itertools::Itertools;
use regex::Regex;
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

fn part_1(input: &str) -> i64 {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(input) {
        let num1 = &cap[1];
        let num2 = &cap[2];
        sum += num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap();
    }
    sum
}

fn part_2(mut input: &str) -> i64 {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;
    let mut do_switch = "don't()";
    while let Some((before, after)) = input.split_once(do_switch) {
        input = after;
        if do_switch == "don't()" {
            sum += part_1(before);
            do_switch = "do()";
        } else {
            do_switch = "don't()";
        }
    }
    if do_switch == "don't()" {
        sum += part_1(input);
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
        assert_eq!(part_1(&read_file("example.txt")), 161);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 188741603);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 48);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 67269798);
    }
}
