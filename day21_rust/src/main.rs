//#![allow(unused_mut)]
use itertools::Itertools;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"), 3)
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt"), 3));
    println!("Answer to Part 2: {}", part_1(&read_file("input.txt"), 26));
}

fn part_1(input: &str, robots: usize) -> usize {
    let codes = parse_input(input);
    let mut cache = HashMap::new();
    let numpad = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
        ('#', (0, 3)),
    ]);
    let mut keypad_position = 'A';
    let mut sum = 0;
    for code in codes {
        //parse keypad
        let mut partial_sum = 0;
        for button in code.1 {
            let mut sequences_1 = Vec::new();
            sequences_1.append(&mut find_paths(keypad_position, button, &numpad));
            // run the robots
            keypad_position = button;
            let mut min = usize::MAX;
            for sequence in sequences_1 {
                let part_sum = dfs(&mut cache, sequence, 1, robots);
                if part_sum < min {
                    min = part_sum;
                }
            }
            partial_sum += min;
        }
        sum += code.0 * partial_sum;
    }
    sum
}

fn find_paths(
    start: char,
    destination: char,
    keypad: &HashMap<char, (i64, i64)>,
) -> Vec<Vec<char>> {
    let mut path = Vec::new();
    let a = keypad[&start];
    let b = keypad[&destination];
    let vertical = b.1 - a.1;
    let horizontal = b.0 - a.0;
    let up = if vertical < 0 { vertical.abs() } else { 0 };
    let down = if vertical > 0 { vertical } else { 0 };
    let left = if horizontal < 0 { horizontal.abs() } else { 0 };
    let right = if horizontal > 0 { horizontal } else { 0 };

    for _ in 0..right {
        path.push('>');
    }
    for _ in 0..up {
        path.push('^');
    }
    for _ in 0..down {
        path.push('v');
    }
    for _ in 0..left {
        path.push('<');
    }
    let mut permutations = path
        .clone()
        .into_iter()
        .permutations(path.len())
        .collect::<Vec<_>>();
    let mut path = Vec::new();
    if keypad.len() == 12 {
        if (a.0 == 0 && b.1 == 3) || (a.1 == 3 && b.0 == 0) {
            // remove dead corner paths
            for _ in 0..left {
                path.push('<');
            }
            for _ in 0..down {
                path.push('v');
            }
            for _ in 0..up {
                path.push('^');
            }
            for _ in 0..right {
                path.push('>');
            }
        }
    } else {
        if (a.0 == 0 && b.1 == 0) || (a.1 == 0 && b.0 == 0) {
            //direcional pad
            for _ in 0..up {
                path.push('^');
            }
            for _ in 0..left {
                path.push('<');
            }
            for _ in 0..down {
                path.push('v');
            }
            for _ in 0..right {
                path.push('>');
            }
        }
    }
    if path.len() > 0 {
        //remove corner paths
        permutations = permutations
            .into_iter()
            .filter(|x| x != &path)
            .collect::<Vec<_>>();
    }
    let upper = permutations.len();
    for perm in &mut permutations[0..upper] {
        perm.push('A');
    }
    permutations
}

fn parse_input(input: &str) -> Vec<(usize, Vec<char>)> {
    let mut codes = Vec::new();
    for line in input.lines() {
        let sequence = line.chars().collect::<Vec<char>>();
        let trimmed = &line[..line.len() - 1];
        let code = trimmed.parse::<usize>().unwrap();
        codes.push((code, sequence));
    }
    codes
}

fn dfs(
    cache: &mut HashMap<(String, usize), usize>,
    sequence: Vec<char>,
    depth: usize,
    robots: usize,
) -> usize {
    let directional = HashMap::from([
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
        ('#', (0, 0)),
    ]);
    if depth == robots {
        return sequence.len();
    }
    let mut sum = 0;
    let mut current = Vec::new();
    for c in sequence {
        if c == 'A' {
            current.push(c);
            let key = current.iter().collect::<String>();
            if let Some(&number) = cache.get(&(key.clone(), depth)) {
                sum += number;
            } else {
                let mut directional_position = 'A';
                let mut partial_sum = 0;
                for curr in &current {
                    let next = find_paths(directional_position, *curr, &directional);
                    directional_position = *curr;

                    let mut min = usize::MAX;
                    for n in next {
                        let part_sum = dfs(cache, n, depth + 1, robots);
                        if part_sum < min {
                            min = part_sum;
                        }
                    }
                    partial_sum += min;
                }
                cache.insert((key, depth), partial_sum);
                sum += partial_sum;
            }
            current.clear();
        } else {
            current.push(c);
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
        assert_eq!(part_1(&read_file("example.txt"), 3), 126384);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 3), 138764);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_1(&read_file("input.txt"), 26), 169137886514152);
    }
}
