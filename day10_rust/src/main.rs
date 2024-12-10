//use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example2.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example2.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let hiking_map = parse_input(input);
    let mut sum = 0;

    for (pos, alt) in &hiking_map {
        let mut peaks = HashSet::new();
        if *alt == 0 {
            let new_peaks = find_trails(&hiking_map, *pos);
            for peak in new_peaks {
                peaks.insert(peak);
            }
        }
        sum += peaks.len() as i64;
    }
    sum
}
fn find_trails(hiking_map: &HashMap<(i64, i64), i64>, pos: (i64, i64)) -> Vec<(i64, i64)> {
    let mut peaks = Vec::new();
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if hiking_map[&pos] == 9 {
        return vec![pos];
    }
    for dir in &directions {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if hiking_map.contains_key(&new_pos) && hiking_map[&new_pos] == hiking_map[&pos] + 1 {
            peaks.extend(find_trails(hiking_map, new_pos));
        }
    }
    peaks
}

fn parse_input(input: &str) -> HashMap<(i64, i64), i64> {
    let mut y = 0;
    let mut hiking_map = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        for pos in line.chars() {
            hiking_map.insert((x, y), pos.to_string().parse::<i64>().unwrap());

            x += 1;
        }
        y += 1;
    }
    hiking_map
}

fn part_2(input: &str) -> i64 {
    let hiking_map = parse_input(input);
    let mut sum = 0;
    for (pos, alt) in &hiking_map {
        if *alt == 0 {
            sum += find_trails_2(&hiking_map, *pos);
        }
    }
    sum
}

fn find_trails_2(hiking_map: &HashMap<(i64, i64), i64>, pos: (i64, i64)) -> i64 {
    let mut sum = 0;
    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if hiking_map[&pos] == 9 {
        return 1;
    }
    for dir in &directions {
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if hiking_map.contains_key(&new_pos) && hiking_map[&new_pos] == hiking_map[&pos] + 1 {
            sum += find_trails_2(hiking_map, new_pos);
        }
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
        assert_eq!(part_1(&read_file("example2.txt")), 36);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 733);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 81);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1514);
    }
}
