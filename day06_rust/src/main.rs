//use itertools::Itertools;
//use std::collections::HashMap;
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
    let (lab_map, mut guard_pos) = parse_input(input);
    let mut visited = HashSet::new();
    visited.insert(guard_pos.0);
    let ymax = input.lines().count() as i64;
    let xmax = input.lines().next().unwrap().len() as i64;
    loop {
        let new_pos = (
            guard_pos.0 .0 + guard_pos.1 .0,
            guard_pos.0 .1 + guard_pos.1 .1,
        );
        if new_pos.0 < 0 || new_pos.0 >= xmax || new_pos.1 < 0 || new_pos.1 >= ymax {
            // out of bounds
            break;
        } else if !lab_map.contains(&new_pos) {
            // clear move
            guard_pos.0 = new_pos;
            visited.insert(new_pos);
        } else {
            // Turn right
            let new_dir = match guard_pos.1 {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => panic!("Invalid direction"),
            };
            guard_pos.1 = new_dir;
        }
    }
    visited.len() as i64
}

fn parse_input(input: &str) -> (HashSet<(i64, i64)>, ((i64, i64), (i64, i64))) {
    let mut y = 0;
    let mut lab_map = HashSet::new();
    let mut guard_pos = ((0, 0), (0, 0));
    for line in input.lines() {
        let mut x = 0;
        for pos in line.chars() {
            if pos == '#' {
                lab_map.insert((x, y));
            } else if pos == '^' {
                guard_pos = ((x, y), (0, -1));
            }
            x += 1;
        }
        y += 1;
    }
    (lab_map, guard_pos)
}

fn part_2(input: &str) -> i64 {
    let ymax = input.lines().count() as i64;
    let xmax = input.lines().next().unwrap().len() as i64;
    let (lab_map, orig_guard_pos) = parse_input(input);
    let mut counter = 0;
    // Brute force
    for y in 0..ymax {
        for x in 0..xmax {
            // If legal to add, try
            if !lab_map.contains(&(x, y)) && (x, y) != orig_guard_pos.0 {
                let mut new_map = lab_map.clone();
                let mut guard_pos = orig_guard_pos.clone();
                new_map.insert((x, y));
                let mut visited = HashSet::new();
                visited.insert(guard_pos);
                loop {
                    let new_pos = (
                        guard_pos.0 .0 + guard_pos.1 .0,
                        guard_pos.0 .1 + guard_pos.1 .1,
                    );
                    if new_pos.0 < 0 || new_pos.0 >= xmax || new_pos.1 < 0 || new_pos.1 >= ymax {
                        // out of bounds, failure
                        break;
                    } else if !new_map.contains(&new_pos) {
                        // clear move
                        guard_pos.0 = new_pos;
                        if visited.contains(&guard_pos) {
                            // loop and success
                            counter += 1;
                            break;
                        }
                        visited.insert(guard_pos);
                    } else {
                        // Turn right
                        let new_dir = match guard_pos.1 {
                            (0, -1) => (1, 0),
                            (1, 0) => (0, 1),
                            (0, 1) => (-1, 0),
                            (-1, 0) => (0, -1),
                            _ => panic!("Invalid direction"),
                        };
                        guard_pos.1 = new_dir;
                    }
                }
            }
        }
    }
    counter
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
        assert_eq!(part_1(&read_file("example.txt")), 41);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 5305);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 6);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 2143);
    }
}
