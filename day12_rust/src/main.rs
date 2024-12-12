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
    let regions = find_regions(&mut parse_input(input));
    let mut sum = 0;
    for region in regions {
        sum += region.len() as i64 * find_fence(&region).len() as i64;
    }
    sum
}

fn find_regions(garden: &mut HashMap<(i64, i64), char>) -> Vec<HashSet<(i64, i64)>> {
    let mut regions = Vec::new();
    while !garden.is_empty() {
        let pos = *garden.keys().next().unwrap();
        let region = find_region(garden, pos);
        regions.push(region);
    }
    regions
}

fn find_region(garden: &mut HashMap<(i64, i64), char>, pos: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut region = HashSet::new();
    let mut to_visit = vec![pos];
    let letter = garden[&pos];
    region.insert(pos);
    garden.remove(&pos);
    while let Some(pos) = to_visit.pop() {
        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if garden.contains_key(&new_pos) && garden[&new_pos] == letter {
                to_visit.push(new_pos);
                region.insert(new_pos);
                garden.remove(&new_pos);
            }
        }
    }
    region
}

fn find_fence(region: &HashSet<(i64, i64)>) -> HashSet<((i64, i64), (i64, i64))> {
    let mut fence_segments = HashSet::new();
    for pos in region {
        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
            if !region.contains(&new_pos) {
                fence_segments.insert((*pos, *dir));
            }
        }
    }
    fence_segments
}

fn parse_input(input: &str) -> HashMap<(i64, i64), char> {
    let mut y = 0;
    let mut garden = HashMap::new();
    for line in input.lines() {
        let mut x = 0;
        for pos in line.chars() {
            garden.insert((x, y), pos);
            x += 1;
        }
        y += 1;
    }
    garden
}

fn part_2(input: &str) -> i64 {
    let regions = find_regions(&mut parse_input(input));
    let mut sum = 0;
    for region in regions {
        sum += region.len() as i64 * find_sides(&mut find_fence(&region));
    }
    sum
}
fn find_sides(fence_segments: &mut HashSet<((i64, i64), (i64, i64))>) -> i64 {
    let mut sum = 0;
    while !fence_segments.is_empty() {
        let (pos, dir) = *fence_segments.iter().next().unwrap();
        fence_segments.remove(&(pos, dir));
        sum += 1;
        let dir_1 = (dir.1, -dir.0);
        let dir_2 = (-dir.1, dir.0);
        for test_dir in [&dir_1, &dir_2] {
            let mut neighbor = ((pos.0 + test_dir.0, pos.1 + test_dir.1), dir);
            while fence_segments.contains(&neighbor) {
                fence_segments.remove(&neighbor);
                neighbor = (
                    (neighbor.0 .0 + test_dir.0, neighbor.0 .1 + test_dir.1),
                    dir,
                );
            }
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
        assert_eq!(part_1(&read_file("example.txt")), 1930);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1464678);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 1206);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 877492);
    }
}
