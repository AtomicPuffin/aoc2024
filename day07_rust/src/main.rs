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
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut sum = 0;
    let (values, numbers) = parse_input(input);
    for i in 0..values.len() {
        let value = values[i];
        let row = &numbers[i];
        if evaluate_tail(value, row, 0) {
            sum += value;
        }
    }
    sum
}

fn evaluate_tail(value: i64, row: &[i64], sum: i64) -> bool {
    let next = row[0];
    let sum_add = sum + next;
    let sum_mul = sum * next;
    if row[1..].len() == 0 {
        return sum_add == value || sum_mul == value;
    }
    evaluate_tail(value, &row[1..], sum_add) || evaluate_tail(value, &row[1..], sum_mul)
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<i64>>) {
    let mut values = Vec::new();
    let mut numbers_list = Vec::new();
    for line in input.lines() {
        let (raw_value, raw_numbers) = line.split_once(": ").unwrap();
        let value = raw_value.trim().parse::<i64>().unwrap();
        values.push(value);
        let numbers = raw_numbers
            .split(' ')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        numbers_list.push(numbers);
    }

    (values, numbers_list)
}

fn part_2(input: &str) -> i64 {
    let mut sum = 0;
    let (values, numbers) = parse_input(input);
    for i in 0..values.len() {
        let value = values[i];
        let row = &numbers[i];
        if evaluate_tail2(value, row, 0) {
            sum += value;
        }
    }
    sum
}

fn evaluate_tail2(value: i64, row: &[i64], sum: i64) -> bool {
    let next = row[0];
    let sum_add = sum + next;
    let sum_mul = sum * next;
    let sum_concat = format!("{}{}", sum, next).parse::<i64>().unwrap();
    if row[1..].len() == 0 {
        return sum_add == value || sum_mul == value || sum_concat == value;
    }
    evaluate_tail2(value, &row[1..], sum_add)
        || evaluate_tail2(value, &row[1..], sum_mul)
        || evaluate_tail2(value, &row[1..], sum_concat)
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
        assert_eq!(part_1(&read_file("example.txt")), 3749);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1611660863222);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 11387);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 945341732469724);
    }
}
