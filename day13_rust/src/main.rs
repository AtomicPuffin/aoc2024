//use itertools::Itertools;
//use std::collections::HashMap;
//use std::collections::HashSet;
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
    let max = 100;
    let machines = parse_input(input);
    let mut sum = 0;
    for machine in machines {
        let solution = solve_linear_equation(
            machine[0].0,
            machine[1].0,
            machine[2].0,
            machine[0].1,
            machine[1].1,
            machine[2].1,
        );
        if let Some((a, b)) = solution {
            if a <= max && b <= max && a >= 0 && b >= 0 {
                sum += a * 3 + b
            }
        }
    }
    sum
}

fn solve_linear_equation(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64) -> Option<(i64, i64)> {
    let det = a * e - b * d;
    let x = (c * e - f * b) / det;
    let y = (a * f - c * d) / det;
    if det == 0 {
        return None;
    }
    if det * x != (c * e - f * b) || det * y != (a * f - c * d) {
        return None;
    }
    Some((x, y))
}

fn parse_input(input: &str) -> Vec<Vec<(i64, i64)>> {
    let machines = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.lines().collect::<Vec<&str>>())
        .map(|x| {
            x.iter()
                .map(|y| y.split_once(": ").unwrap().1.split_once(", ").unwrap())
                .map(|(x, y)| {
                    (
                        x[2..].parse::<i64>().unwrap(),
                        y[2..].parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64)>>>();

    machines
}

fn parse_input_old(input: &str) -> Vec<Vec<(i64, i64)>> {
    let machines_raw = input.split("\n\n").collect::<Vec<&str>>();
    let mut machines = Vec::new();
    for m in machines_raw {
        let rows = m.lines().collect::<Vec<&str>>();
        let mut machine = Vec::new();
        for row in rows {
            let (x, y) = row.split_once(": ").unwrap().1.split_once(", ").unwrap();
            machine.push((
                x[2..].parse::<i64>().unwrap(),
                y[2..].parse::<i64>().unwrap(),
            ));
        }
        machines.push(machine);
    }
    machines
}

fn part_2(input: &str) -> i64 {
    let machines = parse_input(input);
    let mut sum = 0;
    for machine in machines {
        let solution = solve_linear_equation(
            machine[0].0,
            machine[1].0,
            machine[2].0 + 10000000000000,
            machine[0].1,
            machine[1].1,
            machine[2].1 + 10000000000000,
        );
        if let Some((a, b)) = solution {
            if a >= 0 && b >= 0 {
                sum += a * 3 + b
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
        assert_eq!(part_1(&read_file("example.txt")), 480);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 38714);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 875318608908);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 74015623345775);
    }
}
