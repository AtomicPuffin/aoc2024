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
    let reports = parse_lists(input);
    let mut counter = 0;
    for row in reports {
        if test_row(row) {
            counter += 1;
        }
    }
    counter
}

fn part_2(input: &str) -> i64 {
    let reports = parse_lists(input);
    let mut counter = 0;
    for row in reports {
        if test_row(row.clone()) {
            counter += 1;
        } else {
            for i in 0..row.len() {
                let mut row = row.clone();
                row.remove(i);
                if test_row(row) {
                    counter += 1;
                    break;
                }
            }
        }
    }
    counter
}

fn test_row(row: Vec<i64>) -> bool {
    let mut row = row.into_iter();
    let mut old = row.next().unwrap();
    let mut polarity = 0;
    while let Some(new) = row.next() {
        if polarity == 0 {
            if new > old {
                polarity = 1;
            } else {
                polarity = -1;
            }
        } else if (new - old) * polarity < 0 {
            return false;
        }

        if (new - old).abs() < 1 || (old - new).abs() > 3 {
            return false;
        }
        old = new;
    }
    true
}

fn parse_lists(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>()
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
        assert_eq!(part_1(&read_file("example.txt")), 2);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 559);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 4);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 601);
    }
}
