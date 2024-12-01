use itertools::Itertools;
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
    let (mut list_a, mut list_b) = parse_lists(input);
    list_a
        .iter()
        .sorted()
        .zip(list_b.iter().sorted())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(input: &str) -> i64 {
    let (list_a, list_b) = parse_lists(input);
    list_a
        .into_iter()
        .map(|a| a * list_b.iter().filter(|&&x| x == a).count() as i64)
        .sum()
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|line| {
            let parts = line.split("   ").collect::<Vec<&str>>();
            let a = parts[0].parse::<i64>().unwrap();
            let b = parts[1].parse::<i64>().unwrap();
            (a, b)
        })
        .unzip()
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
        assert_eq!(part_1(&read_file("example.txt")), 11);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 936063);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 31);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 23150395);
    }
}
