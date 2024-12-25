//#![allow(unused_mut)]
//use itertools::Itertools;
use std::collections::HashMap;
//use std::collections::HashSet;
use std::fs;
use std::u64;

use itertools::Itertools;

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

fn part_1(input: &str) -> u64 {
    let (registers, gates) = parse_input(input);
    simulate_machine(registers, gates)
}

fn simulate_machine(registers: HashMap<&str, u8>, gates: HashMap<&str, (&str, &str, &str)>) -> u64 {
    let mut all_z_ok = false;
    let mut registers = registers.clone();
    let mut counter = 0;
    while !all_z_ok {
        counter += 1;
        all_z_ok = true;
        for (target, gate) in &gates {
            match gate.1 {
                "AND" => {
                    if registers.contains_key(gate.0) && registers.contains_key(gate.2) {
                        let val1 = registers.get(gate.0).unwrap();
                        let val2 = registers.get(gate.2).unwrap();
                        registers.insert(target, val1 & val2);
                    }
                }
                "OR" => {
                    if registers.contains_key(gate.0) && registers.contains_key(gate.2) {
                        let val1 = registers.get(gate.0).unwrap();
                        let val2 = registers.get(gate.2).unwrap();
                        registers.insert(target, val1 | val2);
                    }
                }
                "XOR" => {
                    if registers.contains_key(gate.0) && registers.contains_key(gate.2) {
                        let val1 = registers.get(gate.0).unwrap();
                        let val2 = registers.get(gate.2).unwrap();
                        registers.insert(target, val1 ^ val2);
                    }
                }
                _ => {
                    println!("Unknown gate: {}", gate.1);
                }
            }
            if target.starts_with("z") && !registers.contains_key(target) {
                all_z_ok = false;
            }
        }
        if counter == 60 {
            return 0;
        }
    }
    //println!("Counter: {}", counter);
    reg_to_u64(&registers, 'z')
}

fn parse_input(input: &str) -> (HashMap<&str, u8>, HashMap<&str, (&str, &str, &str)>) {
    let (reg_raw, gates_raw) = input.split_once("\n\n").unwrap();
    let mut registers = HashMap::new();
    for line in reg_raw.lines() {
        let (register, value) = line.split_once(": ").unwrap();
        registers.insert(register, value.parse::<u8>().unwrap());
    }
    let mut gates = HashMap::new();
    for line in gates_raw.lines() {
        let (instruction, target) = line.split_once(" -> ").unwrap();
        let instruction = instruction.split_whitespace().collect_vec();
        gates.insert(target, (instruction[0], instruction[1], instruction[2]));
    }
    (registers, gates)
}

fn part_2(input: &str) -> String {
    // printing bad bits, printing upstream gates from bad bits
    // then solve on paper using addition gate setup
    let (registers, gates) = parse_input(input);
    let z = simulate_machine(registers.clone(), gates.clone());
    let x = reg_to_u64(&registers, 'x');
    let y = reg_to_u64(&registers, 'y');

    let z_correct = x + y;
    println!("Z_c: {:b}", z_correct);
    let faulty_bits = z_correct ^ z;
    println!("Faulty bits: {:b}", faulty_bits);
    for bit in [
        "z00", "z10", "z11", "z12", "z13", "z14", "z15", "z17", "z32", "z33", "z34",
    ] {
        println!("");
        println!("NEW REG {:?}", bit);
        print_bit_adder(&gates, bit);
    }
    "".to_string()
}

fn print_bit_adder(gates: &HashMap<&str, (&str, &str, &str)>, register: &str) {
    let gate = gates.get(register).unwrap();
    println!("{}: {} {} {}", register, gate.0, gate.1, gate.2);
    if gates.contains_key(gate.0) {
        print_bit_adder(gates, gate.0);
    }
    if gates.contains_key(gate.2) {
        print_bit_adder(gates, gate.2);
    }
}

fn reg_to_u64(registers: &HashMap<&str, u8>, key: char) -> u64 {
    let mut result = 0;
    for (reg, bit) in registers {
        //println!("reg {}: {}", reg, bit);
        if reg.starts_with(key) {
            let pos = &reg[1..].parse::<u64>().unwrap();
            let value = (*bit as u64) << pos;
            //println!("Value: {}", value);
            //println!("Pos: {}", pos);
            result += value;
        }
    }
    result
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

fn count_register_appearances(gates: &Vec<(&str, &str, &str, &str)>) {
    let mut register_appearances = HashMap::new();
    let mut out_regs = HashMap::new();
    for gate in gates {
        if !register_appearances.contains_key(gate.0) {
            register_appearances.insert(gate.0, 1);
        } else {
            let count = register_appearances.get(gate.0).unwrap() + 1;
            register_appearances.insert(gate.0, count);
        }
        if !register_appearances.contains_key(gate.2) {
            register_appearances.insert(gate.2, 1);
        } else {
            let count = register_appearances.get(gate.2).unwrap() + 1;
            register_appearances.insert(gate.2, count);
        }
        if !out_regs.contains_key(gate.3) {
            out_regs.insert(gate.3, 1);
        } else {
            let count = out_regs.get(gate.3).unwrap() + 1;
            out_regs.insert(gate.3, count);
        }
    }
    let mut max = 0;
    let mut counter = 0;
    for (_, count) in register_appearances {
        counter += 1;
        if count > max {
            max = count;
        }
    }
    println!("Max reg ap: {}", max);
    max = 0;
    for (_, count) in out_regs {
        if count > max {
            max = count;
        }
    }
    println!("Max out reg ap: {}", max);
    println!("Counter: {}", counter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("example.txt")), 2024);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 54715147844840);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("example2.txt")), "z00,z01,z02,z05");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), "");
    }
}
