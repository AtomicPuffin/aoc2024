//use itertools::Itertools;
use std::collections::HashMap;
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
    let (mut objects, mut robot, instructions, xmax, ymax) = parse_input(input);
    for instruction in instructions.chars() {
        move_robot(&mut robot, &mut objects, instruction);
    }
    let mut sum = 0;
    print_objects(&mut objects, robot, xmax, ymax);
    for (pos, value) in objects.iter() {
        if *value == 'O' {
            sum += pos.0 + pos.1 * 100;
        }
    }
    sum
}

fn move_robot(robot: &mut (i64, i64), objects: &mut HashMap<(i64, i64), char>, instruction: char) {
    let direction;
    match instruction {
        '^' => direction = (0, -1),
        '>' => direction = (1, 0),
        'v' => direction = (0, 1),
        '<' => direction = (-1, 0),
        _ => panic!("Invalid instruction"),
    }
    let mut we_good = true;
    let mut new_pos = (robot.0 + direction.0, robot.1 + direction.1);
    let mut potential_moves = Vec::new();
    loop {
        if !objects.contains_key(&new_pos) {
            // clear move
            break;
        } else if objects[&new_pos] == 'O' {
            // potentially ok move
            potential_moves.push(new_pos);
            new_pos = (new_pos.0 + direction.0, new_pos.1 + direction.1);
        } else if objects[&new_pos] == '#' {
            // blocked move
            we_good = false;
            break;
        } else {
            println!("Invalid object {}", objects[&new_pos]);
            panic!("Invalid object");
        }
    }
    if we_good {
        //move robot
        robot.0 = robot.0 + direction.0;
        robot.1 = robot.1 + direction.1;
        // reverse to avoid overwriting things
        potential_moves.reverse();
        for pos in potential_moves {
            objects.remove(&pos);
            objects.insert((pos.0 + direction.0, pos.1 + direction.1), 'O');
        }
    }
}

fn parse_input(input: &str) -> (HashMap<(i64, i64), char>, (i64, i64), String, i64, i64) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut y = 0;
    let mut objects = HashMap::new();
    let mut robot = (0, 0);
    let mut x = 0;
    for line in map.lines() {
        x = 0;
        for pos in line.chars() {
            if pos == 'O' || pos == '#' {
                objects.insert((x, y), pos);
            } else if pos == '@' {
                robot = (x, y);
            }
            x += 1;
        }
        y += 1;
    }
    let clean_instructions = &instructions.replace("\n", "");
    (objects, robot, clean_instructions.to_string(), x, y)
}

fn print_objects(objects: &mut HashMap<(i64, i64), char>, robot: (i64, i64), xmax: i64, ymax: i64) {
    objects.insert(robot, '@');
    for y in 0..=ymax {
        for x in 0..=xmax {
            if objects.contains_key(&(x, y)) {
                print!("{}", objects[&(x, y)]);
            } else {
                print!(".");
            }
        }
        println!();
    }
    objects.remove(&robot);
}

fn part_2(input: &str) -> i64 {
    let (mut objects, mut robot, instructions, xmax, ymax) = parse_input2(input);

    for instruction in instructions.chars() {
        objects = move_robot2(&mut robot, &mut objects, instruction);
    }
    let mut sum = 0;
    print_objects(&mut objects, robot, xmax, ymax);
    println!("xmax {} ymax {}", xmax, ymax);
    for y in 1..ymax {
        for x in 1..xmax {
            if objects.contains_key(&(x, y)) && objects[&(x, y)] == '[' {
                sum += x + y * 100;
            }
        }
    }
    sum
}

fn parse_input2(input: &str) -> (HashMap<(i64, i64), char>, (i64, i64), String, i64, i64) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut y = 0;
    let mut objects = HashMap::new();
    let mut robot = (0, 0);
    let mut xmax = 0;
    for line in map.lines() {
        let mut x = 0;
        for pos in line.chars() {
            if pos == '#' {
                objects.insert((x * 2, y), pos);
                objects.insert((x * 2 + 1, y), pos);
            } else if pos == 'O' {
                objects.insert((x * 2, y), '[');
                objects.insert((x * 2 + 1, y), ']');
            } else if pos == '@' {
                robot = (x * 2, y);
            }
            if x * 2 + 1 > xmax {
                xmax = x * 2 + 1;
            }
            x += 1;
        }

        y += 1;
    }
    let clean_instructions = &instructions.replace("\n", "");
    (objects, robot, clean_instructions.to_string(), xmax, y)
}

fn move_robot2(
    robot: &mut (i64, i64),
    objects: &mut HashMap<(i64, i64), char>,
    instruction: char,
) -> HashMap<(i64, i64), char> {
    let direction;
    match instruction {
        '^' => direction = (0, -1),
        '>' => direction = (1, 0),
        'v' => direction = (0, 1),
        '<' => direction = (-1, 0),
        _ => panic!("Invalid instruction"),
    }
    let new_pos = (robot.0 + direction.0, robot.1 + direction.1);
    let (potential_moves, we_good) = test_move(new_pos, objects, direction);
    // to avoid out of order operations
    let mut new_objects = objects.clone();
    if we_good {
        //move robot
        robot.0 = robot.0 + direction.0;
        robot.1 = robot.1 + direction.1;
        // remove first to avoid removing newly moved
        for pos in potential_moves.clone() {
            new_objects.remove(&pos);
        }
        for pos in potential_moves {
            let box_part = objects[&pos];
            new_objects.insert((pos.0 + direction.0, pos.1 + direction.1), box_part);
        }
    }
    new_objects
}

fn test_move(
    new_pos: (i64, i64),
    objects: &mut HashMap<(i64, i64), char>,
    direction: (i64, i64),
) -> (Vec<(i64, i64)>, bool) {
    let mut potential_moves = Vec::new();
    let mut we_ok = true;
    let mut next_pos = new_pos;
    if direction == (1, 0) || direction == (-1, 0) {
        // horizontal move, do as before
        loop {
            if !objects.contains_key(&next_pos) {
                // clear move
                break;
            } else if objects[&next_pos] == '[' || objects[&next_pos] == ']' {
                // potentially ok move
                potential_moves.push(next_pos);
                next_pos = (next_pos.0 + direction.0, next_pos.1 + direction.1);
            } else if objects[&next_pos] == '#' {
                // not good
                we_ok = false;
                break;
            } else {
                println!("Invalid object {}", objects[&next_pos]);
                panic!("Invalid object");
            }
        }
    } else {
        //vertical move, can branch
        if objects.contains_key(&new_pos) {
            if objects[&new_pos] == '#' {
                we_ok = false;
            } else if objects[&new_pos] == '[' || objects[&new_pos] == ']' {
                let neighbor;
                if objects[&new_pos] == '[' {
                    neighbor = 1;
                } else {
                    neighbor = -1;
                }
                let next_pos1 = (new_pos.0 + direction.0, new_pos.1 + direction.1);
                let next_pos2 = (new_pos.0 + direction.0 + neighbor, new_pos.1 + direction.1);
                let (mut potential_moves1, we_ok1) = test_move(next_pos1, objects, direction);
                let (mut potential_moves2, we_ok2) = test_move(next_pos2, objects, direction);
                if we_ok1 && we_ok2 {
                    potential_moves.push(new_pos);
                    potential_moves.push((new_pos.0 + neighbor, new_pos.1));
                    potential_moves.append(&mut potential_moves1);
                    potential_moves.append(&mut potential_moves2);
                } else {
                    we_ok = false;
                }
            } else {
                println!("Invalid object {}", objects[&new_pos]);
                panic!("Invalid test path object");
            }
        }
    }
    (potential_moves, we_ok)
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
        assert_eq!(part_1(&read_file("example.txt")), 10092);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1471826);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example.txt")), 9021);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1457703);
    }
}
