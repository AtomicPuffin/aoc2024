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
    let mut disk_map = parse_input(input);
    let mut refrag_disk_map = Vec::new();
    while let Some(pos) = disk_map.first().cloned() {
        disk_map.remove(0);
        if pos == -1 {
            let mut last = 0;
            loop {
                if disk_map.is_empty() {
                    break;
                }
                last = disk_map.pop().unwrap();
                if last != -1 {
                    break;
                }
            }
            refrag_disk_map.push(last);
        } else {
            refrag_disk_map.push(pos);
        }
    }
    let mut sum = 0;
    for i in 0..refrag_disk_map.len() {
        if refrag_disk_map[i] == -1 {
            continue;
        }
        sum += refrag_disk_map[i] * i as i64;
    }
    sum
}

fn parse_input(input: &str) -> Vec<i64> {
    let mut free_flip = false;
    let mut disk_map = Vec::new();
    let mut id_number = 0;
    for c in input.chars() {
        if !free_flip {
            let file_length = c.to_string().parse::<i64>().unwrap();
            free_flip = true;
            for _ in 0..file_length {
                disk_map.push(id_number);
            }
            id_number += 1;
        } else {
            free_flip = false;
            for _ in 0..c.to_string().parse::<i64>().unwrap() {
                disk_map.push(-1);
            }
        }
    }
    disk_map
}

fn part_2(input: &str) -> i64 {
    let mut disk_map = parse_input2(input);
    let mut id = 0;
    for i in (0..disk_map.len()).rev() {
        if disk_map[i].0 != -1 {
            id = disk_map[i].0;
            break;
        }
    }
    for file_id in (0..id + 1).rev() {
        let mut pos = 0;
        for i in (0..disk_map.len()).rev() {
            if disk_map[i].0 == file_id {
                pos = i;
                break;
            }
        }
        let file = disk_map[pos];
        for space in 0..disk_map.len() {
            if disk_map[space].0 == file_id {
                break;
            }
            if disk_map[space].1 >= file.1 && disk_map[space].0 == -1 {
                disk_map[pos] = (-1, disk_map[pos].1);

                if disk_map[space].1 - file.1 != 0 {
                    disk_map.insert(space + 1, (-1, disk_map[space].1 - file.1));
                }
                disk_map[space] = file;

                break;
            }
        }
    }

    let mut sum = 0;
    let mut refrag_disk_map = Vec::new();
    for i in 0..disk_map.len() {
        for _ in 0..disk_map[i].1 {
            refrag_disk_map.push(disk_map[i].0);
        }
    }
    for i in 0..refrag_disk_map.len() {
        if refrag_disk_map[i] == -1 {
            continue;
        }
        sum += refrag_disk_map[i] * i as i64;
    }
    sum
}

/*fn printable_disk_map(disk_map: Vec<(i64, i64)>) -> String {
    let mut printable_disk_map = String::new();
    for i in 0..disk_map.len() {
        for j in 0..disk_map[i].1 {
            printable_disk_map.push_str(&disk_map[i].0.to_string());
        }
    }
    printable_disk_map.replace("-1", ".")
}*/

fn parse_input2(input: &str) -> Vec<(i64, i64)> {
    let mut free_flip = false;
    let mut disk_map = Vec::new();
    let mut id_number = 0;
    for c in input.chars() {
        let file_length = c.to_string().parse::<i64>().unwrap();
        if !free_flip {
            free_flip = true;
            disk_map.push((id_number, file_length));
            id_number += 1;
        } else {
            free_flip = false;
            disk_map.push((-1, file_length));
        }
    }
    disk_map
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
        assert_eq!(part_1(&read_file("example.txt")), 1928);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 6382875730645);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 2858);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 6420913943576);
    }
}
