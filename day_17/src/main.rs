extern crate core;

use std::fs;
use std::time::Instant;

#[derive(Clone)]
struct Device {
    instruction_pointer: usize,
    a: u64,
    b: u64,
    c: u64,
    values_to_output: Vec<u64>,
}

impl Device {
    fn new(input: (u64, u64, u64)) -> Device {
        Device {
            instruction_pointer: 0,
            a: input.0,
            b: input.1,
            c: input.2,
            values_to_output: Vec::<u64>::new(),
        }
    }

    fn get_combo_operand(&self, op_code: u64) -> u64 {
        match op_code {
            0..4 => op_code,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Op code node supported for combo operand: {}", op_code),
        }
    }

    fn perform_instruction(&mut self, op_code: u64, operand: u64) {
        let mut jump_performed = false;

        match op_code {
            0 => self.a /= 2_u64.pow(self.get_combo_operand(operand) as u32),
            1 => self.b ^= operand,
            2 => self.b = self.get_combo_operand(operand) % 8,
            3 => {
                if self.a != 0 {
                    self.instruction_pointer = operand as usize;
                    jump_performed = true;
                }
            }
            4 => self.b ^= self.c,
            5 => self
                .values_to_output
                .push(self.get_combo_operand(operand) % 8),
            6 => self.b = self.a / 2_u64.pow(self.get_combo_operand(operand) as u32),
            7 => self.c = self.a / 2_u64.pow(self.get_combo_operand(operand) as u32),
            _ => panic!("Op code instruction not supported: {}", op_code),
        }

        if !jump_performed {
            self.instruction_pointer += 2;
        }
    }

    fn output_values(&self) -> String {
        self.values_to_output
            .iter()
            .map(u64::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let start = Instant::now();
    println!("Part One: {} - {:?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part Two: {} - {:?}", part_two(&input), start.elapsed());
}

fn part_one(input: &String) -> String {
    let (mut device, program) = parse_input(input);

    run_program_on_device(&mut device, &program)
}

fn part_two(input: &String) -> u64 {
    let (_, program) = parse_input(input);
    
    find_smallest_reflective_registry(&program)
}

fn find_smallest_reflective_registry(program: &[u64]) -> u64 {
    let mut solutions = Vec::new();
    find_possible_solutions(&mut solutions, &program, 1);
    
    *solutions.iter().min().unwrap()
}

fn find_possible_solutions(solutions: &mut Vec<u64>, program: &[u64], running_possible: u64){
    for possible in running_possible..running_possible + 7 {
        let mut device = Device::new((possible,0,0));
        run_program_on_device(&mut device, program);
        
        if device.values_to_output == program {
            solutions.push(possible);
        } else if output_fits_end(&mut device, program){
            find_possible_solutions(solutions, program, possible << 3);
        }
    }
}

fn output_fits_end(device: &mut Device, program: &[u64]) -> bool {
    program.iter().rev().zip(device.values_to_output.iter().rev())
        .map(|(a, b)| a == b)
        .all(|a| a)
}


fn run_program_on_device(device: &mut Device, program: &[u64]) -> String {
    while device.instruction_pointer < program.len() {
        device.perform_instruction(
            program[device.instruction_pointer],
            program[device.instruction_pointer + 1],
        );
    }

    device.output_values()
}

fn parse_input(input: &String) -> (Device, Vec<u64>) {
    let parts: Vec<&str> = input
        .split(['\r', '\n', ':'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect();

    (
        Device::new((
            parts[1].parse().unwrap(),
            parts[3].parse().unwrap(),
            parts[5].parse().unwrap(),
        )),
        parts[7]
            .split(',')
            .map(|program| program.parse::<u64>().unwrap())
            .collect(),
    )
}

#[test]
fn small_input() {
    let mut device = Device::new((729, 0, 0));
    let program = [0, 1, 5, 4, 3, 0];

    run_program_on_device(&mut device, &program);
    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        run_program_on_device(&mut device, &program)
    );
}

#[test]
fn part_two_test() {
    let program = [0,3,5,4,3,0];

    assert_eq!(
        117440,
        find_smallest_reflective_registry(&program)
    );
}
