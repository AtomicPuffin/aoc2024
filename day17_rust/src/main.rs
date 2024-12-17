//use itertools::Itertools;
//use std::cmp::Reverse;
//use std::collections::BinaryHeap;
//use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("example.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    /*println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("example2.txt"))
    );*/
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> String {
    let (register_a, program) = parse_input(input);
    run_machine(register_a, program)
}

fn run_machine(mut register_a: u64, program: Vec<u8>) -> String {
    let mut position = 0_u64;
    let mut out = Vec::new();
    let mut register_b = 0_u64;
    let mut register_c = 0_u64;
    while position < program.len() as u64 {
        let inst = program[position as usize];
        let mut operand = program[(position + 1) as usize] as u64;
        let mut jumped = false;
        match operand {
            0 | 1 | 2 | 3 => {}
            4 => {
                operand = register_a;
            }
            5 => {
                operand = register_b;
            }
            6 => {
                operand = register_c;
            }
            7 => {
                // never happens
                println!("Reserved opcode: {}", operand);
            }

            _ => {
                println!("Unknown opcode: {}", operand);
                break;
            }
        }
        match inst {
            0 => {
                register_a = register_a >> operand;
            }
            1 => {
                register_b = register_b ^ program[(position + 1) as usize] as u64;
            }
            2 => {
                register_b = operand % 8;
            }
            3 => {
                if register_a != 0 {
                    position = program[(position + 1) as usize] as u64;
                    jumped = true;
                }
            }
            4 => {
                register_b = register_b ^ register_c;
            }
            5 => {
                out.push(operand % 8);
            }
            6 => {
                register_b = register_a >> operand;
            }
            7 => {
                register_c = register_a >> operand;
            }
            _ => {
                println!("Unknown opcode: {}", inst);
                break;
            }
        }
        if !jumped {
            position += 2;
        }
    }
    out.iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse_input(input: &str) -> (u64, Vec<u8>) {
    let (registers, instructions) = input.split_once("\n\n").unwrap();
    let registers = registers.lines().collect::<Vec<&str>>();
    let register_a = registers[0]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let program = instructions
        .split(' ')
        .last()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    (register_a, program)
}

fn part_2(input: &str) -> u64 {
    let (_, program) = parse_input(input);
    let length = program.len();
    println!("Length: {}", length);
    let mut answers = HashSet::new();
    answers.insert(0);
    for i in (0..length).rev() {
        let mut new_answers = HashSet::new();
        for answer in answers {
            for j in 0..8 {
                let new_answer = (answer << 3) + j;
                if run_machine_math(new_answer) == program[i] as u64 {
                    new_answers.insert(new_answer);
                }
            }
        }
        answers = new_answers;
    }
    *answers.iter().min().unwrap()
}

fn run_machine_math(a: u64) -> u64 {
    let first_b = (a % 8) ^ 2;
    ((first_b ^ (a >> first_b)) ^ 7) % 8
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
        assert_eq!(part_1(&read_file("example2.txt")), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), "2,3,4,7,5,7,3,0,7");
    }

    /*#[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), 0);
    }*/

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 190384609508367);
    }
}
