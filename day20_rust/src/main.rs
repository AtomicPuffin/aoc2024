//#![allow(unused_mut)]
//use itertools::Itertools;
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
    let (path, start, stop) = parse_input(input);
    let mut sum = 0;
    let cheats = find_cheats(&path, 2);

    let ordered_path = order_path(&path, start, stop);
    let mut saves = HashMap::new();
    let mut pos_indexes = HashMap::new();
    for (i, pos) in ordered_path.iter().enumerate() {
        pos_indexes.insert(pos, i);
    }
    for (cheat, steps) in cheats {
        let cheat_start = pos_indexes[&cheat.0];
        let cheat_end = pos_indexes[&cheat.1];
        if cheat_start > cheat_end {
            continue;
        }
        let mut saved = cheat_end - cheat_start - steps as usize;
        if steps as usize > cheat_end - cheat_start {
            saved = 0;
        }
        *saves.entry(saved).or_insert(0) += 1;
        if saved >= 100 {
            sum += 1;
        }
    }
    println!("Saves: {:?}", saves);
    sum
}

fn order_path(path: &HashSet<(i64, i64)>, start: (i64, i64), stop: (i64, i64)) -> Vec<(i64, i64)> {
    let mut new_path = Vec::new();
    let mut previous = (0, 0);
    let mut pos = start;
    new_path.push(start);
    loop {
        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if new_pos != previous && path.contains(&new_pos) {
                new_path.push(new_pos);
                previous = pos;
                pos = new_pos;
                if new_pos == stop {
                    return new_path;
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (HashSet<(i64, i64)>, (i64, i64), (i64, i64)) {
    let mut y = 0;
    let mut path = HashSet::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for line in input.lines() {
        let mut x = 0;
        for pos in line.chars() {
            if pos == '.' {
                path.insert((x, y));
            } else if pos == 'S' {
                start = (x, y);
                path.insert((x, y));
            } else if pos == 'E' {
                end = (x, y);
                path.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    (path, start, end)
}

fn part_2(input: &str) -> i64 {
    let (path, start, stop) = parse_input(input);
    let mut sum = 0;
    let cheats = find_cheats(&path, 20);
    let ordered_path = order_path(&path, start, stop);
    let mut pos_indexes = HashMap::new();
    for (i, pos) in ordered_path.iter().enumerate() {
        pos_indexes.insert(pos, i);
    }
    for (cheat, steps) in cheats {
        let cheat_start = pos_indexes[&cheat.0];
        let cheat_end = pos_indexes[&cheat.1];
        if cheat_start > cheat_end {
            continue;
        }
        let mut saved = cheat_end - cheat_start - steps as usize;
        if steps as usize > cheat_end - cheat_start {
            saved = 0;
        }
        if saved >= 100 {
            sum += 1;
        }
    }
    sum
}

fn find_cheats(path: &HashSet<(i64, i64)>, steps: i64) -> HashMap<((i64, i64), (i64, i64)), i64> {
    let mut cheats = HashMap::new();
    for pos in path.clone() {
        for y in 0..steps + 1 {
            for x in 0..steps - y + 1 {
                let new_pos_1 = (pos.0 + x, pos.1 + y);
                let new_pos_2 = (pos.0 - x, pos.1 - y);
                let new_pos_3 = (pos.0 + x, pos.1 - y);
                let new_pos_4 = (pos.0 - x, pos.1 + y);
                for new_pos in vec![new_pos_1, new_pos_2, new_pos_3, new_pos_4] {
                    if path.contains(&new_pos) && x + y > 1 {
                        cheats.insert((pos, new_pos), x + y);
                    }
                }
            }
        }
    }
    cheats
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
        assert_eq!(part_1(&read_file("input.txt")), 1521);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 0);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1013106);
    }
}
