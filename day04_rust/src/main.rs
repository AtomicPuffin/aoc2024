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
    let search_space = parse_input(input);
    let mut count = 0;
    let patterns = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let tail = "MAS";
    for start in search_space.keys() {
        if search_space[start] == 'X' {
            for pattern in patterns.iter() {
                if next_step(*start, *pattern, tail, &search_space) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn next_step(
    current: (i64, i64),
    pattern: (i64, i64),
    tail: &str,
    search_space: &HashMap<(i64, i64), char>,
) -> bool {
    if current.0 as i64 + pattern.0 >= 0 && current.1 as i64 + pattern.1 >= 0 {
        let new = (current.0 + pattern.0, current.1 + pattern.1);
        if search_space.contains_key(&new) {
            if search_space[&new] == tail.chars().next().unwrap() {
                let new_tail = tail.chars().skip(1).collect::<String>();
                if new_tail.len() == 0 {
                    return true;
                } else {
                    return next_step(
                        new,
                        pattern,
                        &tail.chars().skip(1).collect::<String>(),
                        search_space,
                    );
                }
            } else {
                return false;
            }
        }
    }
    //Out of bounds
    false
}

fn part_2(input: &str) -> i64 {
    let search_space = parse_input(input);
    let mut count = 0;
    let pattern = ((1, 1), (-1, -1), (1, -1), (-1, 1));
    for current in search_space.keys() {
        if search_space[current] == 'A' {
            if current.0 as i64 + pattern.1 .0 >= 0
                && current.1 as i64 + pattern.1 .1 >= 0
                && current.1 as i64 + pattern.2 .1 >= 0
                && current.0 as i64 + pattern.3 .0 >= 0
            {
                let neighbor1 = (current.0 + pattern.0 .0, current.1 + pattern.0 .1);
                let neighbor2 = (current.0 + pattern.1 .0, current.1 + pattern.1 .1);
                let neighbor3 = (current.0 + pattern.2 .0, current.1 + pattern.2 .1);
                let neighbor4 = (current.0 + pattern.3 .0, current.1 + pattern.3 .1);
                if search_space.contains_key(&neighbor1)
                    && search_space.contains_key(&neighbor3)
                    && search_space.contains_key(&neighbor4)
                {
                    if ((search_space[&neighbor1] == 'M' && search_space[&neighbor2] == 'S')
                        || (search_space[&neighbor1] == 'S' && search_space[&neighbor2] == 'M'))
                        && ((search_space[&neighbor3] == 'M' && search_space[&neighbor4] == 'S')
                            || (search_space[&neighbor3] == 'S' && search_space[&neighbor4] == 'M'))
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> HashMap<(i64, i64), char> {
    let mut search_space = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            search_space.insert((y, x), c);
            x += 1;
        }
        y += 1;
    }
    search_space
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
        assert_eq!(part_1(&read_file("example.txt")), 18);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 2447);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 9);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1868);
    }
}
