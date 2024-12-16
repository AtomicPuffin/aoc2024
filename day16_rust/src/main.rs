//use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    let (maze, reindeer, xmax, ymax) = parse_input(input);
    dijkstra_with_direction_change(&maze, reindeer, xmax, ymax, (1, 0), 1000).0
}

fn dijkstra_with_direction_change(
    maze: &HashMap<(i64, i64), char>,
    start: (i64, i64),
    xmax: i64,
    ymax: i64,
    direction: (i64, i64),
    change_cost: i64,
) -> (i64, i64) {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut path = HashSet::new();
    path.insert(start);
    dist.insert((start, direction), (0, path));
    heap.push(Reverse((0, start, direction)));
    let mut best = i64::MAX;
    let mut visited = HashSet::new();

    while let Some(Reverse((current_cost, current_position, prev_direction))) = heap.pop() {
        // In theory we can reach E from 2 directions at the same cost
        if maze.get(&current_position) == Some(&'E') && current_cost <= best {
            if current_cost < best {
                best = current_cost;
            }
            visited = visited
                .union(&dist[&(current_position, prev_direction)].1)
                .cloned()
                .collect();
            //print_maze(&maze, xmax, ymax, &visited);
            continue;
        }

        // Explore neighbors
        for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_position = (current_position.0 + dir.0, current_position.1 + dir.1);

            if maze.get(&new_position) == Some(&'#') {
                continue;
            }
            let new_cost = current_cost
                + 1
                + if prev_direction == dir {
                    0
                } else {
                    change_cost
                };
            //let old_dist = dist.get(&(new_position, dir));
            let old_cost = dist
                .get(&(new_position, dir))
                .map(|d| d.0)
                .unwrap_or(i64::MAX);
            if new_cost <= old_cost {
                let mut new_path = dist[&(current_position, prev_direction)].1.clone();
                new_path.insert(new_position);
                //If we have the same cost, combine paths
                if new_cost == old_cost && dist.contains_key(&(new_position, dir)) {
                    new_path = new_path
                        .union(&dist[&(new_position, dir)].1)
                        .cloned()
                        .collect();
                }

                dist.insert((new_position, dir), (new_cost, new_path));
                heap.push(Reverse((new_cost, new_position, dir)));
            }
        }
    }
    (best, visited.len() as i64)
}

fn print_maze(
    maze: &HashMap<(i64, i64), char>,
    xmax: i64,
    ymax: i64,
    visited: &HashSet<(i64, i64)>,
) {
    for y in 0..ymax {
        for x in 0..xmax {
            if visited.contains(&(x, y)) {
                print!("O");
            } else if maze.contains_key(&(x, y)) {
                print!("{}", maze[&(x, y)]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> (HashMap<(i64, i64), char>, (i64, i64), i64, i64) {
    let mut y = 0;
    let mut maze = HashMap::new();
    let mut reindeer = (0, 0);
    let mut x = 0;
    for line in input.lines() {
        x = 0;
        for pos in line.chars() {
            if pos == '#' || pos == 'E' {
                maze.insert((x, y), pos);
            } else if pos == 'S' {
                reindeer = (x, y);
            }
            x += 1;
        }
        y += 1;
    }
    (maze, reindeer, x, y)
}

fn part_2(input: &str) -> i64 {
    let (maze, reindeer, xmax, ymax) = parse_input(input);
    dijkstra_with_direction_change(&maze, reindeer, xmax, ymax, (1, 0), 1000).1
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
        assert_eq!(part_1(&read_file("example2.txt")), 11048);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 72428);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 64);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 456);
    }
}
