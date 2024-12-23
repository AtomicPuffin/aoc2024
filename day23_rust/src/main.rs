#![allow(unused_mut)]
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

fn part_1(input: &str) -> u64 {
    let lan = parse_input_1(input);
    let mut triplets = HashSet::new();
    for (computer, buddies) in lan.clone() {
        for buddy in buddies.clone() {
            if buddy == computer {
                continue;
            }
            for buddy2 in lan[&buddy].clone() {
                if buddy2 != computer && buddies.contains(&buddy2) {
                    let mut triplet = vec![computer, buddy, buddy2];
                    triplet.sort();
                    triplets.insert(triplet);
                }
            }
        }
    }
    let mut sum = 0;
    for triplet in triplets {
        for computer in triplet {
            if computer.starts_with('t') {
                sum += 1;
                break;
            }
        }
    }
    sum
}

fn parse_input_1(input: &str) -> HashMap<&str, HashSet<&str>> {
    let links: Vec<(&str, &str)> = input.lines().map(|x| x.split_once('-').unwrap()).collect();
    let mut lan = HashMap::new();
    for (a, b) in links {
        lan.entry(a).or_insert(HashSet::new()).insert(b);
        lan.entry(b).or_insert(HashSet::new()).insert(a);
    }
    lan
}

fn part_2(input: &str) -> String {
    let connections = parse_input_2(input);
    let mut lan_candidates = Vec::new();
    for (_, buddies) in connections.clone() {
        let mut candidates = Vec::new();
        for buddy in buddies.clone() {
            let i2 = connections[&buddy].clone();
            let i1 = buddies.clone();
            let intersection = i1.intersection(&i2).copied().collect::<HashSet<_>>();
            candidates.push(intersection);
        }
        candidates.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut candidate = candidates[0].clone();
        for n in 1..candidates.len() - 1 {
            if n >= candidates.len() {
                break;
            }
            candidate = candidate.intersection(&candidates[n]).copied().collect();
        }

        lan_candidates.push(candidate);
    }
    let largest_set = lan_candidates.iter().max_by_key(|set| set.len()).unwrap();
    let mut set_vec = largest_set.iter().collect::<Vec<_>>();
    set_vec.sort();
    let result = set_vec.iter().map(|&&s| s).collect::<Vec<_>>().join(",");

    result
}

fn parse_input_2(input: &str) -> HashMap<&str, HashSet<&str>> {
    let links: Vec<(&str, &str)> = input.lines().map(|x| x.split_once('-').unwrap()).collect();
    let mut lan = HashMap::new();
    for (a, b) in links {
        lan.entry(a).or_insert(HashSet::new()).insert(b);
        lan.entry(b).or_insert(HashSet::new()).insert(a);
        lan.entry(a).or_insert(HashSet::new()).insert(a);
        lan.entry(b).or_insert(HashSet::new()).insert(b);
    }
    lan
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
        assert_eq!(part_1(&read_file("example.txt")), 7);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1599);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), "co,de,ka,ta");
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            part_2(&read_file("input.txt")),
            "av,ax,dg,di,dw,fa,ge,kh,ki,ot,qw,vz,yw"
        );
    }
}
