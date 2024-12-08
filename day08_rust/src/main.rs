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
    let xmax = input.lines().next().unwrap().len() as i64;
    let ymax = input.lines().count() as i64;
    let antenna_map = parse_input(input);
    let mut antinodes = HashSet::new();
    for (pos, value) in &antenna_map {
        for (pos2, value2) in &antenna_map {
            if pos != pos2 && value == value2 {
                let antinode = (pos2.0 + (pos2.0 - pos.0), pos2.1 + (pos2.1 - pos.1));
                if antinode.0 >= 0 && antinode.0 < xmax && antinode.1 >= 0 && antinode.1 < ymax {
                    antinodes.insert(antinode);
                }
            }
        }
    }
    antinodes.len() as i64
}

fn parse_input(input: &str) -> HashMap<(i64, i64), char> {
    let mut y = 0;
    let mut antenna_map = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        for pos in line.chars() {
            if pos != '.' {
                antenna_map.insert((x, y), pos);
            }
            x += 1;
        }
        y += 1;
    }
    antenna_map
}

fn part_2(input: &str) -> i64 {
    let xmax = input.lines().next().unwrap().len() as i64;
    let ymax = input.lines().count() as i64;
    let antenna_map = parse_input(input);
    let mut antinodes = HashSet::new();
    for (pos, value) in &antenna_map {
        for (pos2, value2) in &antenna_map {
            if pos != pos2 && value == value2 {
                let mut antinode = *pos2;
                while antinode.0 >= 0 && antinode.0 < xmax && antinode.1 >= 0 && antinode.1 < ymax {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 + (pos2.0 - pos.0), antinode.1 + (pos2.1 - pos.1));
                }
            }
        }
    }
    antinodes.len() as i64
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
        assert_eq!(part_1(&read_file("example.txt")), 14);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 320);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 34);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1157);
    }
}
