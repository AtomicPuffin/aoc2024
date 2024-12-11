//use itertools::Itertools;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_both(&read_file("example.txt"), 25)
    );
    println!(
        "Answer to Part 1: {}",
        part_both(&read_file("input.txt"), 25)
    );
    println!(
        "Answer to Part 2: {}",
        part_both(&read_file("input.txt"), 75)
    );
}

// did this first
fn part_1_old(input: &str) -> i64 {
    let mut stones = input
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let iterations = 25;
    for _ in 0..iterations {
        let mut new_stones = Vec::new();
        for stone in stones.clone().iter_mut() {
            if stone == &0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let length = stone.to_string().len();
                let new_stone_1 = stone
                    .to_string()
                    .chars()
                    .take(length / 2)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                let new_stone_2 = stone
                    .to_string()
                    .chars()
                    .skip(length / 2)
                    .collect::<String>()
                    .parse::<i64>()
                    .unwrap();
                new_stones.push(new_stone_1);
                new_stones.push(new_stone_2);
            } else {
                new_stones.push(*stone * 2024);
            }
        }
        stones = new_stones.clone();
    }
    stones.len() as i64
}

fn part_both(input: &str, blinks: i64) -> i64 {
    let stones = input
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut cache = std::collections::HashMap::new();
    let mut sum = 0;
    for stone in stones {
        sum += depth_first(stone, &mut cache, blinks + 1);
    }
    sum
}
fn depth_first(stone: i64, cache: &mut HashMap<(i64, i64), i64>, old_depth: i64) -> i64 {
    let depth = old_depth - 1;
    if depth == 0 {
        return 1;
    }
    if cache.contains_key(&(stone, depth)) {
        return *cache.get(&(stone, depth)).unwrap();
    }
    if stone == 0 {
        let result = depth_first(1, cache, depth);
        cache.insert((stone, depth), result);
        return result;
    } else if stone.to_string().len() % 2 == 0 {
        let length = stone.to_string().len();
        let new_stone_1 = stone
            .to_string()
            .chars()
            .take(length / 2)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let new_stone_2 = stone
            .to_string()
            .chars()
            .skip(length / 2)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let result_1 = depth_first(new_stone_1, cache, depth);
        let result_2 = depth_first(new_stone_2, cache, depth);
        cache.insert((stone, depth), result_1 + result_2);
        return result_1 + result_2;
    } else {
        let result = depth_first(stone * 2024, cache, depth);
        cache.insert((stone, depth), result);
        return result;
    }
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
        assert_eq!(part_both(&read_file("example.txt"), 25), 55312);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_both(&read_file("input.txt"), 25), 194782);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_both(&read_file("input.txt"), 75), 233007586663131);
    }
}
