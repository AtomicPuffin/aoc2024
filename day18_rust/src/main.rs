//use itertools::Itertools;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), 6, 12)
    );
    println!(
        "Answer to Part 1: {}",
        part_1(&read_file("input.txt"), 70, 1024)
    );
    println!(
        "Answer to Part 2 test: {:?}",
        part_2(&read_file("example.txt"), 6, 12)
    );
    println!(
        "Answer to Part 2: {:?}",
        part_2(&read_file("input.txt"), 70, 1024)
    );
}

fn part_1(input: &str, size: i64, first_bytes: i64) -> i64 {
    let corruption = parse_input(input);
    let cut_corr = HashSet::from_iter(corruption[0..first_bytes as usize].iter().cloned());
    print_memory(&cut_corr, size);
    let start = (0, 0);
    let end = (size, size);
    djikstra(start, end, &cut_corr, size)
}

fn djikstra(
    start: (i64, i64),
    end: (i64, i64),
    corruption: &HashSet<(i64, i64)>,
    size: i64,
) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, steps)) = queue.pop_front() {
        if pos == end {
            return steps;
        }
        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos.0 < 0 || new_pos.0 > size || new_pos.1 < 0 || new_pos.1 > size {
                continue;
            }
            if !visited.contains(&new_pos) && !corruption.contains(&new_pos) {
                visited.insert(new_pos);
                queue.push_back((new_pos, steps + 1));
            }
        }
    }
    0
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    /*let mut objects = Vec::new();
    for line in input.lines() {
        let (a, b) = line.split_once(',').unwrap();

        objects.push((a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()));
    }
    objects*/
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect()
}

fn print_memory(memory: &HashSet<(i64, i64)>, size: i64) {
    for y in 0..size + 1 {
        for x in 0..size + 1 {
            if memory.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_2(input: &str, size: i64, first_bytes: i64) -> (i64, i64) {
    let corruption = parse_input(input);
    let start = (0, 0);
    let end = (size, size);
    for i in first_bytes..corruption.len() as i64 + 1 {
        let cut_corr = HashSet::from_iter(corruption[0..i as usize].iter().cloned());
        let path = djikstra(start, end, &cut_corr, size);
        if path == 0 {
            return corruption[(i - 1) as usize];
        }
    }
    (0, 0)
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
        assert_eq!(part_1(&read_file("example.txt"), 6, 12), 22);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 70, 1024), 268);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt"), 6, 12), (6, 1));
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 70, 1024), (64, 11));
    }
}
