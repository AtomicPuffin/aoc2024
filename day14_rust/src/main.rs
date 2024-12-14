//use itertools::Itertools;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), (11, 7))
    );
    println!(
        "Answer to Part 1: {}",
        part_1(&read_file("input.txt"), (101, 103))
    );
    println!(
        "Answer to Part 2: {}",
        part_2(&read_file("input.txt"), (101, 103))
    );
}

fn part_1(input: &str, max: (i64, i64)) -> i64 {
    let mut robots = parse_input(input);
    move_robots(&mut robots, 100, max);
    let midx = max.0 / 2;
    let midy = max.1 / 2;
    let mut quad_a = 0;
    let mut quad_b = 0;
    let mut quad_c = 0;
    let mut quad_d = 0;
    for robot in robots.iter() {
        if robot.0 .0 < midx && robot.0 .1 < midy {
            quad_a += 1;
        } else if robot.0 .0 > midx && robot.0 .1 < midy {
            quad_b += 1;
        } else if robot.0 .0 < midx && robot.0 .1 > midy {
            quad_c += 1;
        } else if robot.0 .0 > midx && robot.0 .1 > midy {
            quad_d += 1;
        }
    }
    quad_a * quad_b * quad_c * quad_d
}

fn move_robots(robots: &mut Vec<((i64, i64), (i64, i64))>, seconds: i64, max: (i64, i64)) {
    for robot in robots.iter_mut() {
        if robot.1 .0 >= 0 {
            robot.0 .0 = true_modulo(robot.0 .0 + robot.1 .0 * seconds, max.0);
        } else {
            robot.0 .0 = true_modulo(robot.0 .0 + robot.1 .0 * seconds + max.0, max.0);
        }
        if robot.1 .1 >= 0 {
            robot.0 .1 = true_modulo(robot.0 .1 + robot.1 .1 * seconds, max.1);
        } else {
            robot.0 .1 = true_modulo(robot.0 .1 + robot.1 .1 * seconds + max.1, max.1);
        }
    }
}

fn true_modulo(a: i64, b: i64) -> i64 {
    (a % b + b) % b
}

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    let mut robots = Vec::new();
    for r in input.lines() {
        let (p, speed) = r.split_once(" ").unwrap();
        let (x, y) = p[2..].split_once(",").unwrap();
        let pos = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        let (x, y) = speed[2..].split_once(",").unwrap();
        let spd = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        robots.push((pos, spd));
    }
    robots
}

fn print_floor(robots: &Vec<((i64, i64), (i64, i64))>, max: (i64, i64)) {
    let mut floor = vec![vec!['.'; max.0 as usize]; max.1 as usize];
    for robot in robots.iter() {
        floor[robot.0 .1 as usize][robot.0 .0 as usize] = '#';
    }
    for row in floor.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part_2(input: &str, max: (i64, i64)) -> i64 {
    let mut robots = parse_input(input);
    let seconds = 15000;
    let mut states = HashSet::new();
    let mut tree = 0;
    let mut minimum_density = f64::MAX;
    let mut image = Vec::new();
    states.insert(robots.clone());
    for s in 1..seconds {
        move_robots(&mut robots, 1, max);
        if states.contains(&robots) {
            println!("cycle detected at {}", s);
            break;
        }
        states.insert(robots.clone());

        let density = measure_density(&robots);
        if density < minimum_density {
            minimum_density = density;
            tree = s;
            image = robots.clone();
        }
    }
    print_floor(&image, max);
    tree
}

fn measure_density(robots: &Vec<((i64, i64), (i64, i64))>) -> f64 {
    let mut total_distance = 0.0;
    let mut distances = 0;
    for i in 0..robots.len() {
        for j in i + 1..robots.len() {
            let dx = (robots[i].0 .0 - robots[j].0 .0) as f64;
            let dy = (robots[i].0 .1 - robots[j].0 .1) as f64;
            let dist = (dx * dx + dy * dy).sqrt();
            total_distance += dist;
            distances += 1;
        }
    }
    total_distance / distances as f64
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
        assert_eq!(part_1(&read_file("example.txt"), (11, 7)), 12);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), (101, 103)), 225810288);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), (101, 103)), 6752);
    }
}
